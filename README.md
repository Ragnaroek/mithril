[![Build Status](https://travis-ci.org/Ragnaroek/mithril.svg?branch=master)](https://travis-ci.org/Ragnaroek/mithril)

# mithril
rust monero miner (pure Rust is the goal, but the project is not there yet, see help wanted section)

TODOs:
- [x] implement cryptonight hashing function
- [ ] implement stratum protocol (for pooled mining support)
- [ ] use a naive parallelisation scheme
- [ ] measure hash performance
- [ ] optimise cryptonight hashing function
- [ ] auto-optimisation via bandit algorithms

# current status (2017-11-04)

The basic implementation for actual mining monero is ready and complete.
Shares submitted to moneropool.com (pool I am using for testing) are rejected, but
I have no idea why, although the same data as the reference implemenation xmr-stak-cpu are submitted :)

I am currently setting up my own local pool with node-cryptonote-pool to debug share submission in more detail.

# help wanted

The goal of this project is to build a `pure` Rust monero miner implementation. Currently the 
Skein and JH hash functions are used as FFI C-Bindings, because there is not Rust implementation available (or I have not found any). A pure Skein or JH Rust implentation would be very welcomed. Notify me if you did implement one of these hashing
functions and I will gladly use them in mithril!
