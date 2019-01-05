[![Build Status](https://travis-ci.org/Ragnaroek/mithril.svg?branch=master)](https://travis-ci.org/Ragnaroek/mithril)

[![Donate](https://img.shields.io/badge/Donate-PayPal-green.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=A24SWQT3P5DY2&source=url)

# Mithril

**Mithril v0.7.0 is ready for the PoW change. You need to update Mithril to the latest version, it will autodetect the correct PoW version.**

Rust Monero Miner (pure Rust is the goal, but the project is not there yet, see help wanted section)

## Unique Selling Points

aka: Why you should consider using Mithril.

- Auto-Tuning: Finds the optimal setup for your hardware itself with a bandit algorithm
- Easy to compile: inherited from the great rust toolchain
- cross platform: should run on every (x86) platform supported by Rust
- [Fast: should be as fast as the C-implementations] (I want to say that, but I currently cannot prove it)

## Roadmap

Feature Backlog:
- [ ] hard-fork v8 update!
- [ ] integrate GPU Mining  (AMD)
- [ ] WebAssembler Support
- [ ] Better displaying of Hash-Rate

Future Feature Backlog

- [ ] ARM support (Raspberry, Pine64)
- [ ] integrate GPU Mining (NVIDIA)
- [ ] Skein, JH native Rust implementation

DONE:
- [x] hard-fork v7 update
- [x] auto-optimisation via bandit algorithms
- [x] optimise cryptonight hashing function
- [x] implement software AES (current work)
- [x] measure hash performance
- [x] implement stratum protocol (for pooled mining support)
- [x] implement cryptonight hashing function

## Current Status (2019-01-05)

I am currently working on the hard-fork v8 update.

Next feature implemented will be the support for AMD GPU mining (the projects wants to be a All-In-One Miner after all).

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

## Auto-Tuning

### Configuration

Auto-Tuning is enabled by default in Mithril. You can disable and configure auto-tuning in the config.toml:

```toml
[worker]
num_threads = 8
auto_tune = true
auto_tune_interval_minutes = 15
auto_tune_log = "./bandit.log"
```

If you set `auto_tune` to `false`, Mithril will honour your `num_threads` and will use the number of threads configured
there. The other options are only relevant if you set `auto_tune` to `true`. The config `auto_tune_interval_minutes` controls, how often a new bandit arm will be drawn and a new thread count setup will be tried. I suggest picking a longer interval, to average-out some spikes on loads on the machine the miner is running on.

You can enable detailed logging by setting a path to a file in `auto_tune_log`. Each step in the bandit algorithm
will be logged there. You can evaluate the performance of the bandit algorithm on your machine with the Bandit-Tools that have been created for exactly this purpose. You find them here: [Bandit-Tools](https://github.com/Ragnaroek/bandit-tools).

The current state of the bandit algorithm will always be saved to `~/.mithril/bandit_state.json`.
You can stop the miner and on the next startup it will continue the arm evaluation on the point were it stopped last.

## Evaluation

As mentioned you can use the [Bandit-Tools Web-App](https://ragnaroek.github.io/bandit-tools/) to evaluate
the evaluation/exploitation phases of the bandit algorithm. You can upload the `bandit_state.json` and `bandit.log`
file and get a visualisation of the information present in these two files.

If you want to share publicly your log and state files, please open a pull request on the bandit_data branch of this project. Discussing results should be done on Reddit: [Reddit Post](https://www.reddit.com/r/MoneroMining/comments/8vp873/mithril_miner_and_autotuning_with_a/).

## Hash-Rate Logging

Mithril has basic support for logging the hash rate of the miner (in order to tune it). Hash-Rate Logging has to be
enabled in the `[metric]` section and is disabled in the default configuration:

```toml
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

or Paypal:

[![Donate](https://img.shields.io/badge/Donate-PayPal-green.svg)](https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=A24SWQT3P5DY2&source=url)
