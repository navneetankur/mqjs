use rquickjs::{class::{ClassId, JsClass, Trace, Writable}, Class, Ctx, IntoJs, Value};
#[derive(Trace, Clone)]
pub struct RustData {
    pub module_byte: Vec<u8>,
}

impl RustData {
    #[must_use]
    pub fn new(module_byte: Vec<u8>) -> Self {
        Self { module_byte }
    }
}
impl<'js> IntoJs<'js> for RustData
{
    fn into_js(self, ctx: &Ctx<'js>) -> rquickjs::Result<Value<'js>> {
        Class::instance(ctx.clone(), self).into_js(ctx)
    }
}
impl<'js> JsClass<'js> for RustData {
    const NAME: &'static str = "RustData";

    type Mutable = Writable;

    fn class_id() -> &'static rquickjs::class::ClassId {
        static ID: ClassId = ClassId::new();
        &ID
    }

    fn prototype(_: &rquickjs::Ctx<'js>) -> rquickjs::Result<Option<rquickjs::Object<'js>>> {
        Ok(None)
    }

    fn constructor(_: &rquickjs::Ctx<'js>) -> rquickjs::Result<Option<rquickjs::function::Constructor<'js>>> {
        Ok(None)
    }
}
