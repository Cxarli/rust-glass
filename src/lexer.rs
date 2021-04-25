use token::*;
use token::Token::*;
use std::string::String;


pub fn lex(program: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec! { };

    let mut group_token: Option<Token> = None;

    let mut in_comment = false;
    let mut comment_value = String::new();


    for ch in program.chars() {

        if in_comment {
            // end of comment
            if ch == '\'' {
                tokens.push(Token::Comment(comment_value.clone()));

                in_comment = false;
                comment_value.clear();
            }
            else {
                comment_value += &ch.to_string();
            }

            continue;
        }
        else if ch == '\'' {
            in_comment = true;
            continue;
        }

        // If in a capturing group
        if let Some(token) = group_token {
            match (&token, ch) {

                // End of group
                (&Token::String(_), '"')
                | (&Token::Number(_), '>')
                | (&Token::Name(_), ')')
                => {
                    // Push current value
                    tokens.push(token.clone());

                    // Reset value
                    group_token = None;
                },

                // When not closed, add to current value
                _ => group_token = Some(token + ch),
            }


            // It doesn't have to get parsed when it's in a group
            continue;
        }


        if let Some(token) = match ch {
            'a' ... 'z' | 'A' ... 'Z' => Some(Name(ch.to_string())),

            '0' ... '9' => Some(Number(ch.to_digit(10).unwrap() as i32)),

            '{' => Some(StartClass),
            '}' => Some(EndClass),
            '[' => Some(StartFunction),
            ']' => Some(EndFunction),
            '/' => Some(StartWhile),
            '\\' => Some(EndWhile),

            ',' => Some(PopValue),
            '^' => Some(Return),
            '=' => Some(Assign),
            '!' => Some(NewInstance),
            '.' => Some(RetrieveFunction),
            '?' => Some(PopAndRunFunction),
            '*' => Some(RetrieveFromName),
            '$' => Some(AssignCurrent),
            '@' => Some(SwapTwo),

            _ => None,
        } {
            tokens.push(token);
        }


        // Start capturing groups such as "string", <10>, (name)
        else if let Some(x_token) = match ch {
            '"' => Some(Token::String(String::new())),
            '<' => Some(Number(0)),
            '(' => Some(Name(String::new())),

            _ => None,
        } {
            group_token = Some(x_token);
        }


        else {
            match ch {
                // Ignore whitespace
                ' ' | '\t' | '\n' => (),

                _ => panic!("Invalid character `{}` found", ch),
            }

        }
    }

    assert!( ! group_token.is_some(), "Program ended inside of a capturing group!" );

    tokens
}
