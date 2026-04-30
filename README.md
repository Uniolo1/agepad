![Made with Slint](https://slint.dev/logo/MadeWithSlint-logo-dark.svg) <!-- This displays on my labtop (with firefox) but not on another labtop with chome???
EDIT: Now it displays after clicking the link????-->
# AgePad
Frontend for 'age'.

This requires the 'age' program to be installed or `rage` program to be installed using `cargo`, and not the crate, meaning age must be installed. This is to help with binary size, compile times, and development effort.

Licensed under the GPLv3 (only) inorder to *ensure* license compatability with Slint (the GUI framework).

## FAQ
#### Why is there a `Makefile`?
Primarily for `make test`.

#### All the tests ending in `_rage` are failing!
Install `rage`, **and add it to PATH**!

#### All the tests ending in `_age` are failing!
Install `age`.

## TODO
<!-- Keep at bottom of README -->
- [ ] Clean up the test suite
- [ ] Clean up the GUI callback setup logic (and avoid creating a bunch of weaks)
- [ ] Add more stuff to the todo list
