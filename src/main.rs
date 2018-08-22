extern crate ansi_term;
extern crate clap;
extern crate entities;
extern crate how_do_i_escape;

use how_do_i_escape::{lang, language_output};

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
    println!("{}", language_output(&grapheme, lang::css::Css));
    println!();
    println!("{}", language_output(&grapheme, lang::html::Html));
    println!();
    println!("{}", language_output(&grapheme, lang::js::Js));
    println!();
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
