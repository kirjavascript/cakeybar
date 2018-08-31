use nom::*;
use nom::types::CompleteStr as Input;

// TODO: drop debug

#[derive(Debug)]
pub enum Command {
    ReloadConfig(Option<String>),
    ReloadTheme(Option<String>),
    Show(Selectors),
    Hide(Selectors),
    Help(HelpTopic),
}

#[derive(Debug)]
pub enum HelpTopic {
    Default,
    // Show,
    // Hide,
    // Reload,
// add a reload/show help catchall / help show
// show extra message when path fails Some(theme)
// get path from where ipc started
}

#[derive(Debug)]
pub struct Selectors(
    pub Vec<Selector>
);

#[derive(Debug)]
pub enum Selector {
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
    do_parse!(
        name: many1!( alt!( alphanumeric1 | tag!("_") | tag!("-") )) >>
        (name.iter().map(|s| s.to_string()).collect::<String>())
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
        (Command::Show(Selectors(selectors)))
    )
);

named!(hide<Input,Command>,
    do_parse!(
        multispace0 >> tag!("hide") >>
        selectors: many1!( selector ) >>
        (Command::Hide(Selectors(selectors)))
    )
);

named!(get_command<Input,Command>,
    alt!( show | hide | help | reload_theme | reload_config )
);

pub fn parse_message(input: &str) {
    match get_command(Input(input)) {
        Ok((_remainder, command)) => {
            println!("{:#?}", command);
            println!("{}", command);
        },
        Err(err) => {
            error!("{:?}", err);
        },
    }
}
