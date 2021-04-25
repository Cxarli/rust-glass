use token::*;
use ast::*;

use ast::ASTField::*;
use ast::ASTItemType::*;
use ast::ASTAction::*;


// TODO: Fix complexity
#[cfg_attr(feature="clippy", allow(cyclomatic_complexity))]
pub fn parse(tokens: &[Token]) -> AST {
    let mut ast: AST = vec! { };
    let mut stack: Vec<ASTLiteral> = vec! { };

    macro_rules! move_stack_to_ast {
        () => {
            for item in &stack {
                ast.push(Box::new(ASTItem(Action(PushLiteralToStack(ASTLiteral::from(item.clone()))))));
            }

            stack.clear();
        }
    }


    let mut tokens_iter = tokens.iter();

    while let Some(token) = tokens_iter.next() {

        macro_rules! collect_group {
            ($group_type: expr, $start_token: pat, $end_token: pat) => {
                // Keep count of the amount of brackets
                let (mut open, mut close) = (1, 0);

                // Get the name of the group
                let token = tokens_iter.next().expect("Failed to get group name");

                let group_name = match token.clone() {
                    Token::Name(name) => name,
                    _ => panic!("Group name is not a Name"),
                };

                // A vector to store the inner tokens in
                let mut new_tokens: Vec<Token> = vec! { };

                // As long as there are more open brackets than closing brackets
                while open > close {
                    if let Some(new_token) = tokens_iter.next() {
                        // Find matching brackets
                        match *new_token {
                            $start_token => open += 1,
                            $end_token => close += 1,
                            _ => (),
                        }

                        // Don't include closing bracket to new tokens
                        if open != close {
                            new_tokens.push(new_token.clone());
                        }
                    }
                    else {
                        panic!("Not enough closing brackets!");
                    }
                }

                // Generate new AST for inner group
                let new_ast = parse(&new_tokens);

                // Push the inner AST to the main AST
                ast.push(Box::new(ASTField::ASTGroup($group_type, group_name, new_ast)));
            }
        }

        // Collect class
        if let Token::StartClass = *token {
            collect_group!(ASTGroupType::Class(), Token::StartClass, Token::EndClass);
            continue;
        }

        // Collect function
        if let Token::StartFunction = *token {
            collect_group!(ASTGroupType::Function(), Token::StartFunction, Token::EndFunction);
            continue;
        }

        // Collect while
        if let Token::StartWhile = *token {
            collect_group!(ASTGroupType::While(), Token::StartWhile, Token::EndWhile);
            continue;
        }


        if let (Some(astaction), might_change_stack) = match *token {
            Token::StartClass | Token::EndClass
            | Token::StartFunction | Token::EndFunction
            | Token::StartWhile | Token::EndWhile
            => {
                (None, true)
            },

            Token::Comment(ref val) => {
                // A comment may be pushed to the AST directly,
                // because no further parsing is needed
                ast.push(Box::new(ASTItem(Literal(ASTLiteral::Comment(val.clone())))));
                (None, false)
            },

            Token::Name(_) | Token::String(_) | Token::Number(_) => {
                stack.push(ASTLiteral::from(token.clone()));

                (None, false)
            },

            Token::PopValue => {
                if stack.len() >= 1 {
                    stack.pop();

                    (None, false)
                }
                else {
                    (Some(ASTAction::PopValueFromStack()), true)
                }
            },

            Token::SwapTwo => {
                if stack.len() >= 2 {
                    let y = stack.pop().unwrap();
                    let x = stack.pop().unwrap();

                    stack.push(y);
                    stack.push(x);

                    (None, true)
                }
                else {
                    (Some(SwapTwoFromStack()), true)
                }
            },

            Token::Return => {
                (Some(Return()), true)
            },

            Token::Assign => {
                if stack.len() >= 2 {
                    let value = stack.pop().unwrap();
                    let name = stack.pop().unwrap();

                    (Some(Assign(name, value)), false)
                }
                else {
                    (Some(AssignFromStack()), true)
                }
            },

            Token::NewInstance => {
                if stack.len() >= 2 {
                    let cname = stack.pop().unwrap();
                    let name = stack.pop().unwrap();

                    (Some(NewInstance(name, cname)), false)
                }
                else {
                    (Some(NewInstanceFromStack()), true)
                }
            },

            Token::RetrieveFunction => {
                if stack.len() >= 2 {
                    let fname = stack.pop().unwrap();
                    let oname = stack.pop().unwrap();

                    (Some(RetrieveFunction(oname, fname)), true)
                }
                else {
                    (Some(RetrieveFunctionFromStack()), true)
                }
            },

            Token::PopAndRunFunction => {
                (Some(PopAndRunFunction()), true)
            },

            Token::RetrieveFromName => {
                if stack.len() >= 1 {
                    let name = stack.pop().unwrap();

                    (Some(RetrieveFromName(name)), true)
                }
                else {
                    (Some(RetrieveFromNameFromStack()), true)
                }
            },

            Token::AssignCurrent => {
                if stack.len() >= 1 {
                    let name = stack.pop().unwrap();

                    (Some(AssignCurrent(name)), false)
                }
                else {
                    (Some(AssignCurrentFromStack()), true)
                }
            },

        } {
            if might_change_stack {
                move_stack_to_ast!();
            }

            ast.push(Box::new(ASTItem(Action(astaction))));
        }
    }

    if ! stack.is_empty() {
        eprintln!("Stack not empty, pushing to AST! (This might be an error)");

        move_stack_to_ast!();
    }


    ast
}
