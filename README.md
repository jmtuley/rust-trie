# rust-trie

This is my first project in [Rust](rust-lang.org). **Constructive, kind criticism that will help me to learn is welcomed.**

## Description

This library implements a [trie](https://en.wikipedia.org/wiki/Trie), abstracted with generic types for both keys and values. Internally, a `std::collections::hash_map::HashMap` is used in each node to store the branches, so the key type must implement the traits `Eq`, `Clone`, and `std::hash::Hash`. Currently, the value must also implement `Clone`, though I'm looking for ways around that.

## Usage

* Construct a new `Trie` with
  ```rust
	let mut t = Trie::new();
  ```
* Insert a node for prefix `(1, 2, 3)` with
  ```rust
	t.insert(vec![1,2,3], "example value");
  ```
  Note that there must not already be a value with that key. However, you may insert into preexisting, but un-valued, keys:
  ```rust
	t.insert(vec![1,2,3], "example value");
	t.insert(vec![1,2,3], "new value");    // PANIC!
	t.insert(vec![1,2], "prefix's value"); // OK!
  ```
* Fetch the value for a given prefix with
  ```rust
	// assume t from previous example
	t.fetch(vec![1,2,3]); // -> Some("example value")
	t.fetch(vec![1]); // -> None
	t.fetch(vec![1,4]); // PANIC!
  ```

## Improvements
* Better ergonomics around specifying keys; something like variadic parameters (or maybe taking slices to avoid `vec!` everywhere)
* Return `None` instead of panicking when `fetch`ing for nonexistent keys
* Be more idiomatic as I learn Rust's idioms!
