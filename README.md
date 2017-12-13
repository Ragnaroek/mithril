[![Build Status](https://travis-ci.org/Ragnaroek/mithril.svg?branch=master)](https://travis-ci.org/Ragnaroek/mithril)

# mithril
rust monero miner (pure Rust is the goal, but the project is not there yet, see help wanted section)

TODOs:
- [x] implement cryptonight hashing function
- [x] implement stratum protocol (for pooled mining support)
- [x] use a naive parallelisation scheme
- [x] measure hash performance
- [ ] implement software AES (current work)
- [ ] optimise cryptonight hashing function

Middle-Term TODOs:
- [ ] auto-optimisation via bandit algorithms
- [ ] integrate gpu mining (ATI, Nvidia)

# current status (2017-11-14)

The error with wrong submitted shares has been resolved. Mithril is now actually ready to mine Monero!
Performance is probably not yet comparable to other miners, but this will be the next step.

# help wanted

The goal of this project is to build a `pure` Rust monero miner implementation. Currently the 
Skein and JH hash functions are used as FFI C-Bindings, because there is not Rust implementation available (or I have not found any). A pure Skein or JH Rust implentation would be very welcomed. Notify me if you did implement one of these hashing
functions and I will gladly use them in mithril!
