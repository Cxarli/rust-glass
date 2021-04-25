use token;


pub type AST = Vec<Box<ASTField>>;


#[derive(Debug, Clone, PartialEq)]
pub enum ASTField {
    ASTItem(ASTItemType),
    ASTGroup(ASTGroupType, String, AST),
}


#[derive(Debug, Clone, PartialEq)]
pub enum ASTItemType {
    Literal(ASTLiteral),
    Action(ASTAction),
}


#[derive(Debug, Clone, PartialEq)]
pub enum ASTGroupType {
    Function(),
    Class(),
    While(),
}


#[derive(Debug, Clone, PartialEq)]
pub enum ASTLiteral {
    String(String),
    Number(i32),
    Name(String),
    Comment(String),
    Null(),
}


#[derive(Debug, Clone, PartialEq)]
pub enum ASTAction {
    PopValueFromStack(),
    Return(),
    SwapTwoFromStack(),

    Assign(ASTLiteral, ASTLiteral), // name, value
    AssignFromStack(),

    NewInstance(ASTLiteral, ASTLiteral), // name, cname
    NewInstanceFromStack(),

    RetrieveFunction(ASTLiteral, ASTLiteral), // oname, fname
    RetrieveFunctionFromStack(),

    PopAndRunFunction(),

    RetrieveFromName(ASTLiteral), // name
    RetrieveFromNameFromStack(),

    AssignCurrent(ASTLiteral), // name
    AssignCurrentFromStack(),

    PushLiteralToStack(ASTLiteral),
}


impl From<token::Token> for ASTLiteral {
    fn from(token_val: token::Token) -> ASTLiteral {
        use token::Token;

        match token_val {
            Token::String(s) => ASTLiteral::String(s),
            Token::Number(x) => ASTLiteral::Number(x),
            Token::Name(n) => ASTLiteral::Name(n),
            _ => panic!("Can't convert to ASTLiteral"),
        }
    }
}


impl PartialEq<&'static str> for ASTLiteral {
    fn eq(&self, rhs: &&'static str) -> bool {
        match *self {
            ASTLiteral::String(ref s) => rhs == s,
            ASTLiteral::Name(ref n) => rhs == n,
            _ => panic!("Can't compare {:?} with {:?}", &self, &rhs),
        }
    }
}


impl PartialEq<i32> for ASTLiteral {
    fn eq(&self, rhs: &i32) -> bool {
        match *self {
            ASTLiteral::Number(ref x) => rhs == x,
            _ => panic!("Can't compare {:?} with {:?}", &self, &rhs),
        }
    }
}
