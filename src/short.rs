use ArgKind;
use OptError;

// wrapper trait for handled and unhandled short opts
trait ShortOpt {
    fn short_name(&self) -> &char;
    fn argkind(&self) -> &ArgKind;
    fn handle_err(&mut self, &str) -> Option<String>;
}

struct ShortUnhandled {
    short_name: char,
    argkind: ArgKind
}

impl ShortUnhandled {
    fn new(name: char, arg: ArgKind) -> ShortUnhandled {
        ShortUnhandled {
            short_name: name,
            argkind: arg
        }
    }
}

struct ShortHandled<'t, T: 't> {
    short_name: char,
    argkind: ArgKind,
    target: &'t mut T,
    handle: &'t mut FnMut(&str) -> Result<T, String>
}

impl<'t, T: 't> ShortHandled<'t, T> {
    fn new(name: char, arg: ArgKind, target: &'t mut T, handle: &'t mut FnMut(&str) -> Result<T, String>) -> ShortHandled<'t, T> {
        ShortHandled {
            short_name: name,
            argkind: arg,
            target: target,
            handle: handle
        }
    }
}

impl ShortOpt for ShortUnhandled {
    fn short_name(&self) -> &char { &self.short_name }
    fn argkind(&self) -> &ArgKind { &self.argkind }
    fn handle_err(&mut self, _: &str) -> Option<String> { None }
}

// if handle is successful, assign target
impl<'t, T: 't> ShortOpt for ShortHandled<'t, T> {
    fn short_name(&self) -> &char { &self.short_name }
    fn argkind(&self) -> &ArgKind { &self.argkind }
    fn handle_err(&mut self, arg: &str) -> Option<String> {
        match (self.handle)(arg) {
            Ok(v) => {
                *self.target = v;
                None
            }

            Err(s) => Some(s)
        }
    }
}

pub struct ShortOptions<'t> {
    opts: Vec<Box<ShortOpt + 't>>
}

impl<'t> ShortOptions<'t> {
    // iterate over constructors, assembling handled or unhandled variants
    pub fn new<T: 't, C: IntoIterator<Item=(char, ArgKind, Option<(&'t mut T, &'t mut FnMut(&str) -> Result<T, String>)>)>> (constructors: C) -> ShortOptions<'t> {
        let mut opts = Vec::new();
        let mut boxed: Box<ShortOpt>;

        for (name, arg, handle_opt) in constructors.into_iter() {
            if let Some((target, func)) = handle_opt {
                boxed = Box::new(ShortHandled::new(name, arg, target, func));
            } else {
                boxed = Box::new(ShortUnhandled::new(name, arg));
            }

            opts.push(boxed);
        }

        ShortOptions { opts: opts }
    }
}

pub enum PassedShort<'p> {
    Free(&'p str),
    Opt(char, Result<Option<&'p str>, OptError>)
}