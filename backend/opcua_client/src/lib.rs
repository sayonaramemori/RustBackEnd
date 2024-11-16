// #![feature(stmt_expr_attributes)]

pub mod entity;
pub mod example;
pub mod opcua_config;
pub mod utility;

// use AutoReagent::debug_println;

#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        // #[cfg(debug_assertions)]
        if cfg!(debug_assertions)
        {
            use chrono::prelude::*;
            print!("line: {} time: {}   ",line!(),chrono::Local::now().with_nanosecond(0).unwrap());
            println!($($arg)*);
        }
    }
}