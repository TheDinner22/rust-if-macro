#[macro_export]
macro_rules! my_if {
    ($bool:expr => $code:block) => {
        println!("{:#?}", $bool);
    };
}
