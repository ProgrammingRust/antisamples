# *Programming Rust* Antisamples

This repository contains some tests for the code in our book, *Programming Rust*.

These tests are a little unusual: none of them compile.
In fact, if any do compile and run, it's a bug!
These are the tests for code samples in our book that are there to illustrate errors.
The tests check that `rustc` agrees they are erroneous.
They also help us notice when a new `rustc` version has different error messages,
so we can update the book.

We're publishing these so that you can copy and paste our anti-examples into
[play.rust-lang.org](https://play.rust-lang.org)
and tinker with them.
(You could also copy them out of the book,
but often what's in the book is just a fragment, not a complete program.)

Whenever you see a compiler error message in the book:

> ```rust
> use std::thread;
>
> fn start_sorting_thread(mut cities: Vec<City>, stat: Statistic)
>     -> thread::JoinHandle<Vec<City>>
> {
>     let key_fn = |city: &City| -> i64 { -city.get_statistic(stat) };
>
>     thread::spawn(|| {
>         cities.sort_by_key(key_fn);
>         cities
>     })
> }
> ```
>
> Again, the closure `key_fn` contains a reference to `stat`.
> But this time, Rust can't guarantee that the reference is used safely.
> Rust therefore rejects this program:
>
> ```console
> error[E0373]: closure may outlive the current function, but it borrows `stat`,
>               which is owned by the current function
>   --> closures_sort_thread.rs:33:18
>    |
> 33 | let key_fn = |city: &City| -> i64 { -city.get_statistic(stat) };
>    |              ^^^^^^^^^^^^^^^^^^^^                       ^^^^
>    |              |                                      `stat` is borrowed here
>    |              may outlive borrowed value `stat`
> ```

you'll find the filename in the error message (`rustc` prints it right after an arrow, `-->`):

> ```console
>   --> closures_sort_thread.rs:33:18
> ```

The source file will be under either [tests/compile-fail](tests/compile-fail)
or (rarely) [tests/parse-fail](tests/parse-fail).

In this case, it's
[tests/compile-fail/closures_sort_thread.rs](tests/compile-fail/closures_sort_thread.rs).

Happy hacking!

—@jimblandy & @jorendorff


## How to run<sup>*</sup> these tests

<sup>*</sup>(of course they do not actually "run", as such)

You probably don't care about this unless you're working on the book.
These tests use the `compiletest_rs` crate, which requires Nightly Rust.
To set up Nightly Rust:

```console
$ rustup install nightly
```

To run the tests:

```console
$ cargo test
```

## License

The example code in this directory and its subdirectories is licensed under the
terms of the MIT license. See [LICENSE-MIT](LICENSE-MIT) for details.
