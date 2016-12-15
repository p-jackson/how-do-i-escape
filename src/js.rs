pub fn as_js(iter: &mut Iterator<Item = char>) -> Option<String> {
    iter.next().map(|ch| {
        let i = ch as u32;

        if i <= 0xFFFF {
            format!("\\u{:01$X}", i, 4)
        } else {
            format!("\\u{{{:X}}}", i)
        }
    })
}


#[cfg(test)]
mod tests {
    use super::as_js;
    use std::iter::{empty, once};


    #[test]
    fn empty_iterator() {
        assert_eq!(as_js(&mut empty()), None);
    }


    #[test]
    fn values() {
        let expected1 = Some(r"\u0000".to_string());
        assert_eq!(as_js(&mut "\u{0}".chars()), expected1);

        let expected2 = Some(r"\u005E".to_string());
        assert_eq!(as_js(&mut once('^')), expected2);

        let expected3 = Some(r"\u210B".to_string());
        assert_eq!(as_js(&mut once('ℋ')), expected3);

        // "𝔄" is a single code pointer greater than FFFF
        let expected4 = Some(r"\u{1D504}".to_string());
        assert_eq!(as_js(&mut once('𝔄')), expected4);
    }


    #[test]
    fn loop_without_crashing() {
        let v = vec!['a', 'b', 'c'];
        let mut iter = v.into_iter();

        while let Some(_) = as_js(&mut iter) {}
    }
}
