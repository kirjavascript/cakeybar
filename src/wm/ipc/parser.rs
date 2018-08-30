use nom::*;
use nom::types::CompleteStr as Input;

#[derive(Debug)]
enum Command {
    ReloadConfig(Option<String>),
    ReloadTheme(Option<String>),
    Show,
    Hide,
    Error(String),
    Help,
}

named!(get_path<Input,Option<String>>,
    map!(
        tuple!(opt!(multispace1), take_while!(|_| true), opt!(multispace1)),
        |(ms, s, _)| if ms.is_none() || s.len() == 0 {
            None
        } else {
            Some(s.to_string())
        }
    )
);

named!(reload_theme<Input,Command>,
    do_parse!(
        multispace0 >> tag!("reload") >>
        multispace1 >> tag!("theme") >>
        path: get_path >>
        (Command::ReloadTheme(path))
    )
);

named!(reload_config<Input,Command>,
    do_parse!(
        multispace0 >> tag!("reload") >>
        multispace1 >> tag!("config") >>
        path: get_path >>
        (Command::ReloadConfig(path))
    )
);

named!(help<Input,Command>,
    do_parse!(
        multispace0 >> tag!("help") >> multispace0 >>
        (Command::Help)
    )
);

named!(get_command<Input,Command>,
    alt!( help | reload_theme | reload_config )
);


// add a reload help
// get a good readline
//
// optional
//
// show ...list incl bar...

// get_command / do_parse
// fmt::Display

// enum of commands
//
pub fn parse_message(input: &str) {
    println!("{:#?}", get_command(Input(input)));
}
