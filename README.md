# In progress.....

I need your HELP!

Recently I found that there isn't any markdown parser which can meet my needs in rust, because of this, I decide to make a markdown parser by my own.

It's fairly hardcore.

## Modules

- `parser`
The module provides basic `Parser` trait and some useful combinators to help me to build parser components.

Actually, it's just the front of `standard` module. But I do not want to use any third-party parser library there anymore, so I must fill the blanks by myself.

- `standard`
The module filled the **standard semantics** for Markdown, such as *Header*, *Italic*, *Bold*, *Reference* and *Link*, so on.

## Design

Well known, there is a library named `parsec` in **Haskell**, it uses the **Parser Combinators** for the first time, and here's another mainstream idea called **Parser Generators** that I want to use it.

For the two uses, I prefer to use **Parser Combinators**, because I love compositable and flexible design better.

## License
GPL-3.0.
