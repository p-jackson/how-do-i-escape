pub fn as_css(iter: &mut Iterator<Item = u32>) -> Option<String> {
    iter.next().map(|i| format!("\\{:01$X}", i, 4))
}


#[cfg(test)]
mod tests {
    use super::as_css;
    use std::iter::{empty, once};


    #[test]
    fn empty_iterator() {
        assert_eq!(as_css(&mut empty()), None);
    }


    #[test]
    fn values() {
        let expected1 = Some(r"\0000".to_string());
        assert_eq!(as_css(&mut once(0)), expected1);

        let expected2 = Some(r"\FFFF".to_string());
        assert_eq!(as_css(&mut once(0xFFFF)), expected2);

        let expected3 = Some(r"\beef".to_string());
        assert_ne!(as_css(&mut once(0xBEEF)), expected3);

        // assert_eq!(as_css(once(0x10000)), ?);
    }


    #[test]
    fn loop_without_crashing() {
        let v = vec![0, 1, 2];
        let mut iter = v.into_iter();

        while let Some(_) = as_css(&mut iter) {}
    }
}
