# The Biter Modding Library

The Biter Modding Library is a runtime library that hooks into Factorio's runtime to provide deep
modding capabilities. It is designed to be used by modders to create mods that are more powerful and
flexible than what is possible with Factorio's built-in modding API.

## Installation

This project is not yet ready for production. It is still in the early stages of development and is
not yet feature complete. If you would like to try it out, you can clone the repository and build it
yourself. But consider yourself warned: this is not yet a stable project.

## Development

This project is developed using the Rust programming language. You will need to have Rust installed
on your system in order to build and run this project. You can install Rust by following the
instructions on the [official Rust website](https://www.rust-lang.org/).

The project is configured for easy setup as long as you're using Visual Studio Code. The setup
scripts are currently only written with MacOS in mind, but they should be easily adaptable to other
operating systems, and I welcome pull requests for this.

You can run the following commands to set up the project for development:

```bash
$ pre-commit install
$ cargo build
$ cargo task setup
```

This will compile the project, then create a backup of the Factorio binary inside the game folder,
under `factorio.bak`. It will then modify the factorio binary headers to load the library at launch.
Finally, on MacOS, it will sign the binary **with your own certificate**. This is necessary to
prevent MacOS from blocking the binary from running.

## Contributing

Contributions are welcome! If you would like to contribute to this project, please open an issue to
discuss your idea in the Discussions section. I am open to ideas and suggestions, as I am not yet
even aware of all the possibilities that this library could provide.

If you're running on Windows or Linux, we need to figure out how to load the library dynamically on
those platforms as well. Should be easy, but I don't have the means to test it myself. I am more
than happy to pair with someone to get this working. Should be a 2h job at most on each platform.

## Acknowledgements

Well, first of all, I would like to thank the Factorio developers for creating such an amazing game
and for providing such a rich modding API to begin with. Yes, this project is not using that API,
but I want to acknowledge the lightyears ahead that this game is in terms of modding capabilities
compared to other games.

This library wouldn't be possible without the amazing [Frida](https://frida.re/) library, which
provides the means to hook into the Factorio runtime.

Finally, this project is heavily inspired by the
[Factorio Script Extender](https://github.com/0x25CBFC4F/factorio_script_extender) experiment. I
referenced that project heavily to understand how to hook into Factorio's runtime, and I would like
to thank the author for their work.

## License

This project is licensed under the GNU Affero General Public License v3.0. You can read the full
license in the [LICENSE](LICENSE) file.
