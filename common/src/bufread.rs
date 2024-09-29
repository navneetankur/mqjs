use core::any::type_name;
use std::io::{BufRead};
use rquickjs::{atom::PredefinedAtom, class::{ClassId, JsClass, OwnedBorrowMut, Trace, Writable}, prelude::This, Class, Function, IntoJs, Object};

use crate::iterator::NextReturn;
const READ: &str = "read";
const READLINE: &str = "readline";

pub struct JsBufReader<B: BufRead> {
    v: B,
}
impl<'js, B: BufRead + 'static> IntoJs<'js> for JsBufReader<B> {
    fn into_js(self, ctx: &rquickjs::Ctx<'js>) -> rquickjs::Result<rquickjs::Value<'js>> {
        Class::instance(ctx.clone(), self).into_js(ctx)
    }
}

impl<B: BufRead> JsBufReader<B> {
    pub fn new(v: B) -> Self {
        Self { v }
    }
}
impl<'js, B: BufRead> Trace<'js> for JsBufReader<B>{
    fn trace<'a>(&self, _: rquickjs::class::Tracer<'a, 'js>) {}
}
impl<'js, B: BufRead + 'static> JsClass<'js> for JsBufReader<B> {
    const NAME: &'static str = type_name::<Self>();

    type Mutable = Writable;

    fn class_id() -> &'static rquickjs::class::ClassId {
        static ID: ClassId = ClassId::new();
        &ID
    }

    fn prototype(ctx: &rquickjs::Ctx<'js>) -> rquickjs::Result<Option<rquickjs::Object<'js>>> {
        let proto = Object::new(ctx.clone())?;
        let next = |mut this: This<OwnedBorrowMut<'js, Self>>| {
            let mut buffer = String::with_capacity(80);
            let value = this.v.read_line(&mut buffer).unwrap();
            if value > 0 {
                return NextReturn::some(buffer);
            } else {
                return NextReturn::none();
            }
        };
        proto.set(PredefinedAtom::Next, Function::new(ctx.clone(), next).unwrap()).unwrap();
        proto.set(PredefinedAtom::SymbolIterator, Function::new(ctx.clone(), 
                |this: This<Class<'js, Self>>| {
                this.0
        }).unwrap()).unwrap();

        let read = |mut this: This<OwnedBorrowMut<'js, Self>>| {
            let mut buffer = String::with_capacity(512);
            this.v.read_to_string(&mut buffer).unwrap();
            return buffer;
        };
        let read = Function::new(ctx.clone(), read).unwrap().with_name(READ).unwrap();
        proto.set(READ, read).unwrap();


        let readline = |mut this: This<OwnedBorrowMut<'js, Self>>| {
            let mut buffer = String::with_capacity(80);
            this.v.read_line(&mut buffer).unwrap();
            return buffer;
        };
        let readline = Function::new(ctx.clone(), readline).unwrap().with_name(READLINE).unwrap();
        proto.set(READLINE, readline).unwrap();
        return Ok(Some(proto));
    }

    fn constructor(_: &rquickjs::Ctx<'js>) -> rquickjs::Result<Option<rquickjs::function::Constructor<'js>>> {
        Ok(None)
    }
}
