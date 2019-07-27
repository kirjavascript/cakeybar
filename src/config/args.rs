use gumdrop::{Options, ParsingStyle};
use std::env;

#[derive(Debug, Options)]
pub struct Args {
    help: bool,
    #[options(help = "Specify a config path", meta = "[FILE]")]
    pub config: Option<String>,
    #[options(help = "Watch config files and reload on changes")]
    pub watch: bool,
    #[options(help = "Send an IPC message", meta = "[MESSAGE]")]
    pub message: Option<String>,
    #[options(help = "Shows information about monitors")]
    pub monitors: bool,
    #[options(short = "D", no_long)]
    pub multi: bool,
}

pub fn get_args() -> Args {
    let args: Vec<String> = env::args().collect();

    match Args::parse_args(&args[1..], ParsingStyle::AllOptions) {
        Ok(args) => {
            if args.help_requested() {
                println!("{} {}\n", crate::NAME, crate::VERSION);
                Args::parse_args_default_or_exit();
            }
            args
        },
        Err(err) => {
            error!("{}", err);
            std::process::exit(0);
        },
    }
}
