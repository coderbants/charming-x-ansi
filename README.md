<p align="center">
  <picture>
    <source media="(prefers-color-scheme: light)" srcset="https://user-images.githubusercontent.com/25087/236529178-465e9b98-3401-47dd-8691-ea475d96c3ad.png" height="200" />
    <source media="(prefers-color-scheme: dark)" srcset="https://user-images.githubusercontent.com/25087/236529273-6f8c841f-f11b-4ec8-b01d-7e3d9b17c85f.png" height="200" />
    <img src="https://user-images.githubusercontent.com/25087/236529178-465e9b98-3401-47dd-8691-ea475d96c3ad.png" height="200" alt="A 3D rendering of an X"/>
  </picture><br>
  <a href="https://crates.io/crates/rusty-x-ansi"><img src="https://img.shields.io/crates/v/rusty-x-ansi.svg" alt="crates.io"></a>
    <a href="https://github.com/coderbants/rusty-x-ansi/actions"><img src="https://github.com/coderbants/rusty-x-ansi/actions/workflows/ci.yml/badge.svg" alt="Build Status"></a>
</p>

# Rusty X/ANSI (`rusty-x-ansi`)

**Rusty X/ANSI** is a complete, from-scratch Rust port of the [`ansi`](https://github.com/charmbracelet/x/tree/main/ansi) package from Charmbracelet's `x` monorepo — ANSI escape sequence parsing, SGR styling, width/wrap utilities and terminal queries. It tracks upstream on a rolling basis. **Version policy: the crate version and every release tag must equal the tracked upstream version exactly — never ahead, never behind** (enforced by `scripts/verify_upstream_version.sh` in CI and on every release). It shares the **1:1 parity** goals of the rest of the Rusty port family, favoring fidelity to upstream semantics over Rust-native rewrites.

It's part of the Rusty port family of the Bubble Tea ecosystem and underpins [rusty-ultraviolet](https://github.com/coderbants/rusty-ultraviolet), [rusty-lipgloss](https://github.com/coderbants/rusty-lipgloss), [rusty-colorprofile](https://github.com/coderbants/rusty-colorprofile), [rusty-bubbles](https://github.com/coderbants/rusty-bubbles) and [rusty-bubbletea](https://github.com/coderbants/rusty-bubbletea).

***About X/ANSI***

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
cargo add rusty-x-ansi
```

Cleanroom Rust port of the [`ansi`](https://github.com/charmbracelet/x/tree/main/ansi) package:
ANSI escape sequence parsing, SGR styling, width/wrap utilities and terminal queries.


## Usage

Style text with the ANSI SGR style builder, and parse escape sequences with
the incremental parser:

```rust
use rusty_x_ansi::style::{Color, Style};
use rusty_x_ansi::color::RGBColor;

// Build an SGR style and wrap a string in it.
let mut style = Style::default();
style.bold = true;
style.fg_color = Some(Color::RGB(RGBColor { r: 255, g: 0, b: 0 }));
println!("{}", style.styled("red and bold")); // [1;38;2;255;0;0mred and bold[m

// Parse an escape stream into sequences and printable runes.
let mut p = rusty_x_ansi::parser::new_parser();
let decoded = rusty_x_ansi::parser::decode_sequence(b"\x1b[31mred", 0, Some(&mut p));
assert_eq!(decoded.seq, b"\x1b[31m"); // the CSI sequence
assert_eq!(decoded.width, 3);          // width of the printable rune
```

The crate also provides width/wrap utilities (`rusty_x_ansi::width`,
`rusty_x_ansi::wrap`) and terminal queries (background/foreground color,
cursor color, terminal version) used throughout the Rusty port family.
