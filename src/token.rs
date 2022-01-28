#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TokenType {
    Num,
    Plus,
    Times,
    Identifier,
    True,
    False,
    WhiteSpace,
    EOF,
    UNKNOWN,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub text: String,
    pub start_pos: usize,
}
