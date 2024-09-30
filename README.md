# simple-key-monitor

A simple key monitor that prints the key code of the pressed key.

## Install

```shell
cargo install --git https://github.com/nahco314/simple-key-monitor
```

## Usage

Just run `simple-key-monitor`.

If you are using alacritty (on linux), you can display a clean GUI with a command like the following:

```shell
alacritty \
    -T simple-key-monitor \
    --class simple-key-monitor \
    -o "window.dimensions = { columns = 30, lines = 6 }" \
    -e sh -c "stty -echo && printf \"\e[?25l\" && simple-key-monitor"
```

## Build

This tool uses the Rust wrapper for libuiohook (https://github.com/efJerryYang/uiohook-rs), so the libraries that libuiohook depends on are required for building.

If you are using apt, install it with the following command.

```
sudo apt-get install libx11-dev libxtst-dev libxt-dev libxinerama-dev libx11-xcb-dev libxkbcommon-dev libxkbcommon-x11-dev libxkbfile-dev
```
