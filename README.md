# how-do-i-escape

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
