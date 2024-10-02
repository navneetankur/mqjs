macro_rules! class_chore {
    ($classname: ident, $proto: ident) => {

impl<'js> rquickjs::class::Trace<'js> for $classname {
    fn trace<'a>(&self, _: rquickjs::class::Tracer<'a, 'js>) {}
}
impl<'js> rquickjs::IntoJs<'js> for $classname
{
    fn into_js(self, ctx: &rquickjs::Ctx<'js>) -> rquickjs::Result<rquickjs::Value<'js>> {
        rquickjs::Class::instance(ctx.clone(), self).into_js(ctx)
    }
}
impl<'js> JsClass<'js> for $classname {
    const NAME: &'static str = stringify!($classname);

    type Mutable = rquickjs::class::Writable;

    fn class_id() -> &'static rquickjs::class::ClassId {
        use rquickjs::class::ClassId;
        static ID: ClassId = ClassId::new();
        &ID
    }

    fn prototype(ctx: &rquickjs::Ctx<'js>) -> rquickjs::Result<Option<rquickjs::Object<'js>>> {
        return Ok(Some($proto));
    }

    fn constructor(_: &rquickjs::Ctx<'js>) -> rquickjs::Result<Option<rquickjs::function::Constructor<'js>>> {
        Ok(None)
    }
}

    };
}
