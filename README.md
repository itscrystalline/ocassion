# ocassion

[![CI](https://github.com/itscrystalline/ocassion/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/itscrystalline/ocassion/actions/workflows/ci.yaml)
[![Coverage Status](https://coveralls.io/repos/github/itscrystalline/ocassion/badge.svg)](https://coveralls.io/github/itscrystalline/ocassion)

This program simply shows one (or many) messages when run during a configured timeframe. Useful to have in your status bar for example.

## Demonstration

[![asciicast](https://asciinema.org/a/E7idEoQNMf1mWaOy7wMw226tC.svg)](https://asciinema.org/a/E7idEoQNMf1mWaOy7wMw226tC)

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
