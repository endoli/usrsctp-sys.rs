// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! SCTP is a message oriented, reliable transport protocol with direct
//! support for multihoming that runs on top of IP or UDP, and supports
//! both v4 and v6 versions.
//!
//! Like TCP, SCTP provides reliable, connection oriented data delivery
//! with congestion control. Unlike TCP, SCTP also provides message
//! boundary preservation, ordered and unordered message delivery,
//! multi-streaming and multi-homing. Detection of data corruption,
//! loss of data and duplication of data is achieved by using checksums
//! and sequence numbers. A selective retransmission mechanism is
//! applied to correct loss or corruption of data.
//!
//! The API for the SCTP User-land implementation is based on
//! [RFC 6458](http://tools.ietf.org/html/rfc6458).
//! The main focus of this document is on pointing out the differences
//! to the SCTP Sockets API. For all aspects of the sockets API that
//! are not mentioned in this documentation, please refer to
//! [RFC 6458](http://tools.ietf.org/html/rfc6458). Questions about
//! SCTP itself can hopefully be answered by
//! [RFC 4960](http://tools.ietf.org/html/rfc4960).
//!
//! This library provides bindings for the [`usrsctp` library] which
//! provides a userland implementation of SCTP that doesn't require
//! any special support from the underlying operating system or
//! kernel, beyond a normal networking stack. The `usrsctp` library
//! is widely used on a variety of platforms.
//!
//! ## Installation
//!
//! This crate works with Cargo and is on
//! [crates.io](https://crates.io/crates/usrsctp-sys/).
//! Add it to your `Cargo.toml` like so:
//!
//! ```toml
//! [dependencies]
//! usrsctp-sys = "0.1.0"
//! ```
//!
//! Then, let `rustc` know that you're going to use this crate at the
//! top of your own crate:
//!
//! ```
//! extern crate usrsctp_sys;
//! # fn main() {}
//! ```
//!
//! The crate bundles a copy of the underlying [`usrsctp` library] and
//! will handle building it when using this crate.
//!
//! ## Basic Operations
//!
//! All system calls start with the prefix `usrsctp_` to distinguish them
//! from the kernel variants. Some of them are changed to account for the
//! different demands in the userland environment.
//!
//! ## Differences to RFC 6458
//!
//! * [`usrsctp_init()`]: Not in RFC.
//! * [`usrsctp_finish()`]: Not in RFC.
//! * [`usrsctp_socket()`]: Additional arguments for callback-driven I/O.
//! * [`usrsctp_close()`]: No return value as in the RFC.
//!
//! Additionally, while the RFC discusses many functions for sending and
//! receiving data, all but two are deprecated. These deprecated functions
//! are not available within `usrsctp-sys`:
//!
//! * `sctp_recvmsg`
//! * `sctp_send`
//! * `sctp_sendmsg`
//! * `sctp_sendx`
//!
//! These functions may be used:
//!
//! * [`usrsctp_recvv()`]
//! * [`usrsctp_sendv()`]
//!
//! Additionally, since sockets are not raw file descriptors, standard functions
//! like [`read()`] and [`write()`] may not be used.
//!
//! [`usrsctp` library]: https://github.com/sctplab/usrsctp
//! [`usrsctp_init()`]: fn.usrsctp_init.html
//! [`usrsctp_finish()`]: fn.usrsctp_finish.html
//! [`usrsctp_socket()`]: fn.usrsctp_socket.html
//! [`usrsctp_close()`]: fn.usrsctp_close.html
//! [`usrsctp_recvv()`]: fn.usrsctp_recvv.html
//! [`usrsctp_sendv()`]: fn.usrsctp_sendv.html
//! [`read()`]: ../libc/fn.read.html
//! [`write()`]: ../libc/fn.write.html

#![allow(non_camel_case_types)]
#![cfg_attr(feature = "cargo-clippy",
            allow(decimal_literal_representation, new_without_default_derive,
                  should_implement_trait, unreadable_literal, useless_transmute))]

extern crate libc;

mod bindings;

pub use bindings::*;
