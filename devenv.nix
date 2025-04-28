{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: let
  runBeforeTest = tasks: builtins.mapAttrs (name: value: value // {before = ["devenv:enterTest"];}) tasks;
in {
  # https://devenv.sh/basics/
  env.CARGO_TERM_COLOR = "always";

  # https://devenv.sh/packages/

  # https://devenv.sh/languages/
  # languages.rust.enable = true;
  packages = with pkgs; [
    cargo-tarpaulin
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
  scripts.test-quick.exec = ''
    cargo tarpaulin --color always --skip-clean
  '';
  #
  # enterShell = ''
  # '';

  # https://devenv.sh/tasks/
  tasks = runBeforeTest {
    "ocassion:check".exec = "cargo check";
    "ocassion:lint".exec = "cargo clippy";
    "ocassion:test".exec = "cargo test";
  };

  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests & grabbing test coverage"
    cargo tarpaulin --color always
  '';

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

  # See full reference at https://devenv.sh/reference/options/
}
