use lexer::*;

use token::Token;
use token::Token::*;



#[test]
fn numbers() {
    assert_eq!(lex("7"), vec! {
        Number(7),
    });

    assert_eq!(lex("<10>"), vec! {
        Number(10),
    });

    assert_eq!(lex("<123>"), vec! {
        Number(123),
    });

    assert_eq!(lex("<45><67><89>"), vec! {
        Number(45),
        Number(67),
        Number(89),
    });

    // No hexadecimal
    assert_eq!(lex("a"), vec! {
        Name("a".to_string()),
    });

    // Numbers in strings are still strings
    assert_eq!(lex("\"0\""), vec! {
        Token::String("0".to_string()),
    });
}


#[test]
fn strings() {
    assert_eq!(lex("\"abc\""), vec! {
        Token::String("abc".to_string()),
    });

    assert_eq!(lex("\"\""), vec! {
        Token::String("".to_string()),
    });

    assert_eq!(lex("\"de\"\"fg\"\"hi\""), vec! {
        Token::String("de".to_string()),
        Token::String("fg".to_string()),
        Token::String("hi".to_string()),
    });
}


#[test]
fn names() {
    assert_eq!(lex("h"), vec! {
        Name("h".to_string()),
    });

    assert_eq!(lex("(_p)"), vec! {
        Name("_p".to_string()),
    });

    assert_eq!(lex("(abcde)"), vec! {
        Name("abcde".to_string()),
    });

    assert_eq!(lex("(ab)c(d)ef"), vec! {
        Name("ab".to_string()),
        Name("c".to_string()),
        Name("d".to_string()),
        Name("e".to_string()),
        Name("f".to_string()),
    });
}


#[test]
fn class_and_function() {
    assert_eq!(lex("{M[m<10>]}"), vec! {
        StartClass,
        Name("M".to_string()),
        StartFunction,
        Name("m".to_string()),
        Number(10),
        EndFunction,
        EndClass,
    });
}


#[test]
fn class_and_function_with_whitespace() {
    assert_eq!(lex("{ M [ m <10> ] }"), vec! {
        StartClass,
        Name("M".to_string()),

        StartFunction,
        Name("m".to_string()),

        Number(10),

        EndFunction,
        EndClass,
    });
}


#[test]
fn hello_world() {
    let program = "{M[m(_o)O!\"Hello, World!\\n\"(_o)o.?]}";

    let expected = vec! {
        StartClass,
        Name("M".to_string()),

        StartFunction,
        Name("m".to_string()),

        Name("_o".to_string()),
        Name("O".to_string()),
        NewInstance,

        Token::String("Hello, World!\\n".to_string()),

        Name("_o".to_string()),
        Name("o".to_string()),
        RetrieveFunction,

        PopAndRunFunction,

        EndFunction,
        EndClass,
    };

    assert_eq!(lex(program), expected);
}


#[test]
fn while_loop() {
    assert_eq!(lex("/(abc) <10>\\"), vec! {
        StartWhile,
        Name("abc".to_string()),
        Number(10),
        EndWhile,
    });
}
