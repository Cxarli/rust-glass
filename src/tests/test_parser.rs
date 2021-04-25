use parser::parse;
use lexer::lex;

use ast::ASTField::*;
use ast::ASTItemType::*;
use ast::ASTGroupType::*;
use ast::ASTAction::*;

use ast::ASTLiteral;
use ast::ASTLiteral::*;


#[test]
fn numbers() {
    assert_eq!(parse(&lex("8")), vec! {
        Box::new(ASTItem(Action(PushLiteralToStack(Number(8))))),
    });

    assert_eq!(parse(&lex("<321>")), vec! {
        Box::new(ASTItem(Action(PushLiteralToStack(Number(321))))),
    });

    assert_eq!(parse(&lex("<1>2<34>56")), vec! {
        Box::new(ASTItem(Action(PushLiteralToStack(Number(1))))),
        Box::new(ASTItem(Action(PushLiteralToStack(Number(2))))),
        Box::new(ASTItem(Action(PushLiteralToStack(Number(34))))),
        Box::new(ASTItem(Action(PushLiteralToStack(Number(5))))),
        Box::new(ASTItem(Action(PushLiteralToStack(Number(6))))),
    });
}


#[test]
fn class() {
    assert_eq!(parse(&lex("{X}")), vec! {
        Box::new(ASTGroup(Class(), "X".to_string(), vec! { })),
    });

    assert_eq!(parse(&lex("{(Xyz)}")), vec! {
        Box::new(ASTGroup(Class(), "Xyz".to_string(), vec! { })),
    });

    assert_eq!(parse(&lex("{A}{B}{CD}{(EF)}")), vec! {
        Box::new(ASTGroup(Class(), "A".to_string(), vec! { })),
        Box::new(ASTGroup(Class(), "B".to_string(), vec! { })),

        Box::new(ASTGroup(Class(), "C".to_string(), vec! {
            Box::new(ASTItem(Action(PushLiteralToStack(Name("D".to_string()))))),
        })),

        Box::new(ASTGroup(Class(), "EF".to_string(), vec! { })),
    });
}


#[test]
fn class_and_function() {
    assert_eq!(parse(&lex("{X[x<10>]}")), vec! {
        Box::new(ASTGroup(Class(), "X".to_string(), vec! {
            Box::new(ASTGroup(Function(), "x".to_string(), vec! {
                Box::new(ASTItem(Action(PushLiteralToStack(Number(10))))),
            })),
        })),
    });
}


#[test]
fn class_with_multiple_functions() {
    assert_eq!(parse(&lex("{X[x<10>][y\"abc\"]}")), vec! {
        Box::new(ASTGroup(Class(), "X".to_string(), vec! {
            Box::new(ASTGroup(Function(), "x".to_string(), vec! {
                Box::new(ASTItem(Action(PushLiteralToStack(Number(10))))),
            })),

            Box::new(ASTGroup(Function(), "y".to_string(), vec! {
                Box::new(ASTItem(Action(PushLiteralToStack(ASTLiteral::String("abc".to_string()))))),
            })),
        })),
    });
}


#[test]
fn hello_world() {
    assert_eq!(parse(&lex("{M[m(_o)O!\"Hello, World!\\n\"(_o)o.?]}")), vec! {
    	Box::new(ASTGroup(Class(), "M".to_string(), vec! {
    		Box::new(ASTGroup(Function(), "m".to_string(), vec! {
    			Box::new(ASTItem(Action(NewInstance(Name("_o".to_string()), Name("O".to_string()))))),
    			Box::new(ASTItem(Action(PushLiteralToStack(ASTLiteral::String("Hello, World!\\n".to_string()))))),
    			Box::new(ASTItem(Action(RetrieveFunction(Name("_o".to_string()), Name("o".to_string()))))),
    			Box::new(ASTItem(Action(PopAndRunFunction()))),
    		})),
    	})),
    });
}


#[test]
fn while_loop() {
    assert_eq!(parse(&lex("/(abc) <10>\\")), vec! {
        Box::new(ASTGroup(While(), "abc".to_string(), vec! {
            Box::new(ASTItem(Action(PushLiteralToStack(Number(10))))),
        }))
    });
}
