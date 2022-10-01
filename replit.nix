{ pkgs }: {
    deps = [
        pkgs.rustup
        pkgs.gnumake
        pkgs.binaryen
        pkgs.nodejs-16_x
        pkgs.rust-analyzer
    ];
}