# Upstream Go File Mapping: `charming-x-ansi`

Target Upstream Tag: `github.com/charmbracelet/x` monorepo at module tag `ansi/v0.11.7`

This mapping accounts for **every** file of the upstream `charmbracelet/x` monorepo at the
`ansi/v0.11.7` tag. The ported module is `github.com/charmbracelet/x/ansi`, located in the
`ansi/` subdirectory of `upstream-go/` (gitignored). All other modules of the monorepo are
**out of scope** for this crate (see the Out-of-Scope section) but are still inventoried so
no file is silently skipped.

## Source Files (`ansi/*.go` -> `src/`)

| Upstream Go File | Rust Equivalent / Status | Notes / Description |
| :--- | :--- | :--- |
| `ansi/doc.go` | `src/lib.rs` | Package docs |
| `ansi/style.go` | `src/style.rs` | SGR `Style` builder (`String()`, `Styled()`, colors, underline styles) |
| `ansi/color.go` | `src/color.rs` | `BasicColor` (+ named `Black`..`BrightWhite` constants), `IndexedColor`, `RGBColor`, `Convert256`, `Convert16`, `ReadStyleColor`, `ansiHex` palette; deprecated `ExtendedColor`/`TrueColor` aliases |
| `ansi/util.go` | `src/util.rs` | `Strip`, `Cut`, `CutLeft`, `Truncate`, `TruncateLeft`, `StringWidth`, `XParseColor` |
| `ansi/width.go` | `src/width.rs` | `StringWidth`, `StringWidthWc`, `stringWidth` (parser-aware scan), `FirstGraphemeCluster`; v0.11.7 `PrintAction || Utf8State` fix applied |
| `ansi/wrap.go` | `src/wrap.rs` | `Wrap`, `Hardwrap` (grapheme + wc variants) |
| `ansi/truncate.go` | `src/util.rs` | Truncation helpers |
| `ansi/hyperlink.go` | `src/hyperlink.rs` | `SetHyperlink`, `ResetHyperlink` |
| `ansi/method.go` | `src/method.rs` | `WidthMethod` abstraction |
| `ansi/method_test.go` | `src/method.rs` (tests) | `Method.StringWidth` cases (ported at v0.11.7) |
| `ansi/sgr.go` | `src/sgr.rs` | `SelectGraphicRendition`/`SGR` + `Attr` constants (ported at v0.11.7) |
| `ansi/mouse.go` | `src/mouse.rs` | `MouseButton` + encoding, `EncodeMouseButton`, `MouseX10`, `MouseSgr` (ported at v0.11.7; the SGR/urxvt pixel *mode* constants live in `mode.go`) |
| `ansi/mode.go`, `mode_deprecated.go`, `modes.go` | `src/mode.rs` | Terminal mode constants and helpers (ported at v0.11.7) |
| `ansi/cursor.go` | `src/cursor.rs` | Cursor movement/position sequences (ported at v0.11.7; `SetCursorStyle` defined here) |
| `ansi/screen.go` | `src/screen.rs` | Screen clear/size sequences |
| `ansi/title.go` | `src/title.rs` | Window title sequences |
| `ansi/title_test.go` | `src/title.rs` (tests) | Title tests |
| `ansi/clipboard.go` | `src/clipboard.rs` | OSC52 clipboard sequences |
| `ansi/reset.go` | `src/reset.rs` | Reset sequences |
| `ansi/progress.go` | `src/progress.rs` | Terminal progress bar sequences (ported at v0.11.7) |
| `ansi/progress_test.go` | `src/progress.rs` (tests) | Progress tests (ported at v0.11.7) |
| `ansi/inband.go` | `src/inband.rs` | In-band signalling sequences |
| `ansi/graphics.go` | `src/graphics.rs` | Graphics (Sixel/kitty) sequences |
| `ansi/graphics_test.go` | `src/graphics.rs` (tests) | Graphics tests |
| `ansi/keypad.go` | `src/keypad.rs` | Keypad sequences |
| `ansi/cwd.go` | `src/cwd.rs` | Current working directory OSC 7 |
| `ansi/cwd_test.go` | `src/cwd.rs` (tests) | CWD tests |
| `ansi/notification.go` | `src/notification.rs` | Notification OSC 9 |
| `ansi/notification_test.go` | `src/notification.rs` (tests) | Notification tests |
| `ansi/palette.go` | `src/palette.rs` | Palette OSC 4/10/11/12 |
| `ansi/palette_test.go` | `src/palette.rs` (tests) | Palette tests |
| `ansi/status.go` | `src/status.rs` | Status line DECSC |
| `ansi/termcap.go` | `src/termcap.rs` | Termcap/terminfo queries |
| `ansi/xterm.go` | `src/xterm.rs` | XTerm queries |
| `ansi/urxvt.go` | `src/urxvt.rs` | urxvt sequences |
| `ansi/urxvt_test.go` | `src/urxvt.rs` (tests) | urxvt tests |
| `iterm2.go` | `src/iterm2.rs` | iTerm2 sequences |
| `ansi/kitty.go` | `src/kitty.rs` | Kitty keyboard protocol flags + sequences (ported at v0.11.7) |
| `ansi/kitty/decoder.go`, `ansi/kitty/encoder.go`, `ansi/kitty/graphics.go`, `ansi/kitty/options.go`, `ansi/kitty/writer.go` | `src/kitty.rs` | Kitty decoder/encoder/graphics (deferred; root `kitty.go` constants only) |
| `ansi/kitty/decoder_test.go`, `ansi/kitty/encoder_test.go`, `ansi/kitty/options_test.go`, `ansi/kitty/writer_test.go` | `src/kitty.rs` (tests) | Kitty module tests |
| `ansi/passthrough.go` | `src/passthrough.rs` | DCS passthrough |
| `ansi/passthrough_test.go` | `src/passthrough.rs` (tests) | Passthrough tests |
| `ansi/paste.go` | `src/paste.rs` | Bracketed paste |
| `ansi/background.go` | `src/background.rs` | Ported at v0.11.7: OSC 10/11/12 sequences + HexColor/XRGBColor/XRGBAColor | |
| `ansi/background_test.go` | `src/background.rs` (tests) | Background tests |
| `ansi/finalterm.go` | `src/finalterm.rs` | FinalTerm sequences |
| `ansi/focus.go` | `src/focus.rs` | Focus event sequences |
| `ansi/winop.go` | `src/winop.rs` | Window manipulation sequences |
| `ansi/ctrl.go` | `src/ctrl.rs` | Control function helpers |
| `ansi/c0.go`, `ansi/c1.go` | `src/parser.rs` | C0/C1 control-character constants (NUL..US, PAD..APC, SP/DEL) |
| `ansi/ansi.go`, `ascii.go` | `src/parser.rs` | `Execute` writer helper (ansi.go), SP/DEL constants (ascii.go) |
| `ansi/charset.go` | `src/ctrl.rs` | Character set selection (deferred) |
| `ansi/parser.go`, `ansi/parser_decode.go`, `ansi/parser_handler.go`, `ansi/parser_sync.go` | `src/parser.rs` | ANSI parser API: `Parser` state machine, `Handler`, `DecodeSequence`/`DecodeSequenceWc`, `GetParser`/`PutParser` (pool is a no-op perf optimization), `Cmd`/`Param`/`Params`/`Command`/`Parameter`, `FirstGraphemeCluster`, `Has*Prefix` helpers |
| `ansi/parser/const.go`, `ansi/parser/seq.go`, `ansi/parser/transition_table.go` | `src/parser.rs` | Parser states/actions, packed-param constants/shifts, generated VT500 transition table |
| `ansi/parser_test.go`, `ansi/parser_apc_test.go`, `ansi/parser_csi_test.go`, `ansi/parser_dcs_test.go`, `ansi/parser_decode_test.go`, `ansi/parser_esc_test.go`, `ansi/parser_osc_test.go` | `src/parser.rs` (tests) | Parser suite: 21 tests in module (state machine, decode vectors, OSC/DCS/CSI params, grapheme widths) |
| `ansi/parser_decode.go` | `src/parser.rs` | Sequence decoding (DecodeSequence/DecodeSequenceWc) |
| `ansi/parser_handler.go` | `src/parser.rs` | Parser handlers (Handler struct, Params/ToParams) |
| `ansi/parser_sync.go` | `src/parser.rs` | Pooled parsers (GetParser/PutParser) |
| `parser/const.go`, `parser/seq.go`, `parser/transition_table.go` | `src/parser.rs` | Parser tables/constants |
| `ansi/gen.go` | (generated) | Table generator script; documented |
| `ansi/fixtures/*` (UTF-8-demo.txt, demo.vte) | (test fixtures) | Parser test fixtures |
| `ansi/sixel/color.go`, `ansi/sixel/decoder.go`, `ansi/sixel/encoder.go`, `ansi/sixel/palette.go`, `ansi/sixel/palette_sort.go`, `ansi/sixel/raster.go`, `ansi/sixel/repeat.go` | `src/sixel.rs` | Sixel graphics (deferred) |
| `ansi/sixel/color_test.go`, `ansi/sixel/decoder_test.go`, `ansi/sixel/encoder_test.go`, `ansi/sixel/palette_test.go`, `ansi/sixel/raster_test.go`, `ansi/sixel/repeat_test.go`, `ansi/sixel/sixel_bench_test.go`, `ansi/sixel/sixel_test.go` | `src/sixel.rs` (tests) | Sixel tests (deferred) |
| `ansi/kitty/decoder.go`, `ansi/kitty/encoder.go`, `ansi/kitty/graphics.go`, `ansi/kitty/options.go`, `ansi/kitty/writer.go` | `src/kitty.rs` | Kitty graphics/decoder (deferred) |
| `ansi/kitty/decoder_test.go`, `ansi/kitty/encoder_test.go`, `ansi/kitty/options_test.go`, `ansi/kitty/writer_test.go` | `src/kitty.rs` (tests) | Kitty tests (deferred) |
| `ansi/iterm2.go`, `ansi/iterm2/file.go` | `src/iterm2.rs` | iTerm2 image protocol (deferred) |
| `ansi/iterm2/file_test.go`, `ansi/iterm2/iterm2_test.go` | `src/iterm2.rs` (tests) | iTerm2 tests (deferred) |

## Test Files (`ansi/*_test.go` -> `tests/` or module tests)

| Upstream Go Test File | Rust Equivalent / Status | Notes / Description |
| :--- | :--- | :--- |
| `ansi/style_test.go` | `src/style.rs` (tests) | SGR style tests incl. `TestNilColors` (ported at v0.11.7) |
| `ansi/method_test.go` | `src/method.rs` (tests) | `Method.StringWidth` test table (ported at v0.11.7) |
| `ansi/color_test.go` | `tests/color_test.rs` | Color conversion tests |
| `ansi/wrap_test.go` | `tests/wrap_test.rs` | Wrap tests |
| `ansi/truncate_test.go` | `tests/util_test.rs` | Truncation tests |
| `ansi/width_test.go` | `src/width.rs` (tests) | Width tests incl. new v0.11.7 `half width and ascii` case |
| `ansi/hyperlink_test.go` | `tests/hyperlink_test.rs` | Hyperlink tests |
| `ansi/mouse_test.go` | `src/mouse.rs` (tests) | Mouse button encoding + SGR sequence tests (ported at v0.11.7) |
| `ansi/mode_test.go` | `src/mode.rs` (tests) | Mode set/reset/request/report tests (ported at v0.11.7) |
| `ansi/sgr_test.go` | `src/sgr.rs` (tests) | SGR sequence tests (ported at v0.11.7) |
| `ansi/parser_test.go` + `parser_*_test.go` | `tests/parser_test.rs` | Parser suite (in progress) |
| `ansi/clipboard_test.go`, `cwd_test.go`, `title_test.go`, `notification_test.go`, `palette_test.go`, `progress_test.go`, `background_test.go`, `urxvt_test.go`, `passthrough_test.go`, `iterm2/*_test.go`, `kitty/*_test.go`, `sixel/*_test.go`, `graphics_test.go` | documented per module | Deferred with their modules |

## Out-of-Scope Monorepo Modules (not library dependencies)

The following modules of the `charmbracelet/x` monorepo are NOT in the lipgloss/bubbletea
dependency trees and are therefore not ported; they are inventoried here so no file is
skipped: `cellbuf`, `colors`, `conpty`, `editor`, `errors`, `etag`, `examples/*`, `exp/*`,
`input`, `json`, `mosaic`, `pony/*`, `powernap/*`, `scripts`, `sshkey`, `term`, `termios`,
`vcr`, `vt`, `vttest`, `wcwidth`, `windows`, `xpty`, `fixtures`, plus the monorepo root files
(`go.mod`, `go.sum`, `Taskfile.yaml`, `.golangci.yml`, `.goreleaser.yml`, `.gitattributes`,
`.gitignore`, `.editorconfig`).

## Documentation & Support Files

| Upstream File | Rust Equivalent / Status | Notes / Description |
| :--- | :--- | :--- |
| `ansi/LICENSE` | `LICENSE` | MIT License (matching upstream copyright) |
| `ansi/README.md` | `README.md` | Upstream README retained + port identification header |
| `ansi/go.mod` / `ansi/go.sum` | `Cargo.toml` | Dependency manifest |
| `.github/workflows/*` | `.github/workflows/publish.yml` | CI/CD -> Rust publish workflow |

## Dependency Versions (this pin)

| Go dependency | Version | Rust handling |
| --- | --- | --- |
| `bits-and-blooms/bitset` | v1.24.4 | Existing Rust crate or in-line |
| `clipperhouse/displaywidth` | v0.11.0 | `unicode-width` crate (+`cjk` feature for `RUNEWIDTH_EASTASIAN`) |
| `clipperhouse/uax29/v2` | v2.7.0 | `unicode-segmentation` crate |
| `lucasb-eyer/go-colorful` | v1.4.0 | In-line color math |
| `mattn/go-runewidth` | v0.0.23 | `unicode-width` crate |

## Multi-Version Note

The dependency tree requires additional `x/ansi` versions (v0.10.3 for colorprofile v0.3.3,
v0.11.6 for colorprofile v0.4.3). This crate now targets `ansi/v0.11.7` (the pin required by
lipgloss/bubbletea/ultraviolet@20260703) and is published under the pseudo-version
`0.0.0-20260703014108`. Earlier pins are produced by reverse diff-forwarding
(`git diff ansi/v0.11.7..ansi/v0.11.2` etc.) and published as separate crate versions. See
`/Users/jonny/Projects/charming/DEPENDENCY_PLAN.md` §6.

## Porting Status

| Module group | Status |
| --- | --- |
| `ansi/style.go`, `color.go`, `util.go`, `width.go`, `wrap.go`, `truncate.go`, `hyperlink.go`, `method.go`, `kitty.go` (flags) | Ported & Tested at v0.11.7 |
| `ansi/sgr.go`, `mouse.go`, `mode.go`, `mode_deprecated.go`, `modes.go`, `cursor.go`, `progress.go` | Ported & Tested at v0.11.7 (incl. `sgr_test.go`, `mouse_test.go`, `mode_test.go`, `progress_test.go` as inline tests) |
| `ansi/screen.go`, `title.go`, `reset.go`, `clipboard.go`, remaining sequences | Pending (documented above) |
| `parser*.go`, `kitty/*` (decoder/encoder/graphics), `sixel/*`, `iterm2/*`, remaining sequences | Pending (documented above); `parser_decode.go`/`parser_sync.go` v0.11.7 deltas noted, not yet ported |
