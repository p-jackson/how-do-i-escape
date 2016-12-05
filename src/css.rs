pub fn as_css(i: u32) -> String {
    format!("\\{:01$X}", i, 4)
}


#[cfg(test)]
mod tests {
    use super::as_css;


    #[test]
    fn test_as_css() {
        assert_eq!(as_css(0), r"\0000");
        assert_eq!(as_css(0xFFFF), r"\FFFF");
        assert_ne!(as_css(0xBEEF), r"\beef");
        // assert_eq!(as_css(0x10000), ?);
    }
}
