mod short;
mod long;
mod long_only;

/*
This crate is broken into 3 modules: short, long, and long_only, which each provide a slightly different implementation
of opt parsing. short provides implementation for short opts, long for long opts, and long_only for long_only opts.
Each of these modules follows the same convention, which is documented here rather than copied in each individual module.

Options have "handled" and "unhandled" variants. "handled" options allow for values to be computed and loaded into
variables. A classic use case of C's variable loading feature is setting a "help_flag" variable to 1 like so:

    {"help", 0, &help_flag, 1}

However, unlike C's feature of loading static values into variables, getopt for Rust allows values to be computed from
closures. These closures can be used to supply static values, as well as compute values from the option's user supplied
argument as a &str. Unhandled and Handled option variants in each module are enclosed by a wrapper Trait, which implements
the following method:

    handle_err(&mut self) -> Option<String>

the handle_err method of Opt traits always returns None for unhandled options, because they require no work to handle and
no errors can occur doing so. For handled options, the registered handler is supplied the user argument if any, else "",
and must return a Result<T, String>, If the handler returns Ok(T), then the registered target is assigned T. If it returns
Err(S), then handle_err will return Some(S) to the caller.

In order to construct collections of options, each module of getopt provides a Vec<Box<Opt>> wrapper struct, which
implements a constructor method `new(constructors)`. `constructors` must implement IntoIterator, where Item is a tuple
consisting of the desired option's name(s), ArgKind, and an Option<target, handle>. For each item in `constructors`,
this method will create either a Box<Handled> or Box<Unhandled> (depending whether a target and handle were supplied)
and append it to a Vec, at which point the wrapper struct is constructed around the complete Vec. 

Each module provides a single external function whose name begins with `getopt`, which accepts arguments to be parsed and
an Opt wrapper struct. The arguments will be parsed using the supplied options, and the function will yield an Iterator.
This Iterator will yield Enum instances which are either a free floating arg, or a passed option. If the item is a passed
option, it will contain the name the option was passed as, and a Result. This Result represents whether the option was
passed successfully, and will be unsuccessful if

 * the option passed was invalid (user passed an option which does not exist)
 * the option required an argument but did not receive one
 * an argument was supplied, but an error occurred during its handling

In case of failure, the Result will contain an error Enum which will represent one of the listed causes.

If the option was passed successfully, the Result will contain an Option<&str> which contains its argument, if any. Opts
which which require an argument can safely unwrap this Option, as it has already been checked. Opts that never accept an
arg should ignore this Option, as it will always be None.
*/

// an option's expectation of an argument.
pub enum ArgKind {
    Required,
    Optional,
    Nil
}

// reasons for option parsing failure
pub enum OptError {
    Invalid,
    ArgRequired,
    HandleError(String)
}