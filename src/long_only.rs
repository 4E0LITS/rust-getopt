use ArgKind;
use OptError;

// wrapper trait for handled and unhandled long only opts
pub trait LongOnlyOpt {
    fn long_name(&self) -> &str;
    fn argkind(&self) -> &ArgKind;
    fn handle_err(&mut self, &str) -> Option<String>;
}

struct LongOnlyUnhandled {
    long_name: &'static str,
    argkind: ArgKind
}

impl LongOnlyUnhandled {
    fn new(name: &'static str, argkind: ArgKind) -> LongOnlyUnhandled {
        LongOnlyUnhandled {
            long_name: name,
            argkind: argkind
        }
    }
}

struct LongOnlyHandled<'t, T: 't> {
    long_name: &'static str,
    argkind: ArgKind,
    target: &'t mut T,
    handle: &'t mut FnMut(&str) -> Result<T, String>
}

impl<'t, T: 't> LongOnlyHandled<'t, T> {
    fn new(name: &'static str, arg: ArgKind, target: &'t mut T, handle: &'t mut FnMut(&str) -> Result<T, String>) -> LongOnlyHandled<'t, T> {
        LongOnlyHandled {
            long_name: name,
            argkind: arg,
            target: target,
            handle: handle
        }
    }
}

impl LongOnlyOpt for LongOnlyUnhandled {
    fn long_name(&self) -> &str { &self.long_name }
    fn argkind(&self) -> &ArgKind { &self.argkind }
    fn handle_err(&mut self, _: &str) -> Option<String> { None }
}

// if handle is successful, assign target
impl<'t, T: 't> LongOnlyOpt for LongOnlyHandled<'t, T> {
    fn long_name(&self) -> &str { &self.long_name }
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

pub struct LongOnlyOptions<'t> {
    opts: Vec<Box<LongOnlyOpt + 't>>
}

impl<'t> LongOnlyOptions<'t> {
    // iterate over constructors, assembling handled or unhandled variants
    pub fn new<T: 't, C: IntoIterator<Item=(&'static str, ArgKind, Option<(&'t mut T, &'t mut FnMut(&str) -> Result<T, String>)>)>> (constructors: C) -> LongOnlyOptions<'t> {
        let mut opts = Vec::new();
        let mut boxed: Box<LongOnlyOpt>;

        for (name, arg, handle_opt) in constructors.into_iter() {
            if let Some((target, func)) = handle_opt {
                boxed = Box::new(LongOnlyHandled::new(name, arg, target, func));
            } else {
                boxed = Box::new(LongOnlyUnhandled::new(name, arg));
            }

            opts.push(boxed);
        }

        LongOnlyOptions { opts: opts }
    }
}

pub enum PassedLongOnly<'p> {
    Free(&'p str),
    Opt(&'p str, Result<Option<&'p str>, OptError>)
}