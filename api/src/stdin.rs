use std::io::{BufRead, Read, StdinLock};
use common::iterator::NextReturn;
use rquickjs::{atom::PredefinedAtom, class::{ClassId, JsClass, OwnedBorrowMut, Trace, Writable}, prelude::This, Class, Function, IntoJs, Object};

const READ: &str = "read";
const READLINE: &str = "readline";

pub struct JsStdin{
    v: StdinLock<'static>
}

impl JsStdin {
    pub fn new(v: StdinLock<'static>) -> Self {
        Self { v }
    }
}
impl<'js> IntoJs<'js> for JsStdin {
    fn into_js(self, ctx: &rquickjs::Ctx<'js>) -> rquickjs::Result<rquickjs::Value<'js>> {
        Class::instance(ctx.clone(), self).into_js(ctx)
    }
}
impl<'js> Trace<'js> for JsStdin {
    fn trace<'a>(&self, _: rquickjs::class::Tracer<'a, 'js>) {}
}
impl<'js> JsClass<'js> for JsStdin {
    const NAME: &'static str = "JsStdin";

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

