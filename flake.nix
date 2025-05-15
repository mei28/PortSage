{
  description = "🔍 PortSage - TUI to monitor processes and their open ports";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs =
    { self, nixpkgs }:
    {
      packages.x86_64-linux =
        let
          pkgs = import nixpkgs { system = "x86_64-linux"; };
        in
        pkgs.rustPlatform.buildRustPackage {
          pname = "portsage";
          version = "0.1.0";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          doCheck = false;
        };

      packages.aarch64-linux =
        let
          pkgs = import nixpkgs { system = "aarch64-linux"; };
        in
        pkgs.rustPlatform.buildRustPackage {
          pname = "portsage";
          version = "0.1.0";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          doCheck = false;
        };

      packages.x86_64-darwin =
        let
          pkgs = import nixpkgs { system = "x86_64-darwin"; };
        in
        pkgs.rustPlatform.buildRustPackage {
          pname = "portsage";
          version = "0.1.0";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          doCheck = false;
        };

      packages.aarch64-darwin =
        let
          pkgs = import nixpkgs { system = "aarch64-darwin"; };
        in
        pkgs.rustPlatform.buildRustPackage {
          pname = "portsage";
          version = "0.1.0";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          doCheck = false;
        };

      apps.x86_64-linux.default = {
        type = "app";
        program = "${self.packages.x86_64-linux}/bin/portsage";
      };

      apps.aarch64-linux.default = {
        type = "app";
        program = "${self.packages.aarch64-linux}/bin/portsage";
      };

      apps.x86_64-darwin.default = {
        type = "app";
        program = "${self.packages.x86_64-darwin}/bin/portsage";
      };

      apps.aarch64-darwin.default = {
        type = "app";
        program = "${self.packages.aarch64-darwin}/bin/portsage";
      };

      defaultPackage.x86_64-linux = self.packages.x86_64-linux;
      defaultPackage.aarch64-linux = self.packages.aarch64-linux;
      defaultPackage.x86_64-darwin = self.packages.x86_64-darwin;
      defaultPackage.aarch64-darwin = self.packages.aarch64-darwin;

      defaultApp.x86_64-linux = self.apps.x86_64-linux.default;
      defaultApp.aarch64-linux = self.apps.aarch64-linux.default;
      defaultApp.x86_64-darwin = self.apps.x86_64-darwin.default;
      defaultApp.aarch64-darwin = self.apps.aarch64-darwin.default;
    };
}
