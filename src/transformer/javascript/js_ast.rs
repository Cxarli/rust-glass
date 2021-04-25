use ast::*;

use self::JSASTLiteral::*;


pub type JSAST = Vec<Box<JSASTField>>;


#[derive(Debug, Clone, PartialEq)]
pub enum JSASTField {
    JSASTItem(JSASTItemType),
    JSASTGroup(JSASTGroupType, JSAST),
}


#[derive(Debug, Clone, PartialEq)]
pub enum JSASTItemType {
    JSLiteral(JSASTLiteral),
    JSAction(ASTAction),
}


#[derive(Debug, Clone, PartialEq)]
pub enum JSASTGroupType {
    JSFunction(JSASTLiteral),
    JSWhile(JSASTLiteral),
    JSBlock(),
}


#[derive(Debug, Clone, PartialEq)]
pub enum JSASTLiteral {
    JSString(String),
    // JavaScript actually has higher precision, but this'll do
    JSNumber(f64),
    JSName(String),
    JSComment(String),
    JSUndefined(),
}



impl From<ASTLiteral> for JSASTLiteral {
    fn from(astliteral: ASTLiteral) -> JSASTLiteral {
        match astliteral {
            ASTLiteral::String(x) => JSString(x),
            ASTLiteral::Number(x) => JSNumber(f64::from(x)),
            ASTLiteral::Name(x) => JSName(x),
            ASTLiteral::Comment(x) => JSComment(x),
            ASTLiteral::Null() => JSUndefined(),
        }
    }
}
