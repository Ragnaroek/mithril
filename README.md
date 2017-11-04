[![Build Status](https://travis-ci.org/Ragnaroek/mithril.svg?branch=master)](https://travis-ci.org/Ragnaroek/mithril)

# mithril
rust monero miner

TODOs:
- [x] implement cryptonight hashing function
- [ ] implement stratum protocol (for pooled mining support)
- [ ] use a naive parallelisation scheme
- [ ] measure hash performance
- [ ] optimise cryptonight hashing function
- [ ] auto-optimisation via bandit algorithms

# current status (2017-11-04)

The basic implementation for actual mining monero is ready and complete.
Shares submitted to moneropool.org (pool I am using for testing) are rejected, but
I have no idea why, although the same data as the reference implemenation xmr-stak-cpu are submitted :)

I am currently setting up my own local pool with node-cryptonote-pool to debug share submission in more detail.
