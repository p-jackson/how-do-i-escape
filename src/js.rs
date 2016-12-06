pub fn as_js(iter: &mut Iterator<Item = u32>) -> Option<String> {
    iter.next().map(|i| {
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
        assert_eq!(as_js(&mut once(0)), expected1);

        let expected2 = Some(r"\uFFFF".to_string());
        assert_eq!(as_js(&mut once(0xFFFF)), expected2);

        let expected3 = Some(r"\ubeef".to_string());
        assert_ne!(as_js(&mut once(0xBEEF)), expected3);

        let expected4 = Some(r"\u{10000}".to_string());
        assert_eq!(as_js(&mut once(0x10000)), expected4);

        let expected5 = Some(r"\u{10FFFF}".to_string());
        assert_eq!(as_js(&mut once(0x10FFFF)), expected5);

        let expected6 = Some(r"\u{FFFFFFFF}".to_string());
        assert_eq!(as_js(&mut once(0xFFFFFFFF)), expected6);
    }


    #[test]
    fn loop_without_crashing() {
        let v = vec![0, 1, 2];
        let mut iter = v.into_iter();

        while let Some(_) = as_js(&mut iter) {}
    }
}
