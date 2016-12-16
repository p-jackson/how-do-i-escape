pub struct Js;


impl super::CharEncoder for Js {
    fn encode(iter: &mut Iterator<Item = char>) -> Option<String> {
        iter.next().map(|ch| {
            let i = ch as u32;

            if i <= 0xFFFF {
                format!("\\u{:01$X}", i, 4)
            } else {
                format!("\\u{{{:X}}}", i)
            }
        })
    }

    fn wrap_in_quotes() -> bool {
        true
    }
}


#[cfg(test)]
mod tests {
    use super::Js;
    use super::super::CharEncoder;
    use std::iter::{empty, once};


    #[test]
    fn empty_iterator() {
        assert_eq!(Js::encode(&mut empty()), None);
    }


    #[test]
    fn values() {
        let expected1 = Some(r"\u0000".to_string());
        assert_eq!(Js::encode(&mut "\u{0}".chars()), expected1);

        let expected2 = Some(r"\u005E".to_string());
        assert_eq!(Js::encode(&mut once('^')), expected2);

        let expected3 = Some(r"\u210B".to_string());
        assert_eq!(Js::encode(&mut once('ℋ')), expected3);

        // "𝔄" is a single code pointer greater than FFFF
        let expected4 = Some(r"\u{1D504}".to_string());
        assert_eq!(Js::encode(&mut once('𝔄')), expected4);
    }


    #[test]
    fn loop_without_crashing() {
        let v = vec!['a', 'b', 'c'];
        let mut iter = v.into_iter();

        while let Some(_) = Js::encode(&mut iter) {}
    }


    #[test]
    fn quotes() {
        assert!(Js::wrap_in_quotes());
    }
}
