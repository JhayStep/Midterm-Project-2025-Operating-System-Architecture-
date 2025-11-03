use crate::token::Token;

pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut toks = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' | '\r' => { chars.next(); }
            '(' => { chars.next(); toks.push(Token::LParen); }
            ')' => { chars.next(); toks.push(Token::RParen); }
            '+' => { chars.next(); toks.push(Token::Plus); }
            '-' => { chars.next(); toks.push(Token::Minus); }
            '*' => { chars.next(); toks.push(Token::Star); }
            '/' => { chars.next(); toks.push(Token::Slash); }
            '0'..='9' => {
                let mut num = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() {
                        num.push(d);
                        chars.next();
                    } else { break; }
                }
                let v = num.parse::<i64>().map_err(|e| e.to_string())?;
                toks.push(Token::Number(v));
            }
            _ => return Err(format!("Unexpected character: '{}'", c)),
        }
    }

    Ok(toks)
}
