use crate::ast::{Node, Op};
use crate::token::Token;

pub struct Parser {
    toks: Vec<Token>,
    i: usize,
}

impl Parser {
    pub fn new(toks: Vec<Token>) -> Self {
        Self { toks, i: 0 }
    }

    fn peek(&self) -> Option<&Token> { self.toks.get(self.i) }
    fn bump(&mut self) -> Option<Token> {
        if self.i < self.toks.len() {
            let t = self.toks[self.i].clone();
            self.i += 1;
            Some(t)
        } else { None }
    }

    pub fn parse(&mut self) -> Result<Node, String> { self.parse_expr() }

    fn parse_expr(&mut self) -> Result<Node, String> {
        match self.peek() {
            Some(Token::Number(_)) => {
                if let Some(Token::Number(v)) = self.bump() {
                    Ok(Node::Number(v))
                } else { unreachable!() }
            }
            Some(Token::LParen) => self.parse_list(),
            Some(t) => Err(format!("Unexpected token in expression: {:?}", t)),
            None => Err("Unexpected end of input".into()),
        }
    }

    fn parse_list(&mut self) -> Result<Node, String> {
        match self.bump() {
            Some(Token::LParen) => {}
            _ => return Err("Expected '('".into()),
        }

        let op = match self.bump() {
            Some(Token::Plus)  => Op::Add,
            Some(Token::Minus) => Op::Sub,
            Some(Token::Star)  => Op::Mul,
            Some(Token::Slash) => Op::Div,
            other => return Err(format!("Expected operator after '(' but found {:?}", other)),
        };

        let mut args = Vec::new();
        loop {
            match self.peek() {
                Some(Token::RParen) => { self.bump(); break; }
                Some(_) => {
                    let e = self.parse_expr()?;
                    args.push(e);
                }
                None => return Err("Unclosed '('.".into()),
            }
        }

        Ok(Node::Call(op, args))
    }
}

pub fn parse_tokens(toks: Vec<Token>) -> Result<Node, String> {
    Parser::new(toks).parse()
}
