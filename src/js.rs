pub fn as_js(i: u32) -> String {
    if i <= 0xFFFF {
        format!("\\u{:01$X}", i, 4)
    } else {
        format!("\\u{{{:X}}}", i)
    }
}


#[cfg(test)]
mod tests {
    use super::as_js;


    #[test]
    fn test_as_js() {
        assert_eq!(as_js(0), r"\u0000");
        assert_eq!(as_js(0xFFFF), r"\uFFFF");
        assert_ne!(as_js(0xBEEF), r"\ubeef");
        assert_eq!(as_js(0x10000), r"\u{10000}");
        assert_eq!(as_js(0x10FFFF), r"\u{10FFFF}");
        assert_eq!(as_js(0xFFFFFFFF), r"\u{FFFFFFFF}");
    }
}
