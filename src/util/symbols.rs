use nom::types::CompleteStr as Input;

#[derive(Debug, Eq, PartialEq)]
enum Token {
    Text(String),
    Symbol(String),
}

#[derive(Debug)]
pub struct SymbolFmt {
    tokens: Vec<Token>,
}

named!(escaped<Input,Token>,
   map!(
       alt!( tag!("{{") | tag!("}}") ),
       |s| Token::Text(s[..1].to_string())
   )
);

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
    many0!( alt!( escaped | symbol | text ) )
);

impl SymbolFmt {
    pub fn new(input: &str) -> Self {
        match get_tokens(Input(input)) {
            Ok((_, tokens)) => {
                Self { tokens }
            },
            Err(err) => {
                warn!("format: {}", err);
                Self { tokens: vec![] }
            },
        }
    }
    pub fn format<F>(&self, callback: F) -> String where F: Fn(&str) -> String {
        self.tokens
            .iter()
            .map(|tok| {
                match tok {
                    Token::Text(txt) => txt.to_string(),
                    Token::Symbol(sym) => callback(&sym.trim()),
                }
            })
            .collect::<Vec<String>>()
            .concat()
            .trim()
            .to_string()
    }
    pub fn contains(&self, item: &str) -> bool {
        self.tokens.contains(&Token::Symbol(item.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let output = SymbolFmt::new(input).format(|sym| sym.to_string());
        assert_eq!(output, "< one >");
    }
    #[test]
    fn partial_interpolation() {
        let input = "{one}{two}";
        let output = SymbolFmt::new(input).format(|sym| {
            match sym {
                "one" => "interp".to_string(),
                _ => sym.to_string(),
            }
        });
        assert_eq!(output, "interptwo");
    }
    #[test]
    fn multi_interpolation() {
        let input = "hello {one} world { two} end!\"";
        let output = SymbolFmt::new(input).format(|sym| {
            match sym {
                "one" => "ONE".to_string(),
                "two" => "TWO".to_string(),
                _ => sym.to_string(),
            }
        });
        assert_eq!(output, "hello ONE world TWO end!\"");
    }
    #[test]
    fn weird_interpolation() {
        let input = "🔳🔊📣📢🔔🃏{🎴💬🆖} 🀄️♠️♣️♥️🆓➰";
        let output = SymbolFmt::new(input).format(|sym| {
            match sym {
                "🎴💬🆖" => "🤔".to_string(),
                _ => sym.to_string(),
            }
        });
        assert_eq!(output, "🔳🔊📣📢🔔🃏🤔 🀄️♠️♣️♥️🆓➰");
    }
    #[test]
    fn contains() {
        let input = " sdasd {has} {bleh} asda";
        let symbols = SymbolFmt::new(input);
        assert!(symbols.contains("has"));
        assert!(!symbols.contains("poop"));
    }
}
