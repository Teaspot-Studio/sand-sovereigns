let
  sources = import ./nix/sources.nix;
  nixpkgs-mozilla = import sources.nixpkgs-mozilla;
  pkgs = import sources.nixpkgs {
    overlays =
      [
        nixpkgs-mozilla
        (self: super:
            let chan = self.rustChannelOf { date = "2022-12-25"; channel = "nightly"; };
            in {
              rustc = chan.rust;
              cargo = chan.rust;
            }
        )
      ];
  };
  naersk = pkgs.callPackage sources.naersk {};
  merged-openssl = pkgs.symlinkJoin { name = "merged-openssl"; paths = [ pkgs.openssl.out pkgs.openssl.dev ]; };
in
naersk.buildPackage {
  name = "sand-sovereigns";
  root = pkgs.lib.sourceFilesBySuffices ./. [".rs" ".toml" ".lock" ".html" ".css" ".png" ".sh" ".sql"];
  buildInputs = with pkgs; [
    openssl
    pkgconfig
    clang
    llvm
    llvmPackages.libclang
    zlib
    cacert
    curl
    alsa-lib
    libudev-zero
    xorg.libX11
    xorg.libXi
    xorg.libXinerama
    xorg.libXext
    xorg.libXcursor
    xorg.libXrandr
    libGL
  ];
  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang}/lib";
  OPENSSL_DIR = "${merged-openssl}";
  postInstall = ''
    cp -r ${./assets} $out/bin/assets
  '';
}
