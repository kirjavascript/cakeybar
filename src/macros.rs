/// used for cloning in closures

#[macro_export]
macro_rules! enclose {
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
