# daumtils

This is a set of utilities I use across my Various Projects. Use at your own risk.

Many of these things duplicate functionality found in other more established crates, but for 
various reasons ended up rolling my own. You probably don't want these, but if you do, here they are.

GPLv3 license.

Roughly what's in here:

* `SliceRef` - A holder for slices and sub-slices and sub-sub-slices and so on, with pluggable storage backends.
        My attempt to make programming for zero-copy buffer management easier.
* `BitSet` - My own bitset implementation.
* `BitArray` - An array backed by a bitset to indicate presence/not-presence. Alternative to arrays/vectors of
        `Option<T>`
* `PhantomUnsync`/`PhantomUnsend` - Phantom types for marking types as !Sync/!Send
