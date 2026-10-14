# cosmic-osd (форк с индикатором раскладки RU/EN)

Форк [pop-os/cosmic-osd](https://github.com/pop-os/cosmic-osd) для Pop!_OS 24.04 / COSMIC.
Вместо большого окна «Russian (Belarus)» при смене раскладки показывается компактный бейдж **RU** / **EN**
в правом нижнем углу. Размер, отступы и цвета (отдельно для RU и EN) настраиваются в окне настроек.

## Установка

```sh
git clone git@github.com:shagovAlexei/cosmic-osd.git
cd cosmic-osd
cargo build --release
```

Нужны Rust (версия из `rust-toolchain.toml`) и dev-пакеты COSMIC/Wayland (как для остальных компонентов COSMIC).

Ставим оба бинаря. Пакетный `/usr/bin/cosmic-osd` запущен, поэтому поверх него `cp` не работает («Text file busy») — копируем во временный файл и переименовываем:

```sh
sudo cp target/release/cosmic-osd /usr/bin/cosmic-osd.new && sudo mv /usr/bin/cosmic-osd.new /usr/bin/cosmic-osd
install -Dm0755 target/release/cosmic-osd-layout-settings ~/.local/bin/cosmic-osd-layout-settings
pkill cosmic-osd   # сессия перезапустит OSD; если нет — запусти /usr/bin/cosmic-osd вручную
```

`just install` ставит только `cosmic-osd`, окно настроек — вручную, как выше.

## Настройки

```sh
cosmic-osd-layout-settings
```

- размер текста (по умолчанию 15), отступы справа и снизу (по умолчанию 50) — ползунок или число;
- цвет текста и фона для RU и EN: hex `#RRGGBB` или палитра; пусто — цвет темы;
- «Сбросить по умолчанию» — всё сразу, иконка-ластик у цвета — только он.

Значения хранятся в `~/.config/cosmic/com.system76.CosmicOsdLayout/v1/` и подхватываются при следующем показе индикатора.

## Обновление

```sh
git pull mine master        # или git pull в своём клоне форка
cargo build --release
# затем те же команды установки, что выше
```

Обновление пакета `cosmic-osd` через `apt upgrade` перезаписывает бинарь оригиналом — тогда повтори установку.
Чтобы этого не было: `sudo apt-mark hold cosmic-osd` (снять: `apt-mark unhold cosmic-osd`).

## Откат на оригинал

```sh
sudo apt install --reinstall cosmic-osd && pkill cosmic-osd
```

Настройки в `~/.config` можно удалить: `rm -r ~/.config/cosmic/com.system76.CosmicOsdLayout`.

## Остальное

Все остальные индикаторы и диалоги (громкость, яркость, polkit и т.д.) — без изменений, как в оригинале. Для разработки см. `CLAUDE.md`, ручной чек-лист — `TESTING.md`. Лицензия — GPL-3.0 (`LICENSE`).
