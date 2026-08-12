<p>
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="https://user-images.githubusercontent.com/25087/236529178-465e9b98-3401-47dd-8691-ea475d96c3ad.png" height="200" />
    <source media="(prefers-color-scheme: dark)" srcset="https://user-images.githubusercontent.com/25087/236529273-6f8c841f-f11b-4ec8-b01d-7e3d9b17c85f.png" height="200" />
    <img src="https://user-images.githubusercontent.com/25087/236529178-465e9b98-3401-47dd-8691-ea475d96c3ad.png" height="200" alt="A 3D rendering of an X"/>
  </picture><br>
  <a href="https://crates.io/crates/charming-x-ansi"><img src="https://img.shields.io/crates/v/charming-x-ansi.svg" alt="crates.io"></a>
    <a href="https://github.com/coderbants/charming-x-ansi/actions"><img src="https://github.com/coderbants/charming-x-ansi/actions/workflows/ci.yml/badge.svg" alt="Build Status"></a>
</p>

# Charming X/ANSI (`charming-x-ansi`)

**Charming X/ANSI** is a complete, from-scratch Rust port of the [`ansi`](https://github.com/charmbracelet/x/tree/main/ansi) package from Charmbracelet's `x` monorepo — ANSI escape sequence parsing, SGR styling, width/wrap utilities and terminal queries. It tracks upstream on a rolling basis (this crate mirrors the upstream pseudo-version pin `20260703014108`) with the same **1:1 parity** goals as the rest of the Charming port family, favoring fidelity to upstream semantics over Rust-native rewrites.

It's part of the Charming port family of the Bubble Tea ecosystem and underpins [charming-ultraviolet](https://github.com/coderbants/charming-ultraviolet), [charming-lipgloss](https://github.com/coderbants/charming-lipgloss), [charming-colorprofile](https://github.com/coderbants/charming-colorprofile), [charming-bubbles](https://github.com/coderbants/charming-bubbles) and [charming-bubbletea](https://github.com/coderbants/charming-bubbletea).

This repository contains experimental packages with no promises of
backwards compatibility. Once they mature here, they might be moved
into other repositories.

Currently the following packages are available:

- [`ansi`](./ansi): ANSI escape sequence parser and definitions • [Docs](https://pkg.go.dev/github.com/charmbracelet/x/ansi)
- [`cellbuf`](./cellbuf): Cell-based terminal display parser • [Docs](https://pkg.go.dev/github.com/charmbracelet/x/cellbuf)
- [`conpty`](./conpty): Windows Console Pseudo-terminal library • [Docs](https://pkg.go.dev/github.com/charmbracelet/x/conpty)
- [`editor`](./editor): open files in text editors • [Docs](https://pkg.go.dev/github.com/charmbracelet/x/editor)
- [`errors`](./errors): `errors.Join` in older Go versions • [Docs](https://pkg.go.dev/github.com/charmbracelet/x/errors)
- [`golden`](./exp/golden): verify golden file equality • [Docs](https://pkg.go.dev/github.com/charmbracelet/x/exp/golden)
- [`higherorder`](./exp/higherorder): generic higher order functions • [Docs](https://pkg.go.dev/github.com/charmbracelet/x/exp/higherorder)
- [`input`](./input): terminal event input handler and driver • [Docs](https://pkg.go.dev/github.com/charmbracelet/x/input)
- [`json`](./json): JSON parsing using generics • [Docs](https://pkg.go.dev/github.com/charmbracelet/x/json)
- [`maps`](./exp/maps): generic maps utilities
- [`open`](./exp/open): open a file/URL using `open`, `xdg-open`, etc • [Docs](https://pkg.go.dev/github.com/charmbracelet/x/exp/open)
- [`ordered`](./exp/ordered): generic `min`, `max`, and `clamp` functions for ordered types • [Docs](https://pkg.go.dev/github.com/charmbracelet/x/exp/ordered)
- [`slice`](./exp/slice): generic slice utilities • [Docs](https://pkg.go.dev/github.com/charmbracelet/x/exp/slice)
- [`sshkey`](./sshkey): open and parse SSH keys, asks for passphrases when needed • [Docs](https://pkg.go.dev/github.com/charmbracelet/x/sshkey)
- [`strings`](./exp/strings): utilities for working with strings • [Docs](https://pkg.go.dev/github.com/charmbracelet/x/exp/strings)
- [`teatest`](./exp/teatest): a library for testing [Bubble Tea](https://github.com/charmbracelet/bubbletea) programs • [Docs](https://pkg.go.dev/github.com/charmbracelet/x/exp/teatest)
- [`term`](./term): terminal utilities and helpers • [Docs](https://pkg.go.dev/github.com/charmbracelet/x/term)
- [`termios`](./termios): Termios unified API and library • [Docs](https://pkg.go.dev/github.com/charmbracelet/x/termios)
- [`wcwidth`](./wcwidth): Wide character width calculation • [Docs](https://pkg.go.dev/github.com/charmbracelet/x/wcwidth)
- [`windows`](./windows): Windows API used at Charmbracelet • [Docs](https://pkg.go.dev/github.com/charmbracelet/x/windows)
- [`xpty`](./xpty): cross-platform PTY interface • [Docs](https://pkg.go.dev/github.com/charmbracelet/x/xpty)

[docbadge]: https://godoc.org/github.com/golang/gddo?status.svg

## Feedback

We'd love to hear your thoughts on this project. Feel free to drop us a note!

- [Twitter](https://twitter.com/charmcli)
- [The Fediverse](https://mastodon.social/@charmcli)
- [Discord](https://charm.sh/chat)

## License

[MIT](https://github.com/charmbracelet/x/raw/main/LICENSE)


## Installation

```sh
cargo add charming-x-ansi
```

Cleanroom Rust port of the [`ansi`](https://github.com/charmbracelet/x/tree/main/ansi) package:
ANSI escape sequence parsing, SGR styling, width/wrap utilities and terminal queries.


## Usage

Style text with the ANSI SGR style builder, and parse escape sequences with
the incremental parser:

```rust
use charming_x_ansi::style::{Color, Style};
use charming_x_ansi::color::RGBColor;

// Build an SGR style and wrap a string in it.
let mut style = Style::default();
style.bold = true;
style.fg_color = Some(Color::RGB(RGBColor { r: 255, g: 0, b: 0 }));
println!("{}", style.styled("red and bold")); // [1;38;2;255;0;0mred and bold[m

// Parse an escape stream into sequences and printable runes.
let mut p = charming_x_ansi::parser::new_parser();
let decoded = charming_x_ansi::parser::decode_sequence(b"\x1b[31mred", 0, Some(&mut p));
assert_eq!(decoded.seq, b"\x1b[31m"); // the CSI sequence
assert_eq!(decoded.width, 3);          // width of the printable rune
```

The crate also provides width/wrap utilities (`charming_x_ansi::width`,
`charming_x_ansi::wrap`) and terminal queries (background/foreground color,
cursor color, terminal version) used throughout the Charming port family.
