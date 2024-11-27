pub mod model;
pub mod utility;
pub mod middleware;
pub mod interface;
pub mod service;
pub mod router;

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
