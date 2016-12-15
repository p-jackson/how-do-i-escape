pub fn as_css(iter: &mut Iterator<Item = char>) -> Option<String> {
    iter.next().map(|i| format!("\\{:01$X}", i as u32, 4))
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
        assert_eq!(as_css(&mut "\u{0}".chars()), expected1);

        let expected2 = Some(r"\005E".to_string());
        assert_eq!(as_css(&mut once('^')), expected2);

        let expected3 = Some(r"\210B".to_string());
        assert_eq!(as_css(&mut once('ℋ')), expected3);

        // "𝔄" is a single code pointer greater than FFFF
        let expected4 = Some(r"\1D504".to_string());
        assert_eq!(as_css(&mut once('𝔄')), expected4);
    }


    #[test]
    fn loop_without_crashing() {
        let v = vec!['a', 'b', 'c'];
        let mut iter = v.into_iter();

        while let Some(_) = as_css(&mut iter) {}
    }
}
