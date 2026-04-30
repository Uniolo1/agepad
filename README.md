![Made with Slint](https://slint.dev/logo/MadeWithSlint-logo-dark.svg) <!-- This displays on my labtop (with firefox) but not on another labtop with chome. -->
# AgePad
Frontend for 'age'.

This requires the 'age' program to be installed or `rage` program to be installed using `cargo`, and not the crate, meaning age must be installed. This is to help with binary size, compile times, and development effort.

Licensed under the GPLv3 (only) inorder to *ensure* license compatability with Slint (the GUI framework).

## FAQ
(In)frequently asked questions

### Why is there a `Makefile`?
Primarily for `make test`.

### Why is the "about" button there?
Currently, it just displays the [AboutSlint](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/misc/aboutslint/) widget, it will display more in the future however. It is more of a placeholder than anything.

### All the tests ending in `_rage` are failing!
Install `rage`, **and add it to PATH**!

### All the tests ending in `_age` are failing!
Install `age`.
