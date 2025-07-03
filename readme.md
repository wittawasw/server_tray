# Server Tray

## Build

```sh
# Tested in MacOS
cargo build --release

# Window
cargo rustc --release -- -C link-args=/SUBSYSTEM:WINDOWS
```

## Development

### Dependencies

```sh
# GTK
sudo apt install libgtk-3-dev libappindicator3-dev libgdk-pixbuf2.0-dev libpango1.0-dev libcairo2-dev libxdo-dev
```

### export PKG_CONFIG_PATH if not already

```sh
# In Linux
export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/share/pkgconfig
```
