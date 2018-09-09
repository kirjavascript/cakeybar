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
    Show,
    Hide,
    Reload,
    Unknown(String),
}

pub struct Selectors(pub Vec<Selector>);

impl Selectors {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

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
