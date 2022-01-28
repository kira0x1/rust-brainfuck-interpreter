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

#[derive(PartialEq, Eq, Debug)]
enum TokenType {
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

fn lexer(input: &str) {
    let mut tokens: Vec<Token> = Vec::new();
    let mut cur_pos = 0;

    while cur_pos < input.len() {
        let token_start = cur_pos;
        let look_ahead = input.chars().nth(cur_pos).unwrap_or(' ');
        let token_type = get_token_type(look_ahead);

        if token_type == TokenType::UNKNOWN {
            println!("unknown characters {} at position {}", look_ahead, cur_pos);
            break;
        }

        if token_type == TokenType::Num {
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
            continue;
        }

        cur_pos += 1;

        if token_type == TokenType::EOF {
            tokens.push(Token {
                token_type: TokenType::EOF,
                text: "<EOF>".to_owned(),
                start_pos: cur_pos,
            });

            continue;
        }

        tokens.push(Token {
            token_type: token_type,
            text: look_ahead.to_string(),
            start_pos: token_start,
        });
    }

    println!("{:?}", tokens);
}

fn get_token_type(input: char) -> TokenType {
    if input == ' ' {
        return TokenType::WhiteSpace;
    }

    if input == '+' {
        return TokenType::Plus;
    }

    if input == '*' {
        return TokenType::Times;
    }

    if input.is_numeric() {
        return TokenType::Num;
    }

    return TokenType::UNKNOWN;
}
