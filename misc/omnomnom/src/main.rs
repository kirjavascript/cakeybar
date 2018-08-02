#[macro_use]
extern crate nom;

use nom::types::CompleteStr as Input;

#[derive(Debug)]
enum Token {
    Text(String),
    Symbol(String),
}


named!(symbol<Input,Token>,
    map!(
        delimited!(char!('{'), ws!(is_not!("{}")), char!('}')),
        |s| Token::Symbol(s.to_string())
    )
);

named!(text<Input,Token>,
   map!(
       is_not!("{}"),
       |s| Token::Text(s.to_string())
   )
);

named!(get_tokens<Input,Vec<Token>>,
    many0!( alt!( symbol | text ) )
);

fn main() {
    let input = Input("{one} hello% { one} test {two} hello {one} poop  ");
    println!("{:#?}", get_tokens(input));
}

// tests
// config.getFormat
