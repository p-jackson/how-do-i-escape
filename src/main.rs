extern crate docopt;
extern crate rustc_serialize;

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
    arg_grapheme: String
}

fn main() {
  let args: Args = Docopt::new(USAGE)
    .and_then(|d| d.help(true).version(Some("v0.1.0".to_string())).decode())
    .unwrap_or_else(|e| e.exit());

  let grapheme = args.arg_grapheme;

  println!(r#"css  = "{}""#, escape_grapheme(&grapheme, as_css));
  println!(r#"html = {}"#, escape_grapheme(&grapheme, as_html));
  println!(r#"js   = "{}""#, escape_grapheme(&grapheme, as_js));
}

fn escape_grapheme<F>(grapheme: &str, int_to_escape_sequence: F) -> String
  where F: Fn(u32) -> String
{
  grapheme
    .chars()
    .map(|ch| ch as u32)
    .map(int_to_escape_sequence)
    .fold(String::new(), |c, s| {
      let mut copy = c;
      copy.push_str(&s);
      copy
    })
}

fn as_css(i: u32) -> String {
  format!("\\{:01$X}", i, 4)
}

fn as_html(i: u32) -> String {
  format!("&#x{:01$X};", i, 4)
}

fn as_js(i: u32) -> String {
  if i <= 0xFFFF {
    format!("\\u{:01$X}", i, 4)
  } else {
    format!("\\u{{{:X}}}", i)
  }
}

#[cfg(test)]
mod tests {
  use super::{as_css, as_html, as_js};

  #[test]
  fn test_as_css() {
    assert!(as_css(0) == r"\0000");
    assert!(as_css(0xFFFF) == r"\FFFF");
    assert!(as_css(0xBEEF) != r"\beef");
    //assert!(as_css(0x10000) == ?);
  }

  #[test]
  fn test_as_html() {
    assert!(as_html(0) == r"&#x0000;");
    assert!(as_html(0xFFFF) == r"&#xFFFF;");
    assert!(as_html(0xBEEF) != r"&#xbeef;");
    //assert!(as_html(0x10000) == ?);
  }

  #[test]
  fn test_as_js() {
    assert!(as_js(0) == r"\u0000");
    assert!(as_js(0xFFFF) == r"\uFFFF");
    assert!(as_js(0xBEEF) != r"\ubeef");
    assert!(as_js(0x10000) == r"\u{10000}");
    assert!(as_js(0x10FFFF) == r"\u{10FFFF}");
    assert!(as_js(0xFFFFFFFF) == r"\u{FFFFFFFF}");
  }
}
