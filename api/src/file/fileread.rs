use std::{fs::File, io::{BufRead, BufReader, Read}};

use common::iterator::NextReturn;
use rquickjs::{atom::PredefinedAtom, class::{ClassId, JsClass, OwnedBorrowMut, Trace, Writable}, prelude::This, Class, Function, IntoJs, Object};

pub static NONE_MESSAGE: &str = "This file handle has already been given up.";
pub struct FileRead {
    pub v: Option<BufReader<File>>,
}
impl From<File> for FileRead {
    fn from(value: File) -> Self {
        Self { v: Some(BufReader::new(value)) }
    }
}
impl<'js> Trace<'js> for FileRead {
    fn trace<'a>(&self, _: rquickjs::class::Tracer<'a, 'js>) {}
}
impl<'js> IntoJs<'js> for FileRead {
    fn into_js(self, ctx: &rquickjs::Ctx<'js>) -> rquickjs::Result<rquickjs::Value<'js>> {
        Class::instance(ctx.clone(), self).into_js(ctx)
    }
}
impl<'js> JsClass<'js> for FileRead {
    const NAME: &'static str = "FileRead";

    type Mutable = Writable;

    fn class_id() -> &'static rquickjs::class::ClassId {
        static ID: ClassId = ClassId::new();
        &ID
    }

    fn prototype(ctx: &rquickjs::Ctx<'js>) -> rquickjs::Result<Option<rquickjs::Object<'js>>> {
        let proto = Object::new(ctx.clone()).unwrap();
        let iter = Function::new(ctx.clone(), iterator).unwrap();
        proto.set(PredefinedAtom::SymbolIterator, iter).unwrap();

        let next = Function::new(ctx.clone(), next).unwrap();
        proto.set(PredefinedAtom::Next, next).unwrap();

        let read = Function::new(ctx.clone(), read).unwrap();
        proto.set("read", read).unwrap();

        return Ok(Some(proto));
    }

    fn constructor(_: &rquickjs::Ctx<'js>) -> rquickjs::Result<Option<rquickjs::function::Constructor<'js>>> {
        Ok(None)
    }
}
#[rquickjs::function]
pub fn openr(filepath: String) -> Option<FileRead> {
    let file = File::open(filepath).ok()?;
    return Some(file.into());
}
fn iterator(this: This<OwnedBorrowMut<FileRead>>) -> Class<FileRead> {
    this.0.into_inner() 
}
fn next(mut this: This<OwnedBorrowMut<FileRead>>) -> NextReturn<String> {
    let Some(file) = &mut this.v else { panic!("{}", NONE_MESSAGE); };
    let mut buffer = String::with_capacity(80);
    let n = file.read_line(&mut buffer).unwrap();
    if n!=0 {
        return NextReturn::some(buffer);
    }
    else {
        return NextReturn::none();
    }
}
fn read(mut this: This<OwnedBorrowMut<FileRead>>) -> String {
    let Some(file) = &mut this.v else { panic!("{}", NONE_MESSAGE); };
    let mut buffer = String::with_capacity(1024);
    file.read_to_string(&mut buffer).unwrap();
    return buffer;
}
