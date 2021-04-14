use crate::lexer::{lex, Token};
use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug, PartialEq)]
pub struct Select {
    pub select_list: SelectList,
    pub from: From,
}

#[derive(Debug, PartialEq)]
pub struct From {
    pub table: String,
}

#[derive(Debug, PartialEq)]
pub enum SelectList {
    All,
    Subset(Vec<SelectListItem>),
}

#[derive(Debug, PartialEq)]
pub struct SelectListItem {
    name: String,
}

pub fn parse(input: &str) -> Select {
    let ts = lex(input);
    let mut tokens = ts.iter().peekable();
    match tokens.next() {
        Some(Token::Select) => (),
        _ => panic!("Query should start with SELECT"),
    }
    let select_list = parse_select_list(&mut tokens);
    let from = parse_from(&mut tokens);

    Select { select_list, from }
}

type TokenStream<'a> = Peekable<Iter<'a, Token>>;

fn parse_select_list(tokens: &mut TokenStream) -> SelectList {
    match tokens.peek() {
        Some(Token::Asterisk) => parse_select_all(tokens),
        Some(Token::Identifier(_)) => SelectList::Subset(parse_select_items(tokens)),
        _ => panic!("Expected either * or 1 or more identifiers"),
    }
}

fn parse_select_all(tokens: &mut TokenStream) -> SelectList {
    match tokens.next() {
        Some(Token::Asterisk) => (),
        _ => panic!("Expected *"),
    }
    match tokens.peek() {
        Some(Token::From) => (),
        _ => panic!("* was not followed by FROM"),
    }
    SelectList::All
}

fn parse_select_items(tokens: &mut TokenStream) -> Vec<SelectListItem> {
    let mut select_items: Vec<SelectListItem> = Vec::new();
    while let Some(token) = tokens.next() {
        dbg!("{}", token);
        let name = match token {
            Token::Identifier(name) => name.clone(),
            _ => panic!("Expected identifier"),
        };
        select_items.push(SelectListItem { name });

        match tokens.peek() {
            Some(Token::Comma) => {
                tokens.next();
            }
            Some(Token::From) => return select_items,
            _ => panic!("Expected either , or FROM"),
        }
    }
    panic!("Missing FROM");
}

fn parse_from(tokens: &mut TokenStream) -> From {
    match tokens.next() {
        Some(Token::From) => (),
        _ => panic!("Expected FROM"),
    }

    let table = match tokens.next() {
        Some(Token::Identifier(name)) => name.clone(),
        _ => panic!("Expected table identifier after FROM"),
    };

    From { table }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_select_star() {
        let sql = "SELECT * FROM fish";
        let query = parse(sql);
        assert_eq!(
            query,
            Select {
                select_list: SelectList::All,
                from: From {
                    table: String::from("fish")
                },
            }
        )
    }

    #[test]
    fn parse_select_projection() {
        let sql = "SELECT Name FROM fish";
        let query = parse(sql);
        assert_eq!(
            query,
            Select {
                select_list: SelectList::Subset(vec![SelectListItem {
                    name: String::from("Name")
                }]),
                from: From {
                    table: String::from("fish")
                },
            }
        )
    }

    #[test]
    fn parse_select_projections() {
        let sql = r##"SELECT Name, "Unique Entry ID", "#" FROM fish"##;
        let query = parse(sql);
        assert_eq!(
            query,
            Select {
                select_list: SelectList::Subset(vec![
                    SelectListItem {
                        name: String::from("Name"),
                    },
                    SelectListItem {
                        name: String::from("Unique Entry ID")
                    },
                    SelectListItem {
                        name: String::from("#")
                    }
                ]),
                from: From {
                    table: String::from("fish")
                },
            }
        )
    }
}
