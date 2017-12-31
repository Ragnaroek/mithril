[![Build Status](https://travis-ci.org/Ragnaroek/mithril.svg?branch=master)](https://travis-ci.org/Ragnaroek/mithril)

# mithril
rust monero miner (pure Rust is the goal, but the project is not there yet, see help wanted section)

TODOs:
- [x] implement cryptonight hashing function
- [x] implement stratum protocol (for pooled mining support)
- [x] use a naive parallelisation scheme
- [x] measure hash performance
- [x] implement software AES (current work)
- [x] optimise cryptonight hashing function
- [ ] ARM support (Raspberry, Pine64)

Middle-Term TODOs:
- [ ] auto-optimisation via bandit algorithms
- [ ] integrate gpu mining (ATI, Nvidia)


# HowTo Compile, Configure and Run

You need the Rust nightly version to compile Mithril, since it uses inline assembler which is only available
in the nightly version of Rust. The nightly version is best installed with [rustup](https://www.rustup.rs/).
Once you have the nightly version installed, type `cargo build --release` for an optimised binary.
The binary can be found in the `target/release/` folder.

Mithril expects a `config.toml` in the working directory. Copy the `default_config.toml` as `config.toml` to the Mithril
working directory. You need at least configure your Monero address in the `[pool]` section for the reward and the `num_threads` depending on your machine (a good start is to use 2x number of your cores on your machine).

If you get a `wrong instruction set` kind of error you can try to disable hardware AES with the `has_aes` flag in the
`[hardware]` section.

If you find any issues, please report them here: [Mithril Issues](https://github.com/Ragnaroek/mithril/issues)

## Supported Platforms
Mithril was tested on this Platform/architecture combinations so far:
- macOS 10.13/x64
- Windows/x64

Please notify me, if you tested mithril on one other platform (Linux would be intersting, since I'm on macOS only) and it is running stable.

ARM support (Raspberry, Pine64) is a short term goal I am working on.

# Current Status (2017-12-31)

Obvious ineffieciencies have been removed. Mithril should now generate a decent hash rate.

Some measurements from my dev notebook can be found here:
https://docs.google.com/spreadsheets/d/1ZAqV4JXxO-L9sNFzW__wBytgfZKNniK9WUKJk6tl5wI/edit?usp=sharing

Average hash-rate is 183 H/sec with a Intel Core i7 running with 8 threads.

# Help Wanted

The goal of this project is to build a `pure` Rust monero miner implementation. Currently the
Skein and JH hash functions are used as FFI C-Bindings, because there is not Rust implementation available (or I have not found any). A pure Skein or JH Rust implentation would be very welcomed. Notify me if you did implement one of these hashing
functions and I will gladly use them in mithril!
