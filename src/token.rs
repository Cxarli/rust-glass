use std::ops::*;


#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    String(String),
    Number(i32),
    Name(String),
    Comment(String),

    StartClass, // {
    EndClass, // }

    StartFunction, // [
    EndFunction, // ]

    StartWhile, // /
    EndWhile, // \

    PopValue,  // ,
    Return, // ^
    Assign, // =
    NewInstance, // !
    RetrieveFunction, // .
    PopAndRunFunction, // ?
    RetrieveFromName, // *
    AssignCurrent, // $
    SwapTwo, // @
}


impl Add<char> for Token {
    type Output = Token;

    fn add(self, rhs: char) -> Token {
        match self {
            Token::String(x) => Token::String(x + &rhs.to_string()),

            Token::Name(x) => Token::Name(x + &rhs.to_string()),

            Token::Number(x) => {
                if let Some(rhs_number) = rhs.to_digit(10) {
                    Token::Number(x*10 + rhs_number as i32)
                } else {
                    panic!("Failed to parse number inside number group!");
                }
            },


            _ => panic!("You can not add chars to this type"),
        }
    }
}
