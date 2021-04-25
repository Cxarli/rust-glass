pub mod js_ast;
mod test;

use self::js_ast::*;
use self::js_ast::JSASTLiteral::*;
use self::js_ast::JSASTField::*;
use self::js_ast::JSASTGroupType::*;
use self::js_ast::JSASTItemType::*;

use ast::*;
use ast::ASTField::*;
use ast::ASTItemType::*;
use ast::ASTGroupType::*;


pub fn transform(ast: AST) -> JSAST {
    let mut jsast: JSAST = vec! { };

    for item in ast {
        let astfield: ASTField = *item;

        if let ASTGroup(Class(), class_name, body) = astfield {
            let mut constructor_function: JSAST = vec! { };
            let mut functions: JSAST = vec! { };

            for item in body {
                match *item {
                    ASTGroup(Function(), ref function_name, ref body) => {

                        // Constructor
                        if function_name == "__c" {
                            constructor_function = transform(body.clone());
                        }

                        // TODO: Destructor?

                        else {
                            functions.push(Box::new(
                                JSASTGroup(
                                    JSFunction(
                                        JSName(format!("{}.prototype.{}",
                                            class_name,
                                            function_name,
                                        ))
                                    ),

                                    transform(body.clone()),
                                )
                            ));
                        }
                    },

                    _ => panic!("Classes should only contain functions!"),
                }
            }


            let class_group = JSASTGroup(
                JSFunction(JSName(class_name)),
                constructor_function,
            );

            jsast.push(Box::new(class_group));

            for function in functions {
                jsast.push(function);
            }
        }

        else {
            let jsastitem = match astfield {
                ASTItem(Action(x)) => JSASTItem(JSAction(x)),
                ASTItem(Literal(x)) => JSASTItem(JSLiteral(JSASTLiteral::from(x))),

                ASTGroup(Function(), name, body) => JSASTGroup(JSFunction(JSName(name)), transform(body)),
                ASTGroup(While(), name, body) => JSASTGroup(JSWhile(JSName(name)), transform(body)),

                _ => panic!("This shouldn't happen: ASTGroup(Class, _, _) fallthrough"),
            };

            jsast.push(Box::new(jsastitem));
        }
    }


    jsast
}
