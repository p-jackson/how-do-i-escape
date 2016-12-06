extern crate docopt;
extern crate rustc_serialize;
extern crate entities;


mod css;
mod html;
mod js;


use docopt::Docopt;


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

    println!(r#"css  = "{}""#, escape_grapheme(&grapheme, css::as_css));
    println!(r#"html = {}"#, escape_grapheme(&grapheme, html::as_html));
    println!(r#"js   = "{}""#, escape_grapheme(&grapheme, js::as_js));
}


fn escape_grapheme<F>(grapheme: &str, int_to_escape_sequence: F) -> String
    where F: Fn(&mut Iterator<Item = u32>) -> Option<String>
{
    let mut result = String::new();
    let mut iter = grapheme.chars().map(|ch| ch as u32);

    loop {
        match int_to_escape_sequence(&mut iter) {
            Some(s) => result.push_str(&s),
            None => break,
        }
    }

    result

}


#[cfg(test)]
mod tests {
    use super::escape_grapheme;


    #[test]
    fn test_escape_grapheme() {
        let always_hello = escape_grapheme("\u{FF}",
                                           |iter| iter.next().map(|_| "hello".to_string()));
        assert_eq!(always_hello, "hello");

        let simple = escape_grapheme("\u{FF}", |iter| iter.next().map(|i| format!("{}", i)));
        assert_eq!(simple, "255");
    }
}
