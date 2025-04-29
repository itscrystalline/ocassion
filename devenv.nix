{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  # https://devenv.sh/basics/
  env.CARGO_TERM_COLOR = "always";

  # https://devenv.sh/packages/

  # https://devenv.sh/languages/
  # languages.rust.enable = true;
  packages = with pkgs; [
    cargo-tarpaulin
    cargo-bloat
    irust
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
    components = ["rustc" "cargo" "clippy" "rustfmt" "rust-analyzer"];
  };

  # https://devenv.sh/processes/
  # processes.cargo-watch.exec = "cargo-watch";

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/scripts/
  # scripts.test-quick.exec = ''
  #   cargo tarpaulin --color always --skip-clean
  # '';
  #
  # enterShell = ''
  # '';

  # https://devenv.sh/tasks/
  tasks = {
    "occasion:check".exec = "cargo check";
    "occasion:lint".exec = "cargo clippy -- -Dwarnings";
    "occasion:test".exec = "cargo test";
    "occasion:coverage".exec = "${pkgs.cargo-tarpaulin}/bin/cargo-tarpaulin --color always --verbose --all-features --workspace --timeout 120 --out xml";
  };

  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests & grabbing test coverage"
    cargo tarpaulin --color always --skip-clean
  '';

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

  # See full reference at https://devenv.sh/reference/options/
}
