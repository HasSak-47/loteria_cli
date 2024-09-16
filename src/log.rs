
#[macro_export]
macro_rules! debug {
    ($opts: expr, $fmt: tt $(,$params: expr)*) => {
        if $opts.debug {
            println!($fmt $(,$params)*);
        }
    };
}

