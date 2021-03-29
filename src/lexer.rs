#[derive(Debug, PartialEq)]
pub enum Token {
    Select,
    From,
    Identifier(String),
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    for i in input.split_whitespace() {
        let token = match i {
            "SELECT" => Token::Select,
            "FROM" => Token::From,
            s => Token::Identifier(String::from(s)),
        };
        tokens.push(token)
    }
    tokens
}

#[test]
fn lex_sql() {
    let sql = "SELECT * FROM fish";
    let tokens = lex(sql);
    assert_eq!(
        tokens,
        vec![
            Token::Select,
            Token::Identifier(String::from("*")),
            Token::From,
            Token::Identifier(String::from("fish"))
        ]
    );
}
