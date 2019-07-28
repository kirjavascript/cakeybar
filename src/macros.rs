// clone

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

// messages

macro_rules! message {
    ($c:ident, $m:expr, $p:expr) => {{
        let padding = String::from_utf8(vec![b' '; 7 - $m.len()]).unwrap();
        let file_line = format!("{}:{}", file!(), line!());
        if *$crate::config::NO_COLOR {
            eprintln!(" {}{} {} {}", $m, padding, $p, file_line);
        } else {
            use ansi_term::Colour::{$c, Fixed};
            eprint!(" {}{} {}", $c.bold().paint($m), padding, $p);
            eprintln!(" {}", Fixed(240).paint(file_line));
        }
    }};
}

#[macro_export]
macro_rules! error {
    ( $x:expr, $( $y:expr ),* $(,)? ) => {
        message!(Red, "error", format!($x, $($y),*));
    };
    ( $x:expr ) => {
        message!(Red, "error", format!($x));
    };
}

#[macro_export]
macro_rules! warn {
    ( $x:expr, $( $y:expr ),* $(,)? ) => {
        message!(Yellow, "warning", format!($x, $($y),*));
    };
    ( $x:expr ) => {
        message!(Yellow, "warning", format!($x));
    };
}

#[macro_export]
macro_rules! info {
    ( $x:expr, $( $y:expr ),* $(,)? ) => {
        message!(Green, "info", format!($x, $($y),*));
    };
    ( $x:expr ) => {
        message!(Green, "info", format!($x));
    };
}
