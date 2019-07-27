use gumdrop::{Options, ParsingStyle};
use std::env;

#[derive(Debug, Options)]
pub struct Args {
    help: bool,
    #[options(help = "Specify a config path", meta = "[FILE]")]
    config: Option<String>,
    #[options(help = "Watch config files and reload on changes")]
    watch: bool,
    #[options(help = "Send an IPC message", meta = "[MESSAGE]")]
    message: Option<String>,
    #[options(help = "Shows information about monitors")]
    monitors: bool,
    #[options(short = "D", no_long)]
    multi: bool,
}

pub fn get_args() -> Args {
    let args: Vec<String> = env::args().collect();

    match Args::parse_args(&args[1..], ParsingStyle::AllOptions) {
        Ok(args) => {
            if args.help_requested() {
                println!("{} - version {}", crate::NAME, crate::VERSION);
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
