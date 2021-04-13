use itertools::Itertools;
use std::iter::Peekable;
use std::str::Chars;

// Note
// Iterator::take_while consumes the character for which the predicate returns false.
// This screws up lexing in some places where we still want to look at that character.
// Itertools::peeking_take_while avoids this issue.

#[derive(Debug, PartialEq)]
pub enum Token {
    Asterisk,
    Comma,
    From,
    Select,
    Identifier(String),
    NumericLiteral(i64),
    StringLiteral(String),
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = input.chars().into_iter().peekable();
    while let Some(chr) = chars.peek() {
        match chr {
            '*' => {
                tokens.push(Token::Asterisk);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            '"' => lex_quoted_identifier(&mut chars, &mut tokens),
            '\'' => lex_string(&mut chars, &mut tokens),
            c if c.is_alphabetic() => lex_identifier(&mut chars, &mut tokens),
            c if c.is_digit(10) => lex_numeric(&mut chars, &mut tokens),
            // Consume Whitespace
            ' ' | '\n' => {
                chars.next();
            }
            c => {
                panic!("Unexpected character while lexing: {}", c);
            }
        }
    }
    tokens
}

fn lex_identifier(chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>) {
    let ident: String = chars.peeking_take_while(|c| c.is_alphanumeric()).collect();
    let token = match ident.to_uppercase().as_str() {
        "SELECT" => Token::Select,
        "FROM" => Token::From,
        _ => Token::Identifier(ident),
    };
    tokens.push(token)
}

fn lex_quoted_identifier(chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>) {
    chars.next(); // Skip leading "
                  // Using .take_while to skip trailing "
    let ident: String = chars.take_while(|c| *c != '"').collect();
    tokens.push(Token::Identifier(ident))
}

fn lex_string(chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>) {
    chars.next(); // Skip leading '
                  // Using .take_while to skip trailing '
    let value: String = chars.take_while(|c| *c != '\'').collect();
    tokens.push(Token::StringLiteral(value))
}

fn lex_numeric(chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>) {
    let digit: String = chars.peeking_take_while(|c| c.is_digit(10)).collect();
    let value = digit.parse::<i64>();
    tokens.push(Token::NumericLiteral(value.unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_select_star() {
        let sql = "SELECT * FROM fish";
        let tokens = lex(sql);
        assert_eq!(
            tokens,
            vec![
                Token::Select,
                Token::Asterisk,
                Token::From,
                Token::Identifier(String::from("fish"))
            ]
        );
    }

    #[test]
    fn lex_projection() {
        let sql = "SELECT Name FROM fish";
        let tokens = lex(sql);
        assert_eq!(
            tokens,
            vec![
                Token::Select,
                Token::Identifier(String::from("Name")),
                Token::From,
                Token::Identifier(String::from("fish"))
            ]
        );
    }

    #[test]
    fn lex_projections() {
        let sql = r##"SELECT Name, "Unique Entry ID", "#" FROM fish"##;
        let tokens = lex(sql);
        assert_eq!(
            tokens,
            vec![
                Token::Select,
                Token::Identifier(String::from("Name")),
                Token::Comma,
                Token::Identifier(String::from("Unique Entry ID")),
                Token::Comma,
                Token::Identifier(String::from("#")),
                Token::From,
                Token::Identifier(String::from("fish"))
            ]
        );
    }

    #[test]
    fn lex_values() {
        let sql = "SELECT 1, 'one'";
        let tokens = lex(sql);
        assert_eq!(
            tokens,
            vec![
                Token::Select,
                Token::NumericLiteral(1),
                Token::Comma,
                Token::StringLiteral(String::from("one"))
            ]
        )
    }
}
