extern crate docopt;
extern crate rustc_serialize;
extern crate entities;


mod css;
mod html;
mod js;


use docopt::Docopt;


trait CharEncoder: 'static {
    fn encode(&mut Iterator<Item = char>) -> Option<String>;
    fn wrap_in_quotes() -> bool;
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

    println!("");
    println!("  {:<10} -- css", escape_grapheme(&grapheme, css::Css));
    println!("");
    println!("  {:<10} -- html", escape_grapheme(&grapheme, html::Html));
    println!("");
    println!("  {:<10} -- javascript", escape_grapheme(&grapheme, js::Js));
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
    use super::{CharEncoder, escape_grapheme};


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
                false
            }
        }

        let simple = escape_grapheme("a", Simple);
        assert_eq!(simple, "97");
    }
}
