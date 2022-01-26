fn main() {
    let input = "2 * 7 + 5";
    lexer(input);
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    text: String,
    start_pos: usize,
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

fn lexer(input: &str) {
    let mut tokens: Vec<Token> = Vec::new();
    let mut cur_pos = 0;

    while cur_pos < input.len() {
        let mut token_start = cur_pos;

        let look_ahead = input.chars().nth(cur_pos).unwrap_or(' ');

        if look_ahead == ' ' {
            cur_pos += 1;
        } else if look_ahead == '+' {
            cur_pos += 1;

            tokens.push(Token {
                token_type: TokenType::Plus,
                text: look_ahead.to_string(),
                start_pos: token_start,
            });
        } else if look_ahead == '*' {
            cur_pos += 1;

            tokens.push(Token {
                token_type: TokenType::Times,
                text: look_ahead.to_string(),
                start_pos: token_start,
            });
        } else if look_ahead.is_numeric() {
            let mut text: String = "".to_owned();
            while cur_pos < input.len() && input.chars().nth(cur_pos).unwrap_or(' ').is_numeric() {
                text.push_str(&input.chars().nth(cur_pos).unwrap_or(' ').to_string());
                cur_pos += 1;
                tokens.push(Token {
                    token_type: TokenType::Num,
                    text: text.to_owned(),
                    start_pos: token_start,
                })
            }
        } else {
            println!("unknown characters {} at position {}", look_ahead, cur_pos);
            break;
        }

        tokens.push(Token {
            token_type: TokenType::EOF,
            text: "<EOF>".to_owned(),
            start_pos: cur_pos,
        })
    }

    println!("{:?}", tokens);
}
