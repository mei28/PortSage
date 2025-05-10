{
  description = "🔍 PortSage - TUI to monitor processes and their open ports";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forAllSystems =
        f:
        builtins.listToAttrs (
          map (system: {
            name = system;
            value = f system;
          }) systems
        );

      packages = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
          isLinux = pkgs.stdenv.isLinux;
        in
        pkgs.rustPlatform.buildRustPackage {
          pname = "portsage";

          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs =
            with pkgs;
            if isLinux then
              [
                libxkbcommon
                xorg.libX11
              ]
            else
              [ ];
        }
      );
    in
    {
      packages = packages;

      defaultPackage = forAllSystems (system: packages.${system});

      apps = forAllSystems (system: {
        type = "app";
        program = "${packages.${system}}/bin/portsage";
      });

      defaultApp = forAllSystems (system: self.apps.${system});
    };

}
