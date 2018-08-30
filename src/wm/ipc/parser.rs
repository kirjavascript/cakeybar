use nom::*;
use nom::types::CompleteStr as Input;
use wm::ipc::help::HelpTopic;

#[derive(Debug)]
enum Command {
    ReloadConfig(Option<String>),
    ReloadTheme(Option<String>),
    Show(Vec<Selector>),
    Hide(Vec<Selector>),
    Help(HelpTopic),
}

#[derive(Debug)]
enum Selector {
    Class(String),
    Id(String),
}

named!(selector<Input,Selector>,
    do_parse!(
        multispace1 >>
        type_: alt!( tag!("#") | tag!(".")) >>
        selector: selector_name  >>
        (if type_ == Input("#") {
            Selector::Id(selector)
        } else {
            Selector::Class(selector)
        })
    )
);

named!(selector_name<Input,String>,
    map!(
        many1!( alt!( alphanumeric1 | tag!("_") | tag!("-") )),
        |s| s.iter().map(|s| s.to_string()).collect::<String>()
    )
);

named!(get_rest_opt<Input,Option<String>>,
    map!(
        tuple!(opt!(multispace1), take_while!(|_| true)),
        |(ms, s)| if ms.is_none() || s.len() == 0 {
            None
        } else {
            Some(s.to_string())
        }
    )
);

// commands

named!(reload_theme<Input,Command>,
    do_parse!(
        multispace0 >> tag!("reload") >>
        multispace1 >> tag!("theme") >>
        path: get_rest_opt >>
        (Command::ReloadTheme(path))
    )
);

named!(reload_config<Input,Command>,
    do_parse!(
        multispace0 >> tag!("reload") >>
        multispace1 >> tag!("config") >>
        path: get_rest_opt >>
        (Command::ReloadConfig(path))
    )
);

named!(help<Input,Command>,
    do_parse!(
        multispace0 >> tag!("help") >> multispace0 >>
        (Command::Help(HelpTopic::Default))
    )
);

named!(show<Input,Command>,
    do_parse!(
        multispace0 >> tag!("show") >>
        selectors: many1!( selector ) >>
        (Command::Show(selectors))
    )
);

named!(get_command<Input,Command>,
    alt!( show | help | reload_theme | reload_config )
);

    // if just the bar, show/hide the bar
    // if bar + other use bar as a selector

// add a reload/show help catchall / help show
// get a good readline
// fmt::Display

pub fn parse_message(input: &str) {
    println!("{:#?}", get_command(Input(input)));
}
