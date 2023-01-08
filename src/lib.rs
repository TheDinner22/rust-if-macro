#[macro_export]
macro_rules! my_if {
    ($bool:expr => $code:block) => {
        match $bool {
            true => {$code},
            false => {/* cond was false, ignoring input */},
            _ => panic!("expected bool, got {:#?}", stringify!($bool))
        }
    };
}

