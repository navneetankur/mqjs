use futures_channel::mpsc;
use futures_lite::StreamExt;
use rquickjs::{class::{ClassId, JsClass, OwnedBorrowMut, Trace, Writable}, prelude::Async, Class, Ctx, Function, IntoJs, Object, Value};

use crate::iterator::NextReturn;

pub struct JsChannel {
    pub receiver: mpsc::UnboundedReceiver<String>,
    pub sender: mpsc::UnboundedSender<String>,
}
impl<'js> Trace<'js> for JsChannel {
    fn trace<'a>(&self, _: rquickjs::class::Tracer<'a, 'js>) {}
}
impl JsChannel {
    #[must_use]
    pub fn new(receiver: mpsc::UnboundedReceiver<String>, sender: mpsc::UnboundedSender<String>) -> Self {
        Self { receiver, sender }
    }
}
impl<'js> IntoJs<'js> for JsChannel
{
    fn into_js(self, ctx: &Ctx<'js>) -> rquickjs::Result<Value<'js>> {
        Class::instance(ctx.clone(), self).into_js(ctx)
    }
}
impl<'js> JsClass<'js> for JsChannel {
    const NAME: &'static str = "JsChannel";

    type Mutable = Writable;

    fn class_id() -> &'static rquickjs::class::ClassId {
        static ID: ClassId = ClassId::new();
        &ID
    }

    fn prototype(ctx: &Ctx<'js>) -> rquickjs::Result<Option<rquickjs::Object<'js>>> {
        let proto = Object::new(ctx.clone()).unwrap();

        let func = Function::new(ctx.clone(), send).unwrap();
        proto.set("send", func).unwrap();

        let func = Function::new(ctx.clone(), async_iterator).unwrap();
        proto.set("Symbol.asyncIterator", func).unwrap();

        let func = Function::new(ctx.clone(), Async(next)).unwrap();
        proto.set("next", func).unwrap();

        return Ok(Some(proto));
    }

    fn constructor(_: &Ctx<'js>) -> rquickjs::Result<Option<rquickjs::function::Constructor<'js>>> {
        Ok(None)
    }
}
static NO_SERIAL: &str = "Message cannot be serialized. So it can't be sent between threads.";
static NO_DESERIAL: &str = "Message cannot be deserialized. So it can't be sent between threads.";
type This<'js> = rquickjs::function::This<OwnedBorrowMut<'js, JsChannel>>;
fn send<'js>(this: This<'js>, ctx: Ctx<'js>, value: Value<'js> ) -> bool {
    let Ok(message) = ctx.json_stringify(value) else {panic!("{}", NO_SERIAL)};
    let Some(message) = message else {panic!("{}", NO_SERIAL)};
    return this.sender.unbounded_send(message.to_string().unwrap()).is_ok();
}
fn async_iterator(this: This) -> Class<JsChannel> {
    this.0.into_inner()
}
async fn next<'js>(ctx: Ctx<'js>, mut this: This<'js>) -> NextReturn<Value<'js>> {
    if let Some(value) = this.receiver.next().await {
        let value = ctx.json_parse(value).expect(NO_DESERIAL);
        return NextReturn::some(value);
    }
    else {
        return NextReturn::none();
    }
}
