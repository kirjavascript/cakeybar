#[macro_use]
extern crate nom;

#[derive(Debug)]
enum Token {
    Text(String),
    Symbol(String),
}

named!(symbol<&str,Token>,
    map!(
        delimited!(char!('{'), is_not!("{}"), char!('}')),
        |s| Token::Symbol(s.to_string())
    )
);

named!(text<&str,Token>,
   map!(
       is_not!("{}"),
       |s| Token::Text(s.to_string())
   )
);

named!(get_token<&str,Token>,
    alt!( symbol | text )
);

named!(get_tokens<&str,Vec<Token>,
    many0!( get_token )
);

fn main() {
    let mut input = " hello% {one} test {two} hello {one} poop  ";
    let mut tokens: Vec<Token> = Vec::new();
    loop {
        if let Ok((rest, token)) = get_token(input) {
            input = rest;
            tokens.push(token);
        } else {
            break;
        }
    }
    println!("{:#?}", tokens);
}
