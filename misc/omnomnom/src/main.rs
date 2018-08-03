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
        delimited!(char!('{'), is_not!("{}"), char!('}')),
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

fn format_symbols<F: 'static>(input: &str, callback: F) -> String
    where F: Fn(&str) -> &str  {
    get_tokens(Input(input)).unwrap_or((Input(""), vec![])).1
        .iter()
        .map(|tok| {
            match tok {
                Token::Text(txt) => &txt,
                Token::Symbol(sym) => callback(&sym.trim()),
            }
        })
        .collect::<Vec<&str>>()
        .concat()
}

fn main() {
    let input = "hello {one} world { two } end";

    let output = format_symbols(input, |sym| {
        match sym {
            "one" => "ONE",
            "two" => "dongers!",
            _ => sym,
        }
    });

    println!("{:#?}", output); // "hello ONE world dongers! end"
}

// TODO: tests
// config.getFormat
