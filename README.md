# TFPC - Team Fortress Price Calculator

TFPC is a metal currency calculator for TF2.

## Installation

You can download the [compiled version](https://gitlab.com/cuqerr/tfpc/-/tree/master/bin) (I didn't sign the executables. This is why it gives the warning.)

Or you can compile the project by downloading [rustup](https://rustup.rs/) and running:

```bash
cargo install --path .
```

Then you can use TFPC by typing tfpc in your terminal.

## Usage
Type out your calculation. Use ref for refined metal, rec for reclaimed metal, scr for scrap metal.
```
>>> 12.77 ref
115 SCR
38 REC + 1 SCR
12 REF + 2 REC + 1 SCR
------------------------
NET: 12.77 REF
```

## Contributing
Pull requests are welcome.

## License
[GNU General Public License v3.0](https://gitlab.com/cuqerr/tfpc/-/raw/master/LICENSE)
