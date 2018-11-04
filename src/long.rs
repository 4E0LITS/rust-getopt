use ArgKind;
use OptError;

// wrapper trait for handled and unhandled long opts
trait LongOpt {
    fn short_name(&self) -> &char;
    fn long_name(&self) -> &str;
    fn argkind(&self) -> &ArgKind;
    fn handle_err(&mut self, &str) -> Option<String>;
}

struct LongUnhandled {
    short_name: char,
    long_name: &'static str,
    argkind: ArgKind
}

impl LongUnhandled {
    fn new(shortname: char, longname: &'static str, argkind: ArgKind) -> LongUnhandled {
        LongUnhandled {
            short_name: shortname,
            long_name: longname,
            argkind: argkind
        }
    }
}

struct LongHandled<'t, T: 't> {
    short_name: char,
    long_name: &'static str,
    argkind: ArgKind,
    target: &'t mut T,
    handle: &'t mut FnMut(&str) -> Result<T, String>
}

impl<'t, T: 't> LongHandled<'t, T> {
    fn new(shortname: char, longname: &'static str, argkind: ArgKind, target: &'t mut T, handle: &'t mut FnMut(&str) -> Result<T, String>) -> LongHandled<'t, T> {
        LongHandled {
            short_name: shortname,
            long_name: longname,
            argkind: argkind,
            target: target,
            handle: handle
        }
    }
}

impl LongOpt for LongUnhandled {
    fn short_name(&self) -> &char { &self.short_name }
    fn long_name(&self) -> &str { &self.long_name }
    fn argkind(&self) -> &ArgKind { &self.argkind }
    fn handle_err(&mut self, _: &str) -> Option<String> { None }
}

// if handle is successful, assign target
impl<'t, T: 't> LongOpt for LongHandled<'t, T> {
    fn short_name(&self) -> &char { &self.short_name }
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

pub struct LongOptions<'t> {
    opts: Vec<Box<LongOpt + 't>>
}

impl<'t> LongOptions<'t> {
    // iterate over constructors, assembling handled or unhandled variants
    pub fn new<T: 't, C: IntoIterator<Item=(char, &'static str, ArgKind, Option<(&'t mut T, &'t mut FnMut(&str) -> Result<T, String>)>)>> (constructors: C) -> LongOptions<'t> {
        let mut opts = Vec::new();
        let mut boxed: Box<LongOpt>;

        for (short, long, arg, handle_opt) in constructors.into_iter() {
            if let Some((target, func)) = handle_opt {
                boxed = Box::new(LongHandled::new(short, long, arg, target, func));
            } else {
                boxed = Box::new(LongUnhandled::new(short, long, arg));
            }

            opts.push(boxed);
        }

        LongOptions { opts: opts }
    }
}

pub enum LongName<'s> {
    Short(char),
    Long(&'s str)
}

pub enum PassedLong<'p> {
    Free(&'p str),
    Opt(LongName<'p>, Result<Option<&'p str>, OptError>)
}