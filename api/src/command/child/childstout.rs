use std::{io::{self, BufRead, BufReader, Read}, process::ChildStdout};

use common::iterator::NextReturn;
use rquickjs::{atom::PredefinedAtom, class::{ClassId, JsClass, OwnedBorrowMut, Trace, Writable}, Class, Ctx, Function, IntoJs, Object, Value};

pub struct JsChildStdout{
    pub v: Option<BufReader<ChildStdout>>,
}
impl From<ChildStdout> for JsChildStdout {
    fn from(value: ChildStdout) -> Self {
        Self { v: Some(BufReader::new(value)) }
    }
}
impl<'js> Trace<'js> for JsChildStdout {
    fn trace<'a>(&self, _: rquickjs::class::Tracer<'a, 'js>) {}
}
impl<'js> IntoJs<'js> for JsChildStdout
{
    fn into_js(self, ctx: &Ctx<'js>) -> rquickjs::Result<Value<'js>> {
        Class::instance(ctx.clone(), self).into_js(ctx)
    }
}
impl<'js> JsClass<'js> for JsChildStdout {
    const NAME: &'static str = "ChildStdout";

    type Mutable = Writable;

    fn class_id() -> &'static rquickjs::class::ClassId {
        static ID: ClassId = ClassId::new();
        &ID
    }

    fn prototype(ctx: &rquickjs::Ctx<'js>) -> rquickjs::Result<Option<rquickjs::Object<'js>>> {
        let proto = Object::new(ctx.clone()).unwrap();
        // super::super::proto_fn!(proto, read);
        let read = rquickjs::Function::new(proto.ctx().clone(),read).unwrap();
        proto.set("read",read).unwrap();

        let iter = Function::new(ctx.clone(), iterator).unwrap();
        proto.set(PredefinedAtom::SymbolIterator, iter).unwrap();

        let next = Function::new(ctx.clone(), next).unwrap();
        proto.set(PredefinedAtom::Next, next).unwrap();
        return Ok(Some(proto));
    }

    fn constructor(_: &rquickjs::Ctx<'js>) -> rquickjs::Result<Option<rquickjs::function::Constructor<'js>>> {
        Ok(None)
    }
}

pub static NONE_MESSAGE: &str = "this stdout is already given up.";
type This<'js> = rquickjs::prelude::This<OwnedBorrowMut<'js, JsChildStdout>>;
pub fn read(mut this: This) -> io::Result<String> {
    let Some(csout) = &mut this.v else { panic!("{}",NONE_MESSAGE) };
    let mut buffer = String::with_capacity(80);
    csout.read_to_string(&mut buffer)?;
    return Ok(buffer);
}
pub fn iterator(this: This) -> Class<JsChildStdout> {
    this.0.into_inner()
}
pub fn next(mut this: This) -> NextReturn<String> {
    let Some(csout) = &mut this.v else { panic!("{}",NONE_MESSAGE) };
    let mut buffer = String::with_capacity(80);
    let n = csout.read_line(&mut buffer).unwrap();
    if n == 0 {
        return NextReturn::none();
    }
    else {
        return NextReturn::some(buffer);
    }
}
