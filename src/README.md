# Taraffic - A traffic management simulator

## Project Status

This project is currently in its **Proof of Concept (POC)** phase. 

The components and tools here are for initial development, testing and validation of core game mechanics.

Currently, the repo houses my in-house development and debugging tools for the core game mechanic, the TPU.

## RGAL - Rudimentary Game Assembly Language

You can read more about RGAL in its dedicated [documentation](./rgal/rgal.md) file

Essentially, it is a language influenced by IC10, MIPS and 6502 assembly instructions,
created specifically to target the TPU.

## TPU - Traffic Processing Unit

A basic System-on-a-Chip, using a 16-bit word size.

The standard configuration is:

* 16 Words of stack
* 128 Words of RAM
* Upto 65,535 lines of ROM instructions.
* Digital and Analog I/O
* Basic networking functionality for communicating with other peripherals and TPUs

During each level of the game, you can buy or upgrade additional peripherals like sensors and buttons that you can use
to better evaluate your environment and perform more complex traffic management. Depending on what you connect to your
TPU, the I/O pins will be automatically configured as either inputs or outputs.

For instance, you could purchase :

* A sensor that tells you how many vehicles are waiting at the junction
* A counter that tells you how many vehicles passed in the last minute
* A button for pedestrians to press when they are wanting to cross

* Without data from the world around you, you'll only be able to perform basic timing sequences.

## What's in this Repo?

* The RGAL PEG Grammar
* The TPU implementation
* A basic debugging and visualiser tool
* Some tests for the TPU and RGAL implementations

![debugger.png](res/debugger.png)

## Building

To build the project and run tests:

```bash
cargo build
cargo test
```

To run the debugger tool:

``` bash
cargo run
```

## Contributing

This project is currently in its Proof of Concept phase, but feedback and contributions are highly appreciated.

Please open an issue if:

*   **You found a bug?** Please give us as many details as you can.
*   **Have a suggestion?** Feel free to open an issue to discuss new features or improvements.
*   **Want to contribute code?** Please open a discussion about the proposed changes before submitting a pull request.

By contributing to these tools, you agree that your contributions will be licensed under the GPLv3 license, 
which also covers the tools within this repository. 
Please ensure you are comfortable with this licensing before submitting any contributions.

## License

The RGAL language, the debugger and the VM implementation are licensed under the GNU General Public license v3.0.

However, the game engine implementation itself and associated assets will not be available as an open-source project for the foreseeable future.

See the [LICENSE](LICENSE) file for the full license text.


