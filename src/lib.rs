#[macro_export]
macro_rules! my_if {
    ($if_bool:expr => $if_code:block $(elif $elif_bool:expr => $elif_code:block)* $(otherwise => $else_code:block)?) => {};
}
