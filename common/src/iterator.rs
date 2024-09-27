use core::any::type_name;
use rquickjs::{atom::PredefinedAtom, class::{ClassId, JsClass, OwnedBorrowMut, Trace, Writable}, prelude::This, Class, Ctx, FromJs, Function, IntoJs, Object, Value};

#[derive(Clone)]
pub struct JsIterator <T: Iterator> {
    v: T,
}

impl<T: Iterator> JsIterator<T> {
    pub fn new(v: T) -> Self {
        Self { v }
    }
}
impl<'js, T: Iterator + Clone + 'js> IntoJs<'js> for JsIterator<T>
where
    T::Item: IntoJs<'js>
{
    fn into_js(self, ctx: &Ctx<'js>) -> rquickjs::Result<Value<'js>> {
        Class::instance(ctx.clone(), self).into_js(ctx)
    }
}
impl<'js, T: Iterator + Clone + 'js> FromJs<'js> for JsIterator<T>
where
    T::Item: IntoJs<'js>
{
    fn from_js(_: &Ctx<'js>, value: Value<'js>) -> rquickjs::Result<Self> {
        println!("fjs");
        Ok(Class::<Self>::from_value(&value)?.try_borrow()?.clone())
    }
}
impl<'js, T: Iterator> Trace<'js> for JsIterator<T> {
    fn trace<'a>(&self, _tracer: rquickjs::class::Tracer<'a, 'js>) {}
}

impl<'js, T: Iterator + Clone + 'js> JsClass<'js> for JsIterator<T>
where
    T::Item: IntoJs<'js>
{
    const NAME: &'static str = type_name::<Self>();

    type Mutable = Writable;

    fn class_id() -> &'static rquickjs::class::ClassId {
        static ID: ClassId = ClassId::new();
        &ID
    }

    fn prototype(ctx: &rquickjs::Ctx<'js>) -> rquickjs::Result<Option<rquickjs::Object<'js>>> {
        let proto = Object::new(ctx.clone())?;
        let next = |mut this: This<OwnedBorrowMut<'js, Self>>| {
            if let Some(value) = this.v.next() {
                return NextReturn::some(value);
            }
            else {
                return NextReturn::none();
            }
        };
        proto.set(PredefinedAtom::Next, Function::new(ctx.clone(), next).unwrap()).unwrap();
        proto.set(PredefinedAtom::SymbolIterator, Function::new(ctx.clone(), 
                |this: This<Class<'js, Self>>| {
                this.0
        }).unwrap()).unwrap();
        return Ok(Some(proto));
    }

    fn constructor(_: &rquickjs::Ctx<'js>) -> rquickjs::Result<Option<rquickjs::function::Constructor<'js>>> {
        Ok(None)
    }
}

pub struct NextReturn<T> {
    value: Option<T>,
}

impl<T> NextReturn<T> {
    pub fn new(value: Option<T>) -> Self {
        Self { value }
    }
    pub fn some(value: T) -> Self {
        Self::new(Some(value))
    }
    pub fn none() -> Self {
        Self::new(None)
    }

}

impl<'js, T: IntoJs<'js>> IntoJs<'js> for NextReturn<T> {
    fn into_js(self, ctx: &Ctx<'js>) -> rquickjs::Result<Value<'js>> {
        let obj = Object::new(ctx.clone())?;
        if let Some(value) = self.value {
            obj.set(PredefinedAtom::Done, false)?;
            obj.set(PredefinedAtom::Value, value)?;
        } else {
            obj.set(PredefinedAtom::Done, true)?;
        }
        return Ok(obj.into_value());
    }
}

#[cfg(test)]
mod tests {
    use rquickjs::{CatchResultExt, Context, Value};

    use crate::add_global_fn;

    use super::JsIterator;

    #[test]
    fn try_iter() {
        let v = [1, 2, 3, 4];
        let vi = v.into_iter();
        let jvi = JsIterator::new(vi);
        let rt = rquickjs::Runtime::new().unwrap();
        let context = Context::full(&rt).unwrap();
        let js = r#"
            var out = [];
            for (let vi of viter) {
                out.push(vi);
            }
        "#;
        context.with(|mut ctx| {
            add_global_fn(&mut ctx);
            let globals = ctx.globals();
            globals.set("viter", jvi).unwrap();
            ctx.eval::<Value, _>(js).catch(&ctx).unwrap();
            let out = globals.get::<_, rquickjs::Array>("out").unwrap();
            let mut i = 1;
            for val in out.iter::<i32>() {
                assert_eq!(i, val.unwrap());
                i += 1;
            }
        })
    }

}
