use chrono::prelude::*;
use time::Duration;


/// 将北京时间转换为UTC 0时区时间
pub fn bcd_to_datetime(bt:&[u8])->DateTime<Utc> {
    let buf:Vec<u8>= bt.into_iter().map(|x| ((x >> 4 & 0x0fu8) * 10u8) + (x & 0x0fu8) ).collect();
    Utc.ymd(2000+(buf[0] as i32), buf[1] as u32 , buf[2] as u32).and_hms(buf[3].into(), buf[4].into(), buf[5].into())- Duration::seconds(3600 * 8)
    // let dt=Utc.ymd(2000+(buf[0] as i32), buf[1] as u32 , buf[2] as u32).and_hms(buf[3].into(), buf[4].into(), buf[5].into());//- Duration::seconds(3600 * 8)
    // dt.with_timezone(&FixedOffset::east(8*3600))
}