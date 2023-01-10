#[macro_export]
macro_rules! my_if {
    () => {};
    (elif $if_bool:expr => $if_code:block $(elif $elif_bool:expr => $elif_code:block)* $(otherwise => $else_code:block)?) => {
        match $if_bool {
            true => $if_code,
            false => {my_if!($(elif $elif_bool => $elif_code)* $(otherwise => $else_code)?);}
        }
    };
    (otherwise => $else_code:block) => {
        $else_code
    }
}
