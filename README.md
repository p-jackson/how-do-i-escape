# how-do-i-escape [![Build Status](https://travis-ci.org/p-jackson/how-do-i-escape.svg?branch=master)](https://travis-ci.org/p-jackson/how-do-i-escape) [![Build status](https://ci.appveyor.com/api/projects/status/tlof6gpgu837vvx7?svg=true)](https://ci.appveyor.com/project/p-jackson/how-do-i-escape)

> For when you ask yourself "how do I escape that"?

Prints escape sequences for unicode graphemes

It seems like every other day I'm looking up what the escape sequence for a
unicode grapheme is in various web languages. And I'm also trying to learn Rust.
So I wrote a tool in Rust.

## Install & Use

```
$ cargo install how-do-i-escape
$ how-do-i-escape §
css  = "\00A7"
html = &sect;
js   = "\u00A7"
```

## What I'm thinking for 1.0

* Prefer HTML entities if one exists
* Improve output readability (maybe with colour)
* Try and reduce the time it takes to run `cargo install`
* Run against *stable*, *beta* and *nightly* Rust
* Read spec to figure out how graphemes > FFFF are supposed to work
