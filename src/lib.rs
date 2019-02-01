// CITA
// Copyright 2016-2019 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
#![allow(unused_imports)]
#![allow(unused_results)]
#![feature(try_from)]

#[macro_use]
extern crate bincode;
#[macro_use]
extern crate crossbeam;
extern crate crypto_hash;
extern crate lru_cache;
extern crate min_max_heap;
extern crate protobuf;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate util;

pub mod algorithm;
pub mod params;
pub mod timer;
pub mod voteset;
pub mod wal;

pub type Address = Vec<u8>;
pub type Target = Vec<u8>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum VoteType {
    Prevote = 0,
    PreCommit = 1,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Proposal {
    height: usize,
    round: usize,
    content: Target, // block hash
    lock_status: Option<Lock_stat>,
    proposer: Address,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LockStatus {
    proposal: Target, // block hash
    round: usize,
    votes: Vec<Vote>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vote {
    vote_type: VoteType,
    height: usize,
    round: usize,
    proposal: Target, // block hash
    voter: Address,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Feed {
    height: usize,
    proposal: Target, // block hash
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Commit {
    height: usize,
    proposal: Target, // block hash
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Status {
    height: usize,
    interval: u64,
    authority_list: Vec<Address>,
}
