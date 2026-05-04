![Made with Slint](https://slint.dev/logo/MadeWithSlint-logo-dark.svg)
<!-- ^^^ This displays on my labtop (with firefox) but not on another labtop with chome???
EDIT: Now it displays after clicking the link????
EDIT: its probably fine, wouldn't worry about it too much -->
# AgePad
Frontend for 'age'.

Licensed under the GPLv3 (only) inorder to *ensure* license compatability with Slint (the GUI framework).

## FAQ
<!-- I was thinking of calling this the Infrequently Asked Questions (IAQ) but that sounded too on the nose -->
#### The program is telling me to "install either AGE or RAGE"
This program requires the 'age' program to be installed or `rage` program to be installed, This is to help with binary size, compile times, and becuase I had issues reading the documentation for the `age` crate.

I do belive that handling it via an external program instead of through the `age` crate was the better approach though since it results in simpler code (and avoids the potiental for certain cryptographic mistakes).

#### Why is there a `Makefile`?
Why not? You *can* still use `cargo` if you want.

#### All the tests ending in `_rage` are failing!
Install `rage`, **and add it to PATH**!
<!-- May not be in PATH if installed via cargo, hence the "add it to PATH" -->

#### All the tests ending in `_age` are failing!
Install `age`.

## TODO
<!-- Keep at bottom of README -->
<!-- Remove when everything is done -->
- [ ] Clean up the test suite
- [ ] Clean up the GUI callback setup logic
- [ ] Remove code duplication
- [ ] Add more stuff to the FAQ
- [x] Add more stuff to the todo list
<!-- completed things go at the bottom -->
