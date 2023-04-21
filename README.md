This is a toy implementation of a storage engine in Rust.

Specifically, this is a Hash Index key-value store similar to Bitcask. These are explained
in *Chapter 3 -> Hash Indexes* section of Designing Data-Intensive Applications. (In my copy,
page 72).

NOTE: Tests and CLI boilerplate code has been been copied [from here](https://github.com/pingcap/talent-plan/blob/master/courses/rust/README.md).

#### Improvements for the storage engine:

Basic improvements for the storage engine itself that I can think of:
* I kept the Strings themselves as values in the in-memory (HashMap) index. As explained, in
Designing Data-Intensive Applications, byte offets of the files are usually kept as values.
The reason for this is to minimize the amount of data the HashMap contains to not overload
the RAM.
* It's not really crash-tolerant. For one, when writing to the file you'd want to use `fsync`
so the OS does not buffer the writes. There are other edge cases for which it's not really
durable. 
* It is currently only single-threaded.
* I used a very basic encoding/decoding but in real-life implementations they obviously use
more advanced encodings.

The code can easily be extended to implement SSTables/LSM-Trees. You'd need to use a tree 
instead of a HashMap and keep multiple files and perform the compaction together.

#### Improvements for the code:

I didn't really spend time refactoring so fair amount of space for code improvement:

* Error handling is really kept to a minimum.
* Some of the code can be written more compactly (probably the nested match statements for one).
* There is a bit of repetition with the `rm` and `set` functions so could probably be nicely
combined.
