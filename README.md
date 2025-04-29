# occasion

[![CI](https://github.com/itscrystalline/occasion/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/itscrystalline/occasion/actions/workflows/ci.yaml)
[![Coverage Status](https://coveralls.io/repos/github/itscrystalline/occasion/badge.svg)](https://coveralls.io/github/itscrystalline/occasion)

This program simply shows one (or many) messages when run during a configured timeframe. Useful to have in your status bar for example.

## Demonstration

[![asciicast](https://asciinema.org/a/E7idEoQNMf1mWaOy7wMw226tC.svg)](https://asciinema.org/a/E7idEoQNMf1mWaOy7wMw226tC)

## Installation

Currently, you can grab the [latest release](https://github.com/itscrystalline/occasion/releases/latest).
You can also run `cargo install occasion` or `cargo binstall occasion` and it will be avaliable in your `PATH`.

### Nix (with flakes)

Alternatively, if you use Nix, `occasion` has a Nix flake and a home-manager module for you to use.
In your `flake.nix`, add this to your `inputs`:
```nix
{
    inputs = {
        # .. snip ..
        occasion.url = "github:itscrystalline/occasion";
    };
    # ..
}
```
This flake contains two packages: `occasion`, which is the latest release on Github Releases, and `occasion-latest`, which builds from the latest commit on the `main` branch. They are avaliable with `inputs.occasion.packages.${pkgs.system}.occasion` and `inputs.occasion.packages.${pkgs.system}.occasion-latest` respectively.

To use with home-manager, import the `occasion.homeManagerModule` in your home-manager imports, you will then have access to `programs.occasion`. For example:
```nix
programs.occasion = {
  enable = true;
  # package = inputs.occasion.packages.${pkgs.system}.occasion;
  settings = {
    dates = [
      {
        message = "test";
        time = {
          day_of = {
            week = ["Tuesday"];
          };
        };
      }
    ];
    multiple_behavior = {
      all = {seperator = "";};
    };
  };
};
```

## Configuration

When you run `occasion` for the first time, a default config file is generated for you at `CONFIG_DIR/occasions.json`.
The value of `CONFIG_DIR` depends on the OS [(source)](https://docs.rs/dirs/latest/dirs/fn.config_dir.html):
| Platform | Value                                 | Example                                    |
|----------|---------------------------------------|--------------------------------------------|
| Linux    | `$XDG_CONFIG_HOME` or `$HOME/.config` | `/home/alice/.config`                      |
| macOS    | `$HOME/Library/Application Support`   | `/Users/Alice/Library/Application Support` |
| Windows  | `{FOLDERID_RoamingAppData}`           | `C:\Users\Alice\AppData\Roaming`           |

The config file is written in JSON, a schema is automatically added to a default config. The schema lives at `occasions.schema.json` at the root of the repository. 

## Usage

After configuring, simply run the command. If the current day matches any of the rules you set, that message you configured in the rule will show up.

You would probably want to use this in conjuction with another tool that can show outputs of commands, Like in your `PS1`/`Starship`, or in a widget program like `ewww` or `conky`.

For example, in Starship, you can define a `custom` block in your `~/.config/starship.toml`:
```toml
[custom.occasion]
command = "occasion"
when = true
style = "fg:blue bg:black"
format = '[ $output ]($style)'
```
then, add it to your `format`:
```toml
format = """
.. snip ..
${custom.occasion}
.. snip ..
"""
```
You would get something like this.
![starship_result](https://github.com/user-attachments/assets/138cc981-30f7-43ac-b33b-34339c2d7445)

## Development

A Development environment can be set up automatically with [`devenv`](https://devenv.sh).

Otherwise, Install Rust 1.86.0 from https://rustup.rs, and [`cargo-tarpaulin`](https://github.com/xd009642/tarpaulin) for code coverage checking.

To run tests and generate a code coverage report, run `devenv test -d` (omit the `-d` flag if you wanna rebuild the environment everytime), or without devenv, run `cargo tarpaulin --color always --skip-clean` (or without `--skip-clean` to rebuild everything everytime).

Otherwise, run the standard `cargo check`, `cargo clippy` and `cargo test` for checking, linting, and testing (without coverage).
