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

named!(reload<Input,Command>,
    alt!( reload_theme | reload_config )
);

named!(focus<Input,Command>,
    do_parse!(
        multispace0 >> tag!("focus") >>
        selector: selector >>
        (Command::Focus(selector))
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
    alt!( show | hide | focus | reload )
);

pub fn parse_message(input: &str) -> Result<Command, String> {
    match get_command(Input(input)) {
        Ok((_remainder, command)) => Ok(command),
        Err(err) => Err(format!("{:?}", err)),
    }
}
