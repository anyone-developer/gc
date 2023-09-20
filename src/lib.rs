// pub mod config {
//     use toml::{map::Map, Value};

//     pub use crate::config::create_new_table;

//     pub fn gc_new_table() -> Map<std::string::String, Value> {
//         return create_new_table();
//     }
// }

pub mod app;
pub mod constants;
pub mod display;
pub mod global;
pub mod tool;

#[cfg(test)]
mod config_tests {

    #[test]
    fn gc_new_table() {
        // config::create_new_table();
    }
}
