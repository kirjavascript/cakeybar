#[macro_export]
macro_rules! clone {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
    ( $x:ident $y:expr ) => {
        {
            let $x = $x.clone();
            $y
        }
    };
}

#[macro_export]
macro_rules! error {
    ( $x:expr, $( $y:expr ),* ) => {
        eprint!("       {} ", $crate::ansi_term::Colour::Red.bold().paint("error"));
        eprintln!($x, $($y),*);
    };
    ( $x:expr ) => {
        eprint!("       {} ", $crate::ansi_term::Colour::Red.bold().paint("error"));
        eprintln!($x);
    };
}

#[macro_export]
macro_rules! warn {
    ( $x:expr, $( $y:expr ),* ) => {
        eprint!("     {} ", $crate::ansi_term::Colour::Yellow.bold().paint("warning"));
        eprintln!($x, $($y),*);
    };
    ( $x:expr ) => {
        eprint!("     {} ", $crate::ansi_term::Colour::Yellow.bold().paint("warning"));
        eprintln!($x);
    };
}

#[macro_export]
macro_rules! info {
    ( $x:expr, $( $y:expr ),* ) => {
        print!("        {} ", $crate::ansi_term::Colour::Cyan.bold().paint("info"));
        println!($x, $($y),*);
    };
    ( $x:expr ) => {
        print!("        {} ", $crate::ansi_term::Colour::Cyan.bold().paint("info"));
        println!($x);
    };
}
