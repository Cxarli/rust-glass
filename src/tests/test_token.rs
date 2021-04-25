use token::Token;
use token::Token::*;


#[test]
fn string_add_char() {
    assert_eq!(Token::String("abc".to_string()) + 'd', Token::String("abcd".to_string()));
    assert_eq!(Token::String("".to_string()) + 'Z', Token::String("Z".to_string()));
    assert_eq!(Token::String("C".to_string()) + 'h' + 'a' + 'r' + 'l' + 'i' + 'e', Token::String("Charlie".to_string()));
}


#[test]
fn number_add_char() {
    assert_eq!(Number(123) + '4', Number(1234));
    assert_eq!(Number(0) + '9', Number(9));
    assert_eq!(Number(100) + '0', Number(1000));
    assert_eq!(Number(1) + '2' + '3' + '4' + '5', Number(12345));
}


#[test]
fn name_add_char() {
    assert_eq!(Name("abc".to_string()) + 'd', Name("abcd".to_string()));
    assert_eq!(Name("G".to_string()) + 'l' + 'a' + 's' + 's', Name("Glass".to_string()));
}
