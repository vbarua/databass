use crate::lexer::{lex, Token};

#[derive(Debug, PartialEq)]
pub struct Select {
    pub from: String,
}

pub fn parse(input: &str) -> Select {
    let tokens = lex(input);
    match &tokens[..] {
        [Token::Select, Token::Identifier(_), Token::From, Token::Identifier(table)] => Select {
            from: table.clone(),
        },
        _ => panic!("Can only process 'SELECT * FROM <table>'zs"),
    }
}

#[test]
fn parse_select() {
    let sql = "SELECT * FROM fish";
    let query = parse(sql);
    assert_eq!(
        query,
        Select {
            from: String::from("fish")
        }
    )
}
