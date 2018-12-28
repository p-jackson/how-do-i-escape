pub mod lang;

pub trait CharEncoder: 'static {
    fn encode(iter: &mut dyn Iterator<Item = char>) -> Option<String>;
    fn wrap_in_quotes() -> bool;
}

pub trait Named {
    fn name() -> &'static str;
}

pub fn escape_grapheme<T: CharEncoder>(grapheme: &str, _: T) -> String {
    let mut result = String::new();
    let mut iter = grapheme.chars();

    loop {
        match T::encode(&mut iter) {
            Some(s) => result.push_str(&s),
            None => break,
        }
    }

    if T::wrap_in_quotes() {
        format!(r#""{}""#, result)
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::{escape_grapheme, CharEncoder};

    #[test]
    fn test_escape_grapheme() {
        struct AlwaysHello;

        impl CharEncoder for AlwaysHello {
            fn encode(iter: &mut dyn Iterator<Item = char>) -> Option<String> {
                iter.next().map(|_| "hello".to_string())
            }
            fn wrap_in_quotes() -> bool {
                false
            }
        }

        let always_hello = escape_grapheme("a", AlwaysHello);
        assert_eq!(always_hello, "hello");

        struct Simple;

        impl CharEncoder for Simple {
            fn encode(iter: &mut dyn Iterator<Item = char>) -> Option<String> {
                iter.next().map(|i| format!("{}", i as u32))
            }
            fn wrap_in_quotes() -> bool {
                true
            }
        }

        let simple = escape_grapheme("a", Simple);
        assert_eq!(simple, r#""97""#);
    }
}
