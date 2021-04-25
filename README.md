# rust-glass

[![Travis Build Status][travis-badge]][travis-page]
[![Code Climate][codeclimate-badge]][codeclimate-page]

A [Glass][glass-homepage] compiler written in Rust.

Thanks to Mariko Kosaka for [this handy guide][compiler-guide] on how to be/make a compiler.

**NOTE** This version includes a `@` command which swaps the top 2 items from the stack.
Instead of `(_n)1=,` (which, according to the documentation, should *not* work), use `(_n) @=`

[glass-homepage]: https://esolangs.org/wiki/Glass
[compiler-guide]: https://medium.com/@kosamari/how-to-be-a-compiler-make-a-compiler-with-javascript-4a8a13d473b4

[travis-badge]: https://travis-ci.org/Cxarli/rust-glass.svg?branch=master
[travis-page]: https://travis-ci.org/Cxarli/rust-glass

[codeclimate-badge]: https://codeclimate.com/github/Cxarli/rust-glass/badges/gpa.svg
[codeclimate-page]: https://codeclimate.com/github/Cxarli/rust-glass
