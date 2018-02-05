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
//! In this manual the socket API for the SCTP User-land implementation
//! will be described.  It is based on [RFC 6458](http://tools.ietf.org/html/rfc6458).
//! The main focus of this document is on pointing out the differences
//! to the SCTP Sockets API. For all aspects of the sockets API that
//! are not mentioned in this document, please refer to
//! [RFC 6458](http://tools.ietf.org/html/rfc6458). Questions about
//! SCTP itself can hopefully be answered by [RFC 4960](http://tools.ietf.org/html/rfc4960).
//!
//! ## Basic Operations
//!
//! All system calls start with the prefix `usrsctp_` to distinguish them
//! from the kernel variants. Some of them are changed to account for the
//! different demands in the userland environment.
//!
//! ## Differences to RFC 6458
//!
//! ### usrsctp_init()
//!
//! Every application has to start with `usrsctp_init()`. This function
//! calls `sctp_init()` and reserves the memory necessary to administer
//! the data transfer. The function prototype is
//!
//! ```c
//! void usrsctp_init(uint16_t udp_port)
//! ```
//!
//! As it is not always possible to send data directly over SCTP because
//! not all NAT boxes can process SCTP packets, the data can be sent over
//! UDP. To encapsulate SCTP into UDP a UDP port has to be specified, to
//! which the datagrams can be sent. This local UDP port  is set with
//! the parameter `udp_port`. The default value is `9899`, the standard
//! UDP encapsulation port. If UDP encapsulation is not necessary, the
//! UDP port has to be set to `0`.
//!
//! ### usrsctp_finish()
//!
//! At the end of the program `usrsctp_finish()` should be called to
//! free all the memory that has been allocated before. The function
//! prototype is
//!
//! ```c
//! int usrsctp_finish(void)
//! ```
//!
//! The return code is 0 on success and -1 in case of an error.

#![allow(non_camel_case_types)]

extern crate libc;

mod bindings;

pub use bindings::*;
