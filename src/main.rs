fn main() {
    let input = "2 * 7 + 5";
    get_tokens(input);
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
}

#[derive(Debug)]
enum TokenType {
    Num,
    Plus,
    Times,
    Identifier,
    True,
    False,
    EOF,
}

fn get_tokens(input: &str) {
    let mut tokens: Vec<Token> = Vec::new();
    tokens.push(Token {
        token_type: TokenType::Identifier,
    });

    println!("{:?}", tokens);
}
