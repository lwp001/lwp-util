use encoding_rs;
use std::fmt::Write;
/// get gbk bytes
pub fn gbk_bytes(s:&str)->Vec<u8>{
    encoding_rs::GBK.encode(s).0.to_vec()
}
/// get gbk string
pub fn gbk_string(buf: &[u8]) -> String {
    let encoder = encoding_rs::GBK;
    if let Some(index) = buf.iter().position(|b| *b==0) {
        return encoder.decode_without_bom_handling(&buf[..index]).0.to_string();
    }
    return encoder.decode_without_bom_handling(buf).0.to_string();
}

/// get utf8 bytes
pub fn utf8_string(buf:&[u8])-> String {
    let end_offset = buf.iter().position(|b| *b == 0);
    if let Some(index) = end_offset {
         return String::from_utf8(buf[..index].to_vec()).unwrap_or("".to_string());
    }
    String::from_utf8(buf.to_vec()).unwrap_or("".to_string())
}
/// bcd sim to string
pub fn bcd_sim(buf: &[u8])-> String {
    let mut zero=false;
    let mut s=String::new();
    for b in buf {
        if zero {
           let _ = write!(s,"{:02X}",b);
        } else {
            if *b==0u8 {
                continue;
            }
           let _ = write!(s,"{:X}",b);
            zero = true;
        }
    }
    s
}