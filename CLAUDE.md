# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

Task runner is `just` (see `justfile`); `cargo` works directly too.

- Build: `just build-debug` / `just build-release` (sets `POLKIT_AGENT_HELPER_1`, which is read at compile time — plain `cargo build` falls back to the default path)
- Lint: `just check` (clippy `--all-features` with `-W clippy::pedantic`)
- Install: `just install` (installs only the `cosmic-osd` binary to `/usr/bin`)
- Vendored/offline build: `just vendor` then `just build-vendored`; Debian package: `just build-deb`
- There are no automated tests; `TESTING.md` is a manual regression checklist (hotkeys, dialogs, headset prompt, etc.).
- Replacing the running OSD: `pkill cosmic-osd` — the session restarts it. Overwriting `/usr/bin/cosmic-osd` while it runs fails with "Text file busy"; copy to `*.new` and `mv` over it.

## Architecture

`cosmic-osd` is a single libcosmic (iced) `cosmic::Application` (`src/components/app.rs`, `App`/`Msg`) run via `run_single_instance` with no main window. It owns every on-screen surface as a Wayland layer-shell surface and reacts to events from subscriptions:

- `src/subscriptions/` — each module turns an external source into a `Msg`: `dbus` (settings-daemon/brightness/airplane/touchpad hotkey events), `polkit_agent` (+ `polkit_agent_helper`, the polkit authentication dialog), `keyboard_layout` (cosmic-protocols `zcosmic_keyboard_layout_v1` on its own thread/Wayland queue).
- `src/components/osd_indicator.rs` — the one transient indicator (`indicator` field in `App`): `Params` enum picks icon/value/rendering, `State` owns the layer surface, margin and a 3 s close timer. `create_indicator` reuses the surface (`replace_params`) instead of opening a new one when an indicator is already shown.
- `src/components/polkit_dialog.rs` — polkit auth dialog surface; `app.rs` also hosts the log out / restart / shutdown confirm dialogs.
- `src/components/audio/` + `cosmic-settings-audio-client` — volume/mute model; `*Volume` indicators are suppressed until the first volume/mute value arrives.
- CLI: `OsdTask` subcommands (`log-out`, `touchpad`, `display`, `confirm-headphones`, ...) are sent to the already-running instance through single-instance mechanics and executed by `OsdTask::perform`.
- `handle_overlap` positions the indicator: it reads panel/dock overlap rectangles and computes layer-shell margins (`state.margin` is `(top, right, bottom, left)`), and also sets blur and corner radius. Anything that changes an indicator's anchor or margin must be mirrored there, because it overwrites margins whenever the size changes.
- Layout indicator: `Params::KeyboardLayout(String)` anchors bottom-right and shows "RU"/"EN" (xkb name mapped in `Msg::KeyboardLayout` in `app.rs`). Its look is user-configurable via `src/config.rs` (`layout_style`, cosmic-config id `com.system76.CosmicOsdLayout`, keys `font_size`, `margin_right`, `margin_bottom`, `text_color_<lang>`, `bg_color_<lang>` as hex strings), re-read each time the indicator is created.
- `src/bin/cosmic-osd-layout-settings.rs` — separate libcosmic window for editing those keys; it includes `src/config.rs` via `#[path]`, so keep that file free of dependencies on the rest of the crate.

Other config is read straight from cosmic-config (`com.system76.CosmicAudio` amplification flags, `com.system76.CosmicComp` xkb config) — see `src/config.rs` and `app.rs`.

## Conventions

- Strings go through Fluent (`fl!`, `i18n/`), except the layout-settings window which is hardcoded Russian.
- Toolchain is pinned in `rust-toolchain.toml`; edition 2024, `rustfmt.toml` present.
- libcosmic and other pop-os crates are unpinned git dependencies (`Cargo.lock` pins them) — avoid `cargo update` unless intended.
