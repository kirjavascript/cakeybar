use nom::types::CompleteStr as Input;
use nom::*;
use crate::wm::ipc::commands::*;

named!(selector<Input,Selector>,
    do_parse!(
        multispace1 >>
        type_: one_of!("#.") >>
        selector: selector_name  >>
        (if type_ == '#' {
            Selector::Id(selector)
        } else {
            Selector::Class(selector)
        })
    )
);

named!(selector_name<Input,String>,
    do_parse!(
        name: many1!( alt!( alphanumeric1 | is_a!("_-") )) >>
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

named!(reload_help<Input,Command>,
    do_parse!(
        multispace0 >> tag!("reload") >>
        (Command::Help(HelpTopic::Reload))
    )
);

named!(reload<Input,Command>,
    alt!( reload_theme | reload_config | reload_help )
);

named!(show<Input,Command>,
    do_parse!(
        multispace0 >> tag!("show") >>
        selectors_opt: opt!(many1!( selector )) >>
        (if let Some(selectors) = selectors_opt {
            Command::Show(Selectors(selectors))
        } else {
            Command::Help(HelpTopic::Show)
        })
    )
);

named!(hide<Input,Command>,
    do_parse!(
        multispace0 >> tag!("hide") >>
        selectors_opt: opt!(many1!( selector )) >>
        (if let Some(selectors) = selectors_opt {
            Command::Hide(Selectors(selectors))
        } else {
            Command::Help(HelpTopic::Hide)
        })
    )
);

// help

named!(help_default<Input,Command>,
    do_parse!(
        multispace0 >> tag!("help") >>
        (Command::Help(HelpTopic::Default))
    )
);

named!(help_topic<Input,Command>,
    do_parse!(
        multispace0 >> tag!("help") >>
        topic: get_rest_opt >>
        (Command::Help(
            if let Some(topic) = topic {
                match topic.as_str() {
                    "show" => HelpTopic::Show,
                    "hide" => HelpTopic::Hide,
                    "reload" => HelpTopic::Reload,
                    t if t.len() != 0 => HelpTopic::Unknown(t.to_string()),
                    _ => HelpTopic::Default,
                }
            } else { HelpTopic::Default }
        ))
    )
);

named!(help<Input,Command>,
    alt!( help_topic | help_default )
);

named!(get_command<Input,Command>,
    alt!( show | hide | reload | help )
);

pub fn parse_message(input: &str) -> Result<Command, String> {
    match get_command(Input(input)) {
        Ok((_remainder, command)) => Ok(command),
        Err(err) => Err(format!("{:?}", err)),
    }
}
