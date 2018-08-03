#[macro_use]
extern crate nom;

use nom::types::CompleteStr as Input;

#[derive(Debug, Eq, PartialEq)]
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
    let input = "hello {one} world { two} end";

    let output = format_symbols(input, |sym| {
        match sym {
            "one" => "ONE",
            "two" => "dongers!",
            _ => sym,
        }
    });

    println!("{:#?}", output); // "hello ONE world dongers! end"
}

#[cfg(test)]
mod tests {
    use format_symbols;
    use get_tokens;
    use Input;
    use Token;

    #[test]
    fn check_tokens() {
        let input = "text { int  } bork {q}allo";
        let tokens = get_tokens(Input(input));
        assert_eq!(tokens.unwrap().1, vec![
            Token::Text("text ".to_string()),
            Token::Symbol(" int  ".to_string()),
            Token::Text(" bork ".to_string()),
            Token::Symbol("q".to_string()),
            Token::Text("allo".to_string()),
        ]);
    }
    #[test]
    fn no_interpolation() {
        let input = "< { one } >";
        let output = format_symbols(input, |sym| sym);
        assert_eq!(output, "< one >");
    }
    #[test]
    fn partial_interpolation() {
        let input = "{one}{two}";
        let output = format_symbols(input, |sym| {
            match sym {
                "one" => "interp",
                _ => sym,
            }
        });
        assert_eq!(output, "interptwo");
    }
    #[test]
    fn multi_interpolation() {
        let input = "hello {one} world { two} end!\"";
        let output = format_symbols(input, |sym| {
            match sym {
                "one" => "ONE",
                "two" => "TWO",
                _ => sym,
            }
        });
        assert_eq!(output, "hello ONE world TWO end!\"");
    }
    #[test]
    fn weird_interpolation() {
        let input = "🔳🔊📣📢🔔🃏{🎴💬🆖} 🀄️♠️♣️♥️🆓➰";
        let output = format_symbols(input, |sym| {
            match sym {
                "🎴💬🆖" => "🤔",
                _ => sym,
            }
        });
        assert_eq!(output, "🔳🔊📣📢🔔🃏🤔 🀄️♠️♣️♥️🆓➰");
    }
}
