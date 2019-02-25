pub enum Command {
    ReloadConfig(Option<String>),
    ReloadTheme(Option<String>),
    Show(Selectors),
    Hide(Selectors),
    Focus(Selector),
    Help(HelpTopic),
}

#[derive(Debug)]
pub enum HelpTopic {
    Default,
    Show,
    Hide,
    Reload,
    Unknown(String),
}

pub struct Selectors(pub Vec<Selector>);

impl Selectors {
    pub fn contains_id(&self, name: &str) -> bool {
        self.0.contains(&Selector::Id(name.to_owned()))
    }
    pub fn _contains_class(&self, name: &str) -> bool {
        self.0.contains(&Selector::Class(name.to_owned()))
    }
}

#[derive(PartialEq)]
pub enum Selector {
    Class(String),
    Id(String),
}

impl Selector {
    pub fn get_name(&self) -> String {
        match self {
            Selector::Id(name) => name.to_string(),
            Selector::Class(name) => name.to_string(),
        }
    }
}
