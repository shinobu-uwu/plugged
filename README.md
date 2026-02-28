# plugged 🔌🔊

`plugged` is a lightweight hardware event audio daemon written in Rust.
It monitors your system for USB connection events via `udev` and plays
configurable sounds when devices are plugged in or out.

## 🛠️ Installation (Standard Linux)

First of all, you will need `libudev`, and `alsa-lib` installed,
there is a big chance you already do.

1. You can either build the daemon yourself, with `cargo build --release`,
or download it from the latest release

2. Configure the daemon, create the `~/.config/plugged/config.toml` file:

```toml
[sounds]
enable = true
connected = "<absolute path to your file>"
disconnected = "<absolute path to your file>"

[notifications]
enable = true
format = "Device {{device_name}} {{action}}."
```

3. (Optional) Create a systemd unit for your user and enable it:

```bash
mkdir -p `~/.config/systemd/user`
git clone https://github.com/shinobu-uwu/plugged
cd plugged
cp plugged.service ~/.config/systemd/user/
systemctl --user enable --now plugged
```

## 🚀 Installation (Nix / NixOS)

Add `plugged` to your flake inputs:

```nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    # your other inputs...
    plugged.url = "github:shinobu-uwu/plugged";
  };
}
```

### Via home manager (recommended)

```nix
{ inputs, ... }: {
  imports = [ inputs.plugged.homeManagerModules.default ];

  services.plugged = {
    enable = true;
    settings.sounds = {
      enable = true;
      connected = ./path/to/plug.ogg;
      disconnected = ./path/to/unplug.ogg;
    };
    settings.notifications = {
      enable = true;
      format = "Device {{device_name}} {{action}}.";
    };
  };
}
```

### Via NixOS Module

```nix
{ inputs, ... }: {
  imports = [ inputs.plugged.nixosModules.default ];
  services.plugged.enable = true;
}
```

## 🪳 Debugging

- Debug builds: default to `info` log level
- Release builds: default to `warn` log level
- You can override at any time by running the daemon with the `RUST_LOG`
environment variable, possible values are:
    - off: Turns off all logging.
    - error: Only show serious errors.
    - warn: Show errors and warnings (good for production/release).
    - info: Show general operational messages (good for users).
    - debug: Show detailed information for developers.
    - trace: Show everything, including low-level library internal events.
