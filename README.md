**English** | [Русский](README_ru.md)

# cosmic-osd (fork with a compact RU/EN layout indicator)

Fork of [pop-os/cosmic-osd](https://github.com/pop-os/cosmic-osd) for Pop!_OS 24.04 / COSMIC.
Instead of the large "Russian (Belarus)" popup on keyboard layout change, it shows a compact **RU** / **EN** badge
in the bottom-right corner. Size, margins and colors (separately for RU and EN) are configurable in a settings window.

## Install

```sh
git clone git@github.com:shagovAlexei/cosmic-osd.git
cd cosmic-osd
cargo build --release
```

Requires Rust (version from `rust-toolchain.toml`) and the usual COSMIC/Wayland dev packages.

Install both binaries. The packaged `/usr/bin/cosmic-osd` is running, so a plain `cp` over it fails ("Text file busy") — copy to a temp file and rename:

```sh
sudo cp target/release/cosmic-osd /usr/bin/cosmic-osd.new && sudo mv /usr/bin/cosmic-osd.new /usr/bin/cosmic-osd
install -Dm0755 target/release/cosmic-osd-layout-settings ~/.local/bin/cosmic-osd-layout-settings
pkill cosmic-osd   # the session restarts the OSD; if not, run /usr/bin/cosmic-osd manually
```

`just install` installs only `cosmic-osd`; install the settings window manually as above.

## Settings

```sh
cosmic-osd-layout-settings
```

- text size (default 15), right and bottom margins (default 50) — slider or typed number;
- text and background color for RU and EN: hex `#RRGGBB` or the palette; empty = theme color;
- "Сбросить по умолчанию" resets everything; the eraser icon next to a color resets only that color.

Values are stored in `~/.config/cosmic/com.system76.CosmicOsdLayout/v1/` and picked up the next time the indicator is shown.

## Update

```sh
git pull mine master        # or plain git pull in your fork's clone
cargo build --release
# then repeat the install commands above
```

`apt upgrade` of the `cosmic-osd` package overwrites the binary with the original — repeat the install then.
To prevent that: `sudo apt-mark hold cosmic-osd` (undo: `apt-mark unhold cosmic-osd`).

## Roll back to the original

```sh
sudo apt install --reinstall cosmic-osd && pkill cosmic-osd
```

Settings can be removed with `rm -r ~/.config/cosmic/com.system76.CosmicOsdLayout`.

## Everything else

All other indicators and dialogs (volume, brightness, polkit, etc.) are unchanged from upstream. For development see `CLAUDE.md`; the manual checklist is `TESTING.md`. License: GPL-3.0 (`LICENSE`).
