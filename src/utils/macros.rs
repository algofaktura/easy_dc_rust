#[macro_export]
macro_rules! create_array {
    ($order:expr) => {{
        let mut array = [[0_i32; 3]; $order];
        array
    }};
}
