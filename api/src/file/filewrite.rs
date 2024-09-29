use std::{fs::File, io::{Write}};

use rquickjs::{class::{ClassId, JsClass, OwnedBorrowMut, Trace, Writable}, prelude::This, Class, Function, IntoJs, Object};

pub static NONE_MESSAGE: &str = "This file handle has alwritey been given up.";
pub struct FileWrite {
    pub v: Option<File>,
}
impl From<File> for FileWrite {
    fn from(value: File) -> Self {
        Self { v: Some(value) }
    }
}
impl<'js> Trace<'js> for FileWrite {
    fn trace<'a>(&self, _: rquickjs::class::Tracer<'a, 'js>) {}
}
impl<'js> IntoJs<'js> for FileWrite {
    fn into_js(self, ctx: &rquickjs::Ctx<'js>) -> rquickjs::Result<rquickjs::Value<'js>> {
        Class::instance(ctx.clone(), self).into_js(ctx)
    }
}
impl<'js> JsClass<'js> for FileWrite {
    const NAME: &'static str = "FileWrite";

    type Mutable = Writable;

    fn class_id() -> &'static rquickjs::class::ClassId {
        static ID: ClassId = ClassId::new();
        &ID
    }

    fn prototype(ctx: &rquickjs::Ctx<'js>) -> rquickjs::Result<Option<rquickjs::Object<'js>>> {
        let proto = Object::new(ctx.clone()).unwrap();
        let write = Function::new(ctx.clone(), write).unwrap();
        proto.set("write", write).unwrap();
        return Ok(Some(proto));
    }

    fn constructor(_: &rquickjs::Ctx<'js>) -> rquickjs::Result<Option<rquickjs::function::Constructor<'js>>> {
        Ok(None)
    }
}
#[rquickjs::function]
pub fn openw(filepath: String) -> Option<FileWrite> {
    let file = File::create(filepath).ok()?;
    return Some(file.into());
}
#[allow(clippy::needless_pass_by_value)]
fn write(mut this: This<OwnedBorrowMut<FileWrite>>, data: String) {
    let Some(file) = &mut this.v else {panic!("{}", NONE_MESSAGE)};
    file.write_all(data.as_bytes()).unwrap();
}
