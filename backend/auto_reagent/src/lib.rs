pub mod handlers;
pub mod models;
pub mod utility;
pub mod middleware;

#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            use chrono::prelude::*;
            print!("{}   ",chrono::Local::now().with_nanosecond(0).unwrap());
            println!($($arg)*);
        }
    }
}