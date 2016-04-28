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

  let as_int = to_int(args.arg_grapheme);

  println!("css  = {}", as_css(as_int));
  println!("html = {}", as_html(as_int));
  println!("js   = {}", as_js(as_int));
}

fn to_int(grapheme: String) -> u32 {
  grapheme.chars().next().unwrap() as u32
}

fn as_css(i: u32) -> String {
  format!("\"\\{:01$X}\"", i, 4)
}

fn as_html(i: u32) -> String {
  format!("&#x{:01$X};", i, 4)
}

fn as_js(i: u32) -> String {
  format!("\"\\u{:01$X}\"", i, 4)
}
