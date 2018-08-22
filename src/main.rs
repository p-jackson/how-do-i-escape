extern crate ansi_term;
extern crate clap;
extern crate entities;

mod css;
mod html;
mod js;

trait CharEncoder: 'static {
    fn encode(iter: &mut Iterator<Item = char>) -> Option<String>;
    fn wrap_in_quotes() -> bool;
}

trait Named {
    fn name() -> &'static str;
}

const HELP_TEMPLATE: &'static str = "
{bin}: {about}

Usage:
    {usage}

Options:
{flags}
{after-help}";

fn main() {
    let matches = clap::App::new("how-do-i-escape")
        .about("Prints escape sequences for unicode graphemes")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(clap::Arg::with_name("grapheme").required(true))
        .template(HELP_TEMPLATE)
        .after_help(example_text().as_ref())
        .get_matches();

    let grapheme = matches.value_of("grapheme").unwrap();

    println!();
    println!("{}", language_output(&grapheme, css::Css));
    println!();
    println!("{}", language_output(&grapheme, html::Html));
    println!();
    println!("{}", language_output(&grapheme, js::Js));
    println!();
}

fn language_output<T: CharEncoder + Named>(grapheme: &str, t: T) -> String {
    let grey = ansi_term::Colour::Black.bold();
    let escape = escape_grapheme(grapheme, t);
    let lang = grey.paint(format!("-- {}", T::name()));

    format!("  {:<10} {}", escape, lang)
}

fn example_text() -> String {
    let grey = ansi_term::Colour::Black.bold();

    format!(
        "
Example:
  $ how-do-i-escape \u{00A7}

    \"\\00A7\"    {css}

    &sect;     {html}

    \"\\u00A7\"   {js}
",
        css = grey.paint("-- css"),
        html = grey.paint("-- html"),
        js = grey.paint("-- javascript")
    )
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
            fn encode(iter: &mut Iterator<Item = char>) -> Option<String> {
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
            fn encode(iter: &mut Iterator<Item = char>) -> Option<String> {
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
            fn encode(iter: &mut Iterator<Item = char>) -> Option<String> {
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
