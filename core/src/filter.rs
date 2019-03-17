use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::iter::Peekable;
use std::str::CharIndices;
use uuid::Uuid;

use super::Card;

#[derive(Debug)]
pub struct Error {
    pos: Option<usize>,
    what: String,
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.what
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(pos) = self.pos {
            write!(f, "pos {}: {}", pos, self.what)
        } else {
            write!(f, "{}", self.what)
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

pub trait Expression: Debug {
    fn matches(&self, card: &Card) -> bool;
}

#[derive(Debug)]
struct And(Box<Expression>, Box<Expression>);

impl Expression for And {
    fn matches(&self, card: &Card) -> bool {
        self.0.matches(card) && self.1.matches(card)
    }
}

#[derive(Debug)]
struct Or(Box<Expression>, Box<Expression>);

impl Expression for Or {
    fn matches(&self, card: &Card) -> bool {
        self.0.matches(card) || self.1.matches(card)
    }
}

#[derive(Debug)]
struct InList(Uuid);

impl Expression for InList {
    fn matches(&self, card: &Card) -> bool {
        card.list == self.0
    }
}

#[derive(Debug)]
struct HasText(String);

impl Expression for HasText {
    fn matches(&self, card: &Card) -> bool {
        card.title.contains(&self.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Token<'a> {
    OpenParen,
    CloseParen,
    Colon,
    And,
    Or,
    Word(&'a str),
}

#[derive(PartialEq, Eq)]
struct TokenItem<'a> {
    pos: usize,
    token: Token<'a>,
}

struct Tokenizer<'a> {
    text: &'a str,
    iter: Peekable<CharIndices<'a>>,
}

impl<'a> Tokenizer<'a> {
    fn new(text: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            text,
            iter: text.char_indices().peekable(),
        }
    }

    fn is_whitespace(c: char) -> bool {
        c == ' ' || c == '\t' || c == '\r' || c == '\n'
    }

    fn is_word(c: char) -> bool {
        !(Self::is_whitespace(c) || c == '(' || c == ')' || c == ':' || c == '"')
    }

    fn read_word(&mut self) -> Result<&'a str> {
        let start = if let Some(&(start, _)) = self.iter.peek() {
            start
        } else {
            return Err(Error {
                pos: None,
                what: "Internal error: reading word at end of string".into(),
            });
        };
        while let Some(&(end, c)) = self.iter.peek() {
            if Self::is_whitespace(c) || c == ':' || c == ')' {
                return Ok(&self.text[start..end]);
            } else if Self::is_word(c) {
                self.iter.next().unwrap();
            } else {
                return Err(Error {
                    pos: Some(end),
                    what: format!("Unexpected token {} in word", c),
                });
            }
        }
        Ok(&self.text[start..])
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<TokenItem<'a>>;

    fn next(&mut self) -> Option<Result<TokenItem<'a>>> {
        while let Some(&(start, c)) = self.iter.peek() {
            if Self::is_whitespace(c) {
                self.iter.next().unwrap();
                continue;
            }
            let token = if c == '"' {
                self.iter.next().unwrap();
                while let Some((end, c)) = self.iter.next() {
                    if c == '"' {
                        return Some(Ok(TokenItem {
                            pos: start,
                            token: Token::Word(&self.text[start + 1..end]),
                        }));
                    }
                }
                return Some(Err(Error {
                    pos: Some(start),
                    what: "Unterminated quoted word".into(),
                }));
            } else if c == '(' {
                self.iter.next().unwrap();
                Token::OpenParen
            } else if c == ')' {
                self.iter.next().unwrap();
                Token::CloseParen
            } else if c == ':' {
                self.iter.next().unwrap();
                Token::Colon
            } else {
                let word = match self.read_word() {
                    Ok(w) => w,
                    Err(e) => return Some(Err(e)),
                };
                if word == "OR" {
                    Token::Or
                } else if word == "AND" {
                   Token::And
                } else {
                    Token::Word(word)
                }
            };
            return Some(Ok(TokenItem {
                pos: start,
                token,
            }));
        }
        None
    }
}

struct Parser<'a> {
    tokens: Peekable<Tokenizer<'a>>,
    lists: &'a HashMap<String, Uuid>,
}

impl<'a> Parser<'a> {
    fn parse_all(&mut self) -> Result<Box<Expression>> {
        let expr = self.parse(0)?;
        match self.tokens.next() {
            None => {}, // Good, input is exhausted
            Some(Err(e)) => return Err(e),
            Some(Ok(token)) => {
                return Err(Error {
                    pos: Some(token.pos),
                    what: format!("Unexpected token {:?}", token.token),
                });
            }
        }
        Ok(expr)
    }

    fn parse(&mut self, mut precedence: i8) -> Result<Box<Expression>> {
        let &TokenItem { ref token, pos } = match self.tokens.peek() {
            None => return Err(Error {
                pos: None,
                what: "Empty input".into(),
            }),
            Some(Err(_)) => return Err(self.tokens.next().unwrap().err().unwrap()),
            Some(Ok(ti)) => ti,
        };

        // Read LHS
        let mut expr = match token {
            Token::OpenParen => {
                self.tokens.next().unwrap()?;
                let expr = self.parse(0)?;
                if let Some(Ok(TokenItem { token: Token::CloseParen, .. })) = self.tokens.next() {
                    expr
                } else {
                    return Err(Error {
                        pos: Some(pos),
                        what: "Unmatched parenthesis".into(),
                    });
                }
            }
            Token::CloseParen|Token::Colon|Token::And|Token::Or => {
                return Err(Error {
                    pos: Some(pos),
                    what: format!("Unexpected token {:?}", token),
                });
            }
            Token::Word(_) => {
                self.parse_pred()?
            }
        };

        // Read operators
        loop {
            let &TokenItem { ref token, pos } = match self.tokens.peek() {
                None => break,
                Some(Ok(ti)) => ti,
                Some(Err(_)) => return Err(self.tokens.next().unwrap().err().unwrap()),
            };

            match token {
                Token::OpenParen => {
                    expr = Box::new(And(expr, self.parse(0)?));
                }
                Token::CloseParen => break,
                Token::Colon => {
                    return Err(Error {
                        pos: Some(pos),
                        what: format!("Unexpected token {:?}", token),
                    });
                }
                Token::And => {
                    if precedence < 1 {
                        self.tokens.next().unwrap()?;
                        expr = Box::new(And(expr, self.parse(1)?));
                        precedence = 1;
                    } else {
                        break;
                    }
                }
                Token::Or => {
                    if precedence < 2 {
                        self.tokens.next().unwrap()?;
                        expr = Box::new(Or(expr, self.parse(2)?));
                        precedence = 2;
                    } else {
                        break;
                    }
                }
                Token::Word(_) => {
                    if precedence < 1 {
                        expr = Box::new(And(expr, self.parse_pred()?));
                        precedence = 1;
                    } else {
                        break;
                    }
                }
            }
        }
        Ok(expr)
    }

    fn parse_pred(&mut self) -> Result<Box<Expression>> {
        let TokenItem { token, pos } = match self.tokens.next() {
            None => {
                return Err(Error {
                    pos: None,
                    what: "Internal error: reading predicate at end of input"
                          .into(),
                });
            }
            Some(ti) => ti?,
        };

        // Get word
        let word = match token {
            Token::Word(w) => w,
            _ => {
                return Err(Error {
                    pos: Some(pos),
                    what: format!(
                        "Internal error: reading predicate which is not a \
                         word: {:?}",
                        token),
                });
            }
        };

        // Optional colon and other word
        if let Some(&Ok(TokenItem { token: Token::Colon, .. })) =
            self.tokens.peek()
        {
            self.tokens.next().unwrap()?;
            let TokenItem { token, pos } = match self.tokens.next() {
                None => {
                    return Err(Error {
                        pos: Some(pos),
                        what: "Missing argument after operation".into(),
                    });
                }
                Some(ti) => ti?,
            };
            match token {
                Token::Word(arg) => {
                    self.recognize_pred(word, arg, pos)
                }
                _ => {
                    return Err(Error {
                        pos: Some(pos),
                        what: "Missing argument after operation".into(),
                    });
                }
            }
        } else {
            Ok(Box::new(HasText(word.into())))
        }
    }

    fn recognize_pred(&self, op: &'a str, arg: &'a str, pos: usize,
    ) -> Result<Box<Expression>>
    {
        match op {
            "list" => {
                match self.lists.get(arg) {
                    None => {
                        return Err(Error {
                            pos: Some(pos),
                            what: format!("Unknown list {}", arg),
                        });
                    }
                    Some(list_id) => Ok(Box::new(InList(list_id.clone()))),
                }
            }
            _ => {
                return Err(Error {
                    pos: Some(pos),
                    what: format!("Unknown operator {}", op),
                });
            }
        }
    }
}

/// Parse text into an Expression.
///
/// This takes in a mapping for the lists, so that the user can enter list
/// names instead of UUIDs
pub fn parse(text: &str, lists: &HashMap<String, Uuid>,
) -> Result<Box<Expression>>
{
    let mut parser = Parser {
        tokens: Tokenizer::new(text).peekable(),
        lists,
    };
    parser.parse_all()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use uuid::Uuid;

    use super::{parse, Error, Token, TokenItem, Tokenizer};

    #[test]
    fn test_tokenizer() {
        let input = "some-word pred:other-word OR pred2:(thing) AND \"other thing\"";
        let expected = [
            TokenItem { pos: 0, token: Token::Word("some-word") },
            TokenItem { pos: 10, token: Token::Word("pred") },
            TokenItem { pos: 14, token: Token::Colon },
            TokenItem { pos: 15, token: Token::Word("other-word") },
            TokenItem { pos: 26, token: Token::Or },
            TokenItem { pos: 29, token: Token::Word("pred2") },
            TokenItem { pos: 34, token: Token::Colon },
            TokenItem { pos: 35, token: Token::OpenParen },
            TokenItem { pos: 36, token: Token::Word("thing") },
            TokenItem { pos: 41, token: Token::CloseParen },
            TokenItem { pos: 43, token: Token::And },
            TokenItem { pos: 47, token: Token::Word("other thing") },
        ];
        let tokens = Tokenizer::new(input).map(Result::unwrap).collect::<Vec<_>>();
        assert!(tokens == expected);
    }

    #[test]
    fn test_parse() {
        let map = [
            ("todo", "936DA01F9ABD4D9D80C7000011110001"),
            ("doing", "936DA01F9ABD4D9D80C7000011110002"),
            ("done", "936DA01F9ABD4D9D80C7000011110003"),
        ].into_iter().map(|(n, i)| {
            ((*n).into(), Uuid::parse_str(i).unwrap())
        }).collect::<HashMap<String, Uuid>>();

        let string = "(list:todo OR list:doing) word1 (word2 word3 OR word4)";
        parse(string, &map).expect("Parsing failed");

        if let Err(Error { pos, what }) = parse("", &map) {
            assert!(pos == None);
            assert!(what == "Empty input");
        } else {
            panic!("Parsing empty input succeeded");
        }
    }
}
