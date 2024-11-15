#[derive(Debug, Clone)]
pub enum Token {
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Plus,
    Identifier(String),
    StringLiteral(String),
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.trim().chars().peekable();

    while let Some(c) = chars.next() {
        if c.is_whitespace() {
            continue;
        }
        match c {
            '[' => tokens.push(Token::LeftBracket),
            ']' => tokens.push(Token::RightBracket),
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            '+' => tokens.push(Token::Plus),
            '"' => {
                let mut literal = String::new();
                for c in chars.by_ref() {
                    if c == '"' {
                        break;
                    }
                    literal.push(c);
                }
                tokens.push(Token::StringLiteral(literal));
            }
            _ => {
                let mut identifier = String::new();
                identifier.push(c);
                while let Some(&c) = chars.peek() {
                    if !c.is_alphanumeric() {
                        break;
                    }
                    identifier.push(chars.next().unwrap());
                }
                tokens.push(Token::Identifier(identifier));
            }
        }
    }
    tokens
}
