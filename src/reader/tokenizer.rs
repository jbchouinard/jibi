use std::fmt;

use lazy_static::lazy_static;
use regex::Regex;

use crate::state::JState;
use crate::types::{JTFloat, JTInt};
use crate::PositionTag;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
    Char(char),
    Int(JTInt),
    Float(JTFloat),
    Ident(String),
    String(String),
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub value: TokenValue,
    pub pos: PositionTag,
}

impl Token {
    pub fn new(value: TokenValue, pos: PositionTag) -> Self {
        Self { value, pos }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        self.value == other.value
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TokenValue::*;
        match &self.value {
            Int(n) => write!(f, "INT({})", n),
            Float(x) => write!(f, "FLOAT({})", x),
            Ident(s) => write!(f, "IDENT({})", s),
            String(s) => write!(f, "STRING(\"{}\")", s),
            Eof => write!(f, "EOF"),
            Char(c) => write!(f, "CHAR('{}')", c),
        }
    }
}

lazy_static! {
    static ref RE_WS: Regex = Regex::new(r"^\s+").unwrap();
    static ref RE_IDENT: Regex = Regex::new(
        r"(?x)
            ^
            ([a-zA-Z+.*/<>=!?$%_&~^-][0-9a-zA-Z+.*/<=>!?$%_&~^-]*)
            (::[a-zA-Z+.*/<>=!?$%_&~^-][0-9a-zA-Z+.*/<=>!?$%_&~^-]*)*
        "
    )
    .unwrap();
    static ref RE_STRING: Regex = Regex::new(r#"^"([^"]|\\")*""#).unwrap();
    static ref RE_FLOAT: Regex = Regex::new(r"^-?([.][0-9]+|[0-9]+[.][0-9]*)").unwrap();
    static ref RE_INT: Regex = Regex::new(r"^-?[0-9]+").unwrap();
    static ref RE_COMMENT: Regex = Regex::new(r"^;[^\n]*").unwrap();
    static ref RE_CHAR: Regex = Regex::new(r"^.").unwrap();
}

type TResult = std::result::Result<TokenValue, String>;

fn t_int(val: &str) -> TResult {
    match val.parse::<JTInt>() {
        Ok(n) => Ok(TokenValue::Int(n)),
        Err(e) => Err(format!("int error: {}", e)),
    }
}

fn t_float(val: &str) -> TResult {
    match val.parse::<JTFloat>() {
        Ok(n) => Ok(TokenValue::Float(n)),
        Err(e) => Err(format!("float error: {}", e)),
    }
}

fn t_ident(val: &str) -> TResult {
    Ok(TokenValue::Ident(val.to_string()))
}

fn t_string(val: &str) -> TResult {
    Ok(TokenValue::String(val[1..val.len() - 1].to_string()))
}

fn t_char(s: &str) -> TResult {
    Ok(TokenValue::Char(s.chars().into_iter().next().unwrap()))
}

pub struct Tokenizer {
    filename: String,
    input: String,
    pos: usize,
    lineno: usize,
    last_newline_pos: usize,
}

impl Tokenizer {
    pub fn new(filename: String, input: String) -> Self {
        Self::with_lineno(filename, input, 1)
    }
    pub fn with_lineno(filename: String, input: String, lineno: usize) -> Self {
        Self {
            filename,
            input,
            pos: 0,
            lineno,
            last_newline_pos: 0,
        }
    }
    fn ptag(&self, pos: usize) -> PositionTag {
        PositionTag {
            filename: self.filename.clone(),
            lineno: self.lineno,
            col: pos - self.last_newline_pos,
        }
    }

    fn eat_comment(&mut self) -> bool {
        match RE_COMMENT.find(&self.input[self.pos..]) {
            Some(mat) => {
                self.pos += mat.end();
                true
            }
            None => false,
        }
    }

    fn eat_whitespace(&mut self) -> bool {
        match RE_WS.find(&self.input[self.pos..]) {
            Some(mat) => {
                let spos = self.pos;
                self.pos += mat.end();
                let mut newline_count = 0;
                for (p, c) in mat.as_str().chars().enumerate() {
                    if c == '\n' {
                        newline_count += 1;
                        self.last_newline_pos = spos + p;
                    }
                }
                self.lineno += newline_count;
                true
            }
            None => false,
        }
    }

    fn try_token<T>(&mut self, re: &Regex, cons: T) -> Result<Option<Token>, TokenError>
    where
        T: Fn(&str) -> TResult,
    {
        match re.find(&self.input[self.pos..]) {
            Some(mat) => {
                let spos = self.pos;
                self.pos += mat.end();
                match cons(mat.as_str()) {
                    Ok(tokval) => Ok(Some(Token::new(tokval, self.ptag(spos)))),
                    Err(reason) => Err(TokenError::new(&reason, self.ptag(spos))),
                }
            }
            None => Ok(None),
        }
    }
}

pub trait TokenProducer {
    fn next_token(&mut self, state: &mut JState) -> Result<Token, TokenError>;
}

pub struct TokenIterator<'a, 'b, T: TokenProducer + ?Sized> {
    tokeniter: &'a mut T,
    state: &'b mut JState,
}

impl<'a, 'b, T: TokenProducer> Iterator for TokenIterator<'a, 'b, T> {
    type Item = Result<Token, TokenError>;
    fn next(self: &mut TokenIterator<'a, 'b, T>) -> Option<Result<Token, TokenError>> {
        match self.tokeniter.next_token(self.state) {
            Ok(tok) => match tok.value {
                TokenValue::Eof => None,
                _ => Some(Ok(tok)),
            },
            Err(e) => Some(Err(e)),
        }
    }
}

pub trait TokenToIter {
    fn to_iter<'a, 'b>(&'a mut self, state: &'b mut JState) -> TokenIterator<'a, 'b, Self>
    where
        Self: TokenProducer;
}

impl<T: TokenProducer> TokenToIter for T {
    fn to_iter<'a, 'b>(&'a mut self, state: &'b mut JState) -> TokenIterator<'a, 'b, Self> {
        TokenIterator {
            tokeniter: self,
            state,
        }
    }
}

impl TokenProducer for Tokenizer {
    fn next_token(&mut self, _state: &mut JState) -> Result<Token, TokenError> {
        while self.eat_whitespace() || self.eat_comment() {}

        if self.pos >= self.input.len() {
            return Ok(Token::new(TokenValue::Eof, self.ptag(self.pos)));
        }
        if let Some(token) = self.try_token(&RE_FLOAT, t_float)? {
            return Ok(token);
        }
        if let Some(token) = self.try_token(&RE_INT, t_int)? {
            return Ok(token);
        }
        if let Some(token) = self.try_token(&RE_IDENT, t_ident)? {
            return Ok(token);
        }
        if let Some(token) = self.try_token(&RE_STRING, t_string)? {
            return Ok(token);
        }
        if let Some(token) = self.try_token(&RE_CHAR, t_char)? {
            return Ok(token);
        }
        Err(TokenError::new(
            &format!(
                "unexpected character {}",
                &self.input[self.pos..self.pos + 1]
            ),
            self.ptag(self.pos),
        ))
    }
}

impl TokenProducer for std::vec::IntoIter<Token> {
    fn next_token(&mut self, _: &mut JState) -> Result<Token, TokenError> {
        match self.next() {
            Some(tok) => Ok(tok),
            None => Ok(Token::new(TokenValue::Eof, PositionTag::new("", 0, 0))),
        }
    }
}

pub struct TokenValidator<'a> {
    filename: String,
    balance: Vec<TokenValue>,
    tokens: Vec<Token>,
    lineno: usize,
    state: &'a mut JState,
}

/// Balanced parens validation for multi-line input.
impl<'a> TokenValidator<'a> {
    pub fn new(filename: &str, state: &'a mut JState) -> Self {
        Self {
            filename: filename.to_string(),
            balance: vec![],
            tokens: vec![],
            lineno: 0,
            state,
        }
    }
    /// Returns None when more input is expected based on counting parens.
    /// Returns tokens when it looks like it may form a complete expression.
    pub fn input(&mut self, s: String) -> Result<Option<Vec<Token>>, TokenError> {
        self.lineno += 1;
        let mut tokenizer = Tokenizer::with_lineno(self.filename.clone(), s, self.lineno);
        let new_toks: Vec<Token> = tokenizer
            .to_iter(self.state)
            .collect::<Result<Vec<Token>, TokenError>>()?;
        for tok in new_toks {
            match tok.value {
                TokenValue::Char('(') => self.balance.push(TokenValue::Char('(')),
                TokenValue::Char(')') => match self.balance.pop() {
                    Some(TokenValue::Char('(')) => (),
                    _ => return Err(TokenError::new("unexpected closing parens", tok.pos)),
                },
                _ => (),
            }
            self.tokens.push(tok);
        }
        Ok(if self.balance.is_empty() {
            Some(std::mem::take(&mut self.tokens))
        } else {
            // Remove the last EOF token
            self.tokens.pop();
            None
        })
    }
}

#[derive(Debug)]
pub struct TokenError {
    pub pos: PositionTag,
    pub reason: String,
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Syntax error: {} at character {}", self.reason, self.pos,)
    }
}

impl TokenError {
    pub fn new(reason: &str, pos: PositionTag) -> Self {
        Self {
            pos,
            reason: reason.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_tokenizer(input: &str, expected: Vec<TokenValue>) {
        let mut tokenizer = Tokenizer::new("test".to_string(), input.to_string());
        let mut state = JState::new();
        let tokens = tokenizer
            .to_iter(&mut state)
            .collect::<Result<Vec<Token>, TokenError>>()
            .unwrap();
        let tokvalues: Vec<TokenValue> = tokens.into_iter().map(|t| t.value).collect();
        assert_eq!(expected, tokvalues);
    }

    #[test]
    fn test_tokenizer_1() {
        test_tokenizer(
            "(* 12 -15)",
            vec![
                TokenValue::Char('('),
                TokenValue::Ident("*".to_string()),
                TokenValue::Int(12),
                TokenValue::Int(-15),
                TokenValue::Char(')'),
            ],
        );
    }

    #[test]
    fn test_tokenizer_2() {
        test_tokenizer(
            "(concat \"foo\" \"bar\")",
            vec![
                TokenValue::Char('('),
                TokenValue::Ident("concat".to_string()),
                TokenValue::String("foo".to_string()),
                TokenValue::String("bar".to_string()),
                TokenValue::Char(')'),
            ],
        );
    }

    #[test]
    fn test_tokenizer_3() {
        test_tokenizer(
            "(quote '(1 2 3))",
            vec![
                TokenValue::Char('('),
                TokenValue::Ident("quote".to_string()),
                TokenValue::Char('\''),
                TokenValue::Char('('),
                TokenValue::Int(1),
                TokenValue::Int(2),
                TokenValue::Int(3),
                TokenValue::Char(')'),
                TokenValue::Char(')'),
            ],
        );
    }

    #[test]
    fn test_tokenizer_4() {
        test_tokenizer(
            "(quote ; this is a comment!
                '(1 2 3))",
            vec![
                TokenValue::Char('('),
                TokenValue::Ident("quote".to_string()),
                TokenValue::Char('\''),
                TokenValue::Char('('),
                TokenValue::Int(1),
                TokenValue::Int(2),
                TokenValue::Int(3),
                TokenValue::Char(')'),
                TokenValue::Char(')'),
            ],
        );
    }

    #[test]
    fn test_tokenizer_5() {
        test_tokenizer(
            "(* 12.0 -0.5)",
            vec![
                TokenValue::Char('('),
                TokenValue::Ident("*".to_string()),
                TokenValue::Float(12.0),
                TokenValue::Float(-0.5),
                TokenValue::Char(')'),
            ],
        );
    }

    #[test]
    fn test_tokenizer_6() {
        test_tokenizer("2.025", vec![TokenValue::Float(2.025)]);
    }

    #[test]
    fn test_tokenizer_7() {
        test_tokenizer(
            "(token 'char \")\")",
            vec![
                TokenValue::Char('('),
                TokenValue::Ident("token".to_string()),
                TokenValue::Char('\''),
                TokenValue::Ident("char".to_string()),
                TokenValue::String(")".to_string()),
                TokenValue::Char(')'),
            ],
        );
    }
}
