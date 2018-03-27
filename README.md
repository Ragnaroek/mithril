[![Build Status](https://travis-ci.org/Ragnaroek/mithril.svg?branch=master)](https://travis-ci.org/Ragnaroek/mithril)

# mithril
rust monero miner (pure Rust is the goal, but the project is not there yet, see help wanted section)

Feature Backlog:
- [ ] hard-fork update
- [ ] ARM support (Raspberry, Pine64)
- [ ] integrate gpu mining (ATI, Nvidia)

DONE:
- [x] auto-optimisation via bandit algorithms
- [x] optimise cryptonight hashing function
- [x] implement software AES (current work)
- [x] measure hash performance
- [x] implement stratum protocol (for pooled mining support)
- [x] implement cryptonight hashing function

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

## Hash-Rate Logging

Mithril has basic support for logging the hash rate of the miner (in order to tune it). Hash-Rate Logging has to be
enabled in the `[metric]` section and is disabled in the default configuration:
```
enabled = false
resolution = 100 #determines how often a hash result is reported
sample_interval_seconds = 60
report_file = "/path/to/hash/report/file.csv"
```
The most important configuration option is `report_file`. You can configure an absolute path to a csv file where the hash rate is logged. Each `sample_interval_seconds` a new line with `<unix-timestamp>;<#hashes since last sample>` is appended to this file. You can calculate the average hash rate (for a given time interval) from this file with external tools (e.g. Google Drive).

The `resolution` option determines how often a hash count is measured internally. Every `resolution` hashes the result is published to a metric sub-thread in the program. Setting this to a low value will increase the overhead for measuring.

## Supported Platforms
Mithril was tested on this Platform/architecture combinations so far:
- macOS 10.13/x64
- Windows/x64
- Linux
  - CentOS 7 64bit

Please notify me, if you tested mithril on one other platform and it is running stable.

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

# Support

If you want to donate directly to support further development, this is my Monero donation address:
```
48y3RCT5SzSS4jumHm9rRL91eWWzd6xcVGSCF1KUZGWYJ6npqwFxHee4xkLLNUqY4NjiswdJhxFALeRqzncHoToeJMg2bhL
```

or support the project on Beerpay:

[![Beerpay](https://beerpay.io/Ragnaroek/mithril/badge.svg?style=beer-square)](https://beerpay.io/Ragnaroek/mithril)  [![Beerpay](https://beerpay.io/Ragnaroek/mithril/make-wish.svg?style=flat-square)](https://beerpay.io/Ragnaroek/mithril?focus=wish)


