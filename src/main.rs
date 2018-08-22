extern crate ansi_term;
extern crate clap;
extern crate how_do_i_escape;

use how_do_i_escape::write_to;
use std::{io, str};

const HELP_TEMPLATE: &'static str = "
{bin}: {about}

USAGE:
    {usage}

OPTION:
{flags}
{after-help}";

fn main() {
    run().unwrap();
}

fn run() -> io::Result<()> {
    let matches = clap::App::new("how-do-i-escape")
        .about("Prints escape sequences for unicode graphemes")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(clap::Arg::with_name("grapheme").required(true))
        .template(HELP_TEMPLATE)
        .after_help(example_text()?.as_ref())
        .get_matches();

    // We marked the grapheme argument as required above, so unwrap
    let grapheme = matches.value_of("grapheme").unwrap();

    write_to(&mut io::stdout(), &grapheme)?;
    println!();

    Ok(())
}

fn example_text() -> io::Result<String> {
    let white = ansi_term::Colour::White.bold();

    let mut buffer = Vec::new();
    write_to(&mut buffer, "\u{00A7}").unwrap();

    let example_output =
        str::from_utf8(&buffer).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let text = format!(
        "
EXAMPLE:

{shell} how-do-i-escape \u{00A7}
{output}",
        shell = white.paint(">"),
        output = example_output
    );

    Ok(text)
}
