#[macro_export]
macro_rules! my_if {
    ($bool:expr => $code:block) => {
        match $bool {
            true => {$code},
            false => {/* cond was false, ignoring input */},
            _ => panic!("expected bool, got {:#?}", stringify!($bool))
        }
    };
    
    ($bool:expr => $if_code:block otherwise: $else_code:block) => {
        match $bool {
            true => {$if_code},
            false => {$else_code},
            _ => panic!("expected bool, got {:#?}", stringify!($bool))
        }
    };
}

