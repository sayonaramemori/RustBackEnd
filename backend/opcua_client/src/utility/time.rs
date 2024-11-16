use lazy_static::lazy_static;
use chrono::prelude::*;

pub fn with_timezone(time:DateTime<Utc>)->DateTime<FixedOffset> {
    return time.with_timezone(&get_timezone());
}

fn get_timezone()->FixedOffset{
    lazy_static! {
        static ref zone: FixedOffset = FixedOffset::east_opt(8*3600).unwrap();
    }
    return *zone;
}