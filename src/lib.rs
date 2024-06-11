// Copyright (C) 2024 Ryan Daum <ryan.daum@gmail.com>
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, version 3.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with
// this program. If not, see <https://www.gnu.org/licenses/>.
//

mod bitarray;
mod bitset;
mod slice_ref;

use std::cell::Cell;
use std::marker::PhantomData;
use std::sync::MutexGuard;

pub use bitarray::BitArray;
pub use bitset::{Bitset, Bitset16, Bitset32, Bitset64, Bitset8, BitsetTrait};
pub use slice_ref::{ByteSource, SliceRef};

/// A phantom type for explicitly marking types as !Sync
pub type PhantomUnsync = PhantomData<Cell<()>>;

/// A phantom type for explicitly marking types as !Send
pub type PhantomUnsend = PhantomData<MutexGuard<'static, ()>>;
