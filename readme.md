# Server Tray

## Build

```sh
cargo build

# Window
cargo build --release --target x86_64-pc-windows-gnu
```

## Development

### Dependencies

```sh
# GTK
sudo apt install libgtk-3-dev libappindicator3-dev libgdk-pixbuf2.0-dev libpango1.0-dev libcairo2-dev libxdo-dev

# For Window
sudo apt install mingw-w64
```

### export PKG_CONFIG_PATH if not already

```sh
export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/share/pkgconfig
```
