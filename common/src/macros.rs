/// class_chore!(class, get_proto)
#[macro_export]
macro_rules! class_chore {
    ($classname: ident, $get_proto: ident) => {

impl<'js> rquickjs::class::Trace<'js> for $classname {
    fn trace<'a>(&self, _: rquickjs::class::Tracer<'a, 'js>) {}
}
impl<'js> rquickjs::IntoJs<'js> for $classname
{
    fn into_js(self, ctx: &rquickjs::Ctx<'js>) -> rquickjs::Result<rquickjs::Value<'js>> {
        rquickjs::Class::instance(ctx.clone(), self).into_js(ctx)
    }
}
impl<'js> rquickjs::class::JsClass<'js> for $classname {
    const NAME: &'static str = stringify!($classname);

    type Mutable = rquickjs::class::Writable;

    fn class_id() -> &'static rquickjs::class::ClassId {
        use rquickjs::class::ClassId;
        static ID: ClassId = ClassId::new();
        &ID
    }

    fn prototype(ctx: &rquickjs::Ctx<'js>) -> rquickjs::Result<Option<rquickjs::Object<'js>>> {
        return Ok(Some($get_proto(ctx)));
    }

    fn constructor(_: &rquickjs::Ctx<'js>) -> rquickjs::Result<Option<rquickjs::function::Constructor<'js>>> {
        Ok(None)
    }
}

    };
}
/// object_fn!(object, func)
#[macro_export]
macro_rules! object_fn {
    ($object:ident, $($funcs:ident),+ $(,)?) => {
        let ctx = $object.ctx();
        $(
        let temp = rquickjs::Function::new(ctx.clone(), $funcs).unwrap();
        $object.set(stringify!($funcs), temp).unwrap();
        )+
    };
}
