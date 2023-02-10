use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[regex("let ([a-zA-Z_][a-zA-Z0-9_]*)", |lex| lex.slice().split_at(4).1.to_owned())]
    VariableDeclaration(String),

    #[token("=")]
    Equal,

    #[regex(r"[0-9]+\.?[0-9]*", |lex| lex.slice().parse())]
    Number(f64),

    #[regex("\"([^\"]|(\\\\\"))*\"", |lex| lex.slice()
    .to_owned()
    .strip_prefix("\"")
    .unwrap()
    .strip_suffix("\"")
    .unwrap()
    .replace("\\n", "\n")
    .replace("\\r", "\r")
    .replace("\\t", "\t")
    .replace("\\\"", "\""))]
    String(String),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error
}

fn main() {
    let mut lexer = Token::lexer("let zmienna = 5
let Zmie_nna2 = \"a tutaj przykład \\\"stringa\\\"\"");
    while let Some(token) = lexer.next() {
        println!("{:?}", token);
        if let Token::String(val) = token {
            println!("{}", val);
        }
    }
}