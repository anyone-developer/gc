#[cfg(debug_assertions)]
pub fn debug_print(msg: &str) -> bool {
    println!("{}", msg);
    true
}

#[cfg(not(debug_assertions))]
pub fn debug_print(_: &str) -> bool {
    false
}
