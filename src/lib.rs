use ansi_term;

use std::io;

mod lang;

trait CharEncoder: 'static {
    fn encode(iter: &mut dyn Iterator<Item = char>) -> Option<String>;
    fn wrap_in_quotes() -> bool;
}

trait Named {
    fn name() -> &'static str;
}

pub fn write_to<W: io::Write>(writer: &mut W, grapheme: &str) -> std::io::Result<()> {
    writeln!(writer)?;
    writeln!(writer, "{}", language_output(grapheme, lang::css::Css))?;
    writeln!(writer)?;
    writeln!(writer, "{}", language_output(grapheme, lang::html::Html))?;
    writeln!(writer)?;
    writeln!(writer, "{}", language_output(grapheme, lang::js::Js))?;

    Ok(())
}

fn language_output<T: CharEncoder + Named>(grapheme: &str, t: T) -> String {
    let grey = ansi_term::Colour::Black.bold();
    let escape = escape_grapheme(grapheme, t);
    let lang = grey.paint(format!("-- {}", T::name()));

    format!("  {:<10} {}", escape, lang)
}

fn escape_grapheme<T: CharEncoder>(grapheme: &str, _: T) -> String {
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
    use super::{escape_grapheme, language_output, CharEncoder, Named};

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

    #[test]
    fn test_output() {
        struct AlwaysHello;

        impl CharEncoder for AlwaysHello {
            fn encode(iter: &mut dyn Iterator<Item = char>) -> Option<String> {
                iter.next().map(|_| "hello".to_string())
            }
            fn wrap_in_quotes() -> bool {
                false
            }
        }

        impl Named for AlwaysHello {
            fn name() -> &'static str {
                "AlwaysHello"
            }
        }

        let grey = super::ansi_term::Colour::Black.bold();

        let actual = language_output("a", AlwaysHello);
        let expected = format!("  hello      {}", grey.paint("-- AlwaysHello"));
        assert_eq!(actual, expected);
    }
}
