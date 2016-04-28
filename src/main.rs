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

  println!("css  = {}", as_css(&args.arg_grapheme));
  println!("html = {}", as_html(&args.arg_grapheme));
  println!("js   = {}", as_js(&args.arg_grapheme));
}

fn as_css(_: &str) -> String {
  "\"\\00A7\"".to_owned()
}

fn as_html(_: &str) -> String {
  "&sect;".to_owned()
}

fn as_js(_: &str) -> String {
  "\"\\u00A7\"".to_owned()
}
