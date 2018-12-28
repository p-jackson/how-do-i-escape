use escape_grapheme::{escape_grapheme, lang, CharEncoder, Named};
use std::io;

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

#[cfg(test)]
mod tests {
    use super::language_output;
    use escape_grapheme::{CharEncoder, Named};

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

        let grey = ansi_term::Colour::Black.bold();

        let actual = language_output("a", AlwaysHello);
        let expected = format!("  hello      {}", grey.paint("-- AlwaysHello"));
        assert_eq!(actual, expected);
    }
}
