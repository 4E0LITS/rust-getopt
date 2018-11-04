Have you ever wanted an opt parsing crate for Rust where the parser is a parser, '-h' is an option, how you handle it is your own business, the API stays out of your face about it, and sheep are nervous? Look no futher, `getopt` for Rust is intended to be the equivalent of GNU C's getopt, only for Rust. The purpose of this crate is to provide adaptations of:
 * `getopt()`
 * `getopt_long()`
 * `getopt_long_only()`

One particular thing of note is that whereas C's getopt allows pre-set values to be loaded into variables through the use of `struct option` like so,

```{"help", 0, &hflag, 1}```

`getopt` allows these values to be computed on the fly, and allows them to be computed from an argument passed to the option.