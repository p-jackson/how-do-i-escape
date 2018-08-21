extern crate ansi_term;
extern crate docopt;
extern crate entities;
extern crate rustc_serialize;

mod css;
mod html;
mod js;

use docopt::Docopt;

trait CharEncoder: 'static {
    fn encode(iter: &mut Iterator<Item = char>) -> Option<String>;
    fn wrap_in_quotes() -> bool;
}

trait Named {
    fn name() -> &'static str;
}

const USAGE: &'static str = "
how-do-i-escape: Prints escape sequences for unicode graphemes

Usage:
  how-do-i-escape <grapheme>
  how-do-i-escape (--help | --version)

Options:
  -h, --help  Show this screen.
  --version   Show version.

Example:
  $ how-do-i-escape \u{00A7}
  css  = \"\\00A7\"
  html = &sect;
  js   = \"\\u00A7\"
";

#[derive(RustcDecodable)]
struct Args {
    arg_grapheme: String,
}

fn main() {
    let version = format!("v{}", env!("CARGO_PKG_VERSION"));

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.help(true).version(Some(version)).decode())
        .unwrap_or_else(|e| e.exit());

    let grapheme = args.arg_grapheme;

    println!();
    println!("{}", language_output(&grapheme, css::Css));
    println!();
    println!("{}", language_output(&grapheme, html::Html));
    println!();
    println!("{}", language_output(&grapheme, js::Js));
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
