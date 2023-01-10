#[macro_export]
macro_rules! my_if {
    ($if_bool:expr => $if_code:block $(elif $elif_bool:expr => $elif_code:block)* $(otherwise => $else_code:block)?) => {
        match $if_bool {
            true => $if_code,
            false => {my_if!(
                elif_chain_to_if_elif_chain!(
                    $(elif $elif_bool => $elif_code)*
                )
            );}
        }
    };
}

#[macro_export]
macro_rules! elif_chain_to_if_elif_chain {
    () => {};
    (elif $first_elif_bool:expr => $first_elif_code:block $(elif $elif_bool:expr => $elif_code:block)*) => {
        $first_elif_bool => $first_elif_code $(elif $elif_bool:expr => $elif_code:block)*
    };
}
