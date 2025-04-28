# ocassion

[![CI](https://github.com/itscrystalline/ocassion/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/itscrystalline/ocassion/actions/workflows/ci.yaml)
[![Coverage Status](https://coveralls.io/repos/github/itscrystalline/ocassion/badge.svg)](https://coveralls.io/github/itscrystalline/ocassion)

This program simply shows one (or many) messages when run during a configured timeframe. Useful to have in your status bar for example.

## Demonstration

[![asciicast](https://asciinema.org/a/E7idEoQNMf1mWaOy7wMw226tC.svg)](https://asciinema.org/a/E7idEoQNMf1mWaOy7wMw226tC)

## Installation

Currently, the only to install is with `cargo install`. (TBA: grabbing from github releases or `cargo binstall`)

## Configuration

When you run `ocassion` for the first time, a default config file is generated for you at `CONFIG_DIR/ocassions.json`.
The value of `CONFIG_DIR` depends on the OS [(source)](https://docs.rs/dirs/latest/dirs/fn.config_dir.html):
| Platform | Value                                 | Example                                    |
|----------|---------------------------------------|--------------------------------------------|
| Linux    | `$XDG_CONFIG_HOME` or `$HOME/.config` | `/home/alice/.config`                      |
| macOS    | `$HOME/Library/Application Support`   | `/Users/Alice/Library/Application Support` |
| Windows  | `{FOLDERID_RoamingAppData}`           | `C:\Users\Alice\AppData\Roaming`           |

The config file is written in JSON, a schema is in the works (see [#2](https://github.com/users/itscrystalline/projects/3/views/1?visibleFields=%5B%22Title%22%2C%22Assignees%22%2C%22Status%22%2C187565926%2C187565928%2C187565927%2C%22Labels%22%2C%22Milestone%22%5D&pane=issue&itemId=108037405&issue=itscrystalline%7Cocassion%7C2))

in the meantime, this is all the possible values as of 0.1:
```json
{
    "dates": [
        {
            "message": "", // String to print if the date matches time
            "time": [
                "day_of": {
                    // either "week" or "month"
                    "week": [] // List of weekdays, e.g "Mon", "Fri" or "Monday", "Friday"
                    "month": [] // List of days in the month, e.g. 1, 3, 31
                },
                "month": [], // List of month names, e.g "April", "June"
                "year": [] // List of year numbers, in AD. e.g. 2025, 2026
            ]
        },...
    ]
}
```

## Usage

After configuring, simply run the command. If the current day matches any of the rules you set, that message you configured in the rule will show up.

You would probably want to use this in conjuction with another tool that can show outputs of commands, Like in your `PS1`/`Starship`, or in a widget program like `ewww` or `conky`.

For example, in Starship, you can define a `custom` block in your `~/.config/starship.toml`:
```toml
[custom.ocassion]
command = "ocassion"
when = true
style = "fg:blue bg:black"
format = '[ $output ]($style)'
```
then, add it to your `format`:
```toml
format = """
.. snip ..
${custom.ocassion}
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
