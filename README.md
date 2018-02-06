# usrsctp-sys

[![Build Status](https://travis-ci.org/endoli/usrsctp-sys.rs.svg?branch=master)](https://travis-ci.org/endoli/usrsctp-sys.rs)
[![](http://meritbadge.herokuapp.com/usrsctp-sys)](https://crates.io/crates/usrsctp-sys)

Dual licensed under the MIT and Apache 2 licenses.

SCTP is a message oriented, reliable transport protocol with direct
support for multihoming that runs on top of IP or UDP, and supports
both v4 and v6 versions.

Like TCP, SCTP provides reliable, connection oriented data delivery
with congestion control. Unlike TCP, SCTP also provides message
boundary preservation, ordered and unordered message delivery,
multi-streaming and multi-homing. Detection of data corruption,
loss of data and duplication of data is achieved by using checksums
and sequence numbers. A selective retransmission mechanism is
applied to correct loss or corruption of data.

This library provides bindings for the [`usrsctp` implementation](https://github.com/sctplab/usrsctp)
of SCTP.

## Documentation

The API is fully documented with examples:
[https://docs.rs/usrsctp-sys/](https://docs.rs/usrsctp-sys/)

## Installation

This crate works with Cargo and is on
[crates.io](https://crates.io/crates/usrsctp-sys).
Add it to your `Cargo.toml` like so:

```toml
[dependencies]
usrsctp-sys = "0.1.0"
```

## Status of Implementation

Things are under active development. This project is not quite
usable yet as some of the basic functionality is being written.

## Support and Maintenance

I am developing this library largely on my own so far. I am able
to offer support and maintenance, but would very much appreciate
donations via [Patreon](https://patreon.com/endoli). I can also
provide commercial support, so feel free to
[contact me](mailto:bruce.mitchener@gmail.com).

## Contribution

Unless you explicitly state otherwise, any contribution
intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional
terms or conditions.
