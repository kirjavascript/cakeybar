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

macro_rules! message {
    ($c:ident, $m:expr, $p:expr) => {
        {
            use $crate::ansi_term::Colour::{$c, Fixed};
            let padding = String::from_utf8(vec![b' '; 12 - $m.len()]).unwrap();
            eprint!("{}{} {}", padding, $c.bold().paint($m), $p);
            let file_line = format!("{}:{}", file!(), line!());
            eprintln!(" {}", Fixed(240).paint(file_line));
        }
    };
}
#[macro_export]
macro_rules! error {
    ( $x:expr, $( $y:expr ),* ) => {
        message!(Red, "error", format!($x, $($y),*));
    };
    ( $x:expr ) => {
        message!(Red, "error", format!($x));
    };
}

#[macro_export]
macro_rules! warn {
    ( $x:expr, $( $y:expr ),* ) => {
        message!(Yellow, "warning", format!($x, $($y),*));
    };
    ( $x:expr ) => {
        message!(Yellow, "warning", format!($x));
    };
}

#[macro_export]
macro_rules! info {
    ( $x:expr, $( $y:expr ),* ) => {
        message!(Green, "info", format!($x, $($y),*));
    };
    ( $x:expr ) => {
        message!(Green, "info", format!($x));
    };
}
