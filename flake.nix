{
  description = "plugged - a lightweight hardware event audio daemon";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    let
      perSystem = flake-utils.lib.eachDefaultSystem (
        system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs { inherit system overlays; };

          runtimeDeps = with pkgs; [
            systemd
            alsa-lib
            libopus
            libnotify
            glib
          ];

          buildDeps = with pkgs; [ pkg-config ];

          rust-toolchain = pkgs.rust-bin.stable.latest.default.override {
            extensions = [
              "rust-analyzer"
              "rust-src"
            ];
          };

          rustPlatform = pkgs.makeRustPlatform {
            cargo = rust-toolchain;
            rustc = rust-toolchain;
          };

          plugged-pkg = rustPlatform.buildRustPackage {
            pname = "plugged";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = buildDeps;
            buildInputs = runtimeDeps;
          };
        in
        {
          packages.default = plugged-pkg;

          devShells.default = pkgs.mkShell {
            nativeBuildInputs = [ rust-toolchain ] ++ buildDeps;
            buildInputs = runtimeDeps;
            env = {
              RUST_SRC_PATH = "${rust-toolchain}/lib/rustlib/src/rust/library";
            };

          };
        }
      );
    in
    perSystem
    // {
      nixosModules.default =
        {
          config,
          lib,
          pkgs,
          ...
        }:
        let
          cfg = config.services.plugged;
          binary = "${self.packages.${pkgs.system}.default}/bin/plugged";
        in
        {
          options.services.plugged.enable = lib.mkEnableOption "plugged daemon";
          config = lib.mkIf cfg.enable {
            systemd.services.plugged = {
              Unit = {
                Description = "Plugged Audio Daemon";
                Documentation = "file://${self}/plugged.service";
                After = [ "graphical-session.target" ];
              };
              Service = {
                ExecStart = binary;
                Restart = "always";
              };
              Install.WantedBy = [ "default.target" ];
            };
          };
        };

      homeManagerModules.default =
        {
          config,
          lib,
          pkgs,
          ...
        }:
        let
          cfg = config.services.plugged;
          binary = "${self.packages.${pkgs.system}.default}/bin/plugged";
          tomlFormat = pkgs.formats.toml { };
          configFile = tomlFormat.generate "config.toml" {
            sounds = {
              enable = cfg.settings.sounds.enable;
              connected = toString cfg.settings.sounds.connected;
              disconnected = toString cfg.settings.sounds.disconnected;
            };
            notifications = {
              enable = cfg.settings.notifications.enable;
              format = cfg.settings.notifications.format;
            };
          };
        in
        {
          options.services.plugged = {
            enable = lib.mkEnableOption "plugged daemon";
            settings = {
              sounds = {
                enable = lib.mkOption {
                  type = lib.types.bool;
                  default = true;
                  description = "Enable sound playback on USB events";
                };
                connected = lib.mkOption {
                  type = lib.types.path;
                  description = "Path to the connection sound";
                };
                disconnected = lib.mkOption {
                  type = lib.types.path;
                  description = "Path to the disconnection sound";
                };
              };
              notifications = {
                enable = lib.mkOption {
                  type = lib.types.bool;
                  default = true;
                  description = "Enable desktop notifications on USB events";
                };
                format = lib.mkOption {
                  type = lib.types.str;
                  default = "Device {{device_name}} {{action}}.";
                  description = "Notification body template";
                };
              };
            };
          };

          config = lib.mkIf cfg.enable {
            xdg.configFile."plugged/config.toml".source = configFile;
            home.packages = [ self.packages.${pkgs.system}.default ];
            systemd.user.services.plugged = {
              Unit = {
                Description = "Plugged Audio Daemon";
                Documentation = "file://${self}/plugged.service";
                After = [ "graphical-session.target" ];
              };
              Service = {
                ExecStart = binary;
                Restart = "always";
              };
              Install.WantedBy = [ "default.target" ];
            };
          };
        };
    };
}
