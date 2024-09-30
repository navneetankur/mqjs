use rquickjs::{class::{ClassId, JsClass, OwnedBorrowMut, Trace, Writable}, prelude::Async, Class, Ctx, Function, IntoJs, Object, Value};

use crate::iterator::NextReturn;

pub struct JsChannel {
    pub receiver: Option<async_channel::Receiver<String>>,
    pub sender: Option<async_channel::Sender<String>>,
}
impl<'js> Trace<'js> for JsChannel {
    fn trace<'a>(&self, _: rquickjs::class::Tracer<'a, 'js>) {}
}
impl JsChannel {
    #[must_use]
    pub fn new(receiver: Option<async_channel::Receiver<String>>, sender: Option<async_channel::Sender<String>>) -> Self {
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

        let func = Function::new(ctx.clone(), Async(send)).unwrap();
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
static NONE_CHANNEL: &str = "Channel has not been setup.";
static NO_SERIAL: &str = "Message cannot be serialized. So it can't be sent between threads.";
static NO_DESERIAL: &str = "Message cannot be serialized. So it can't be sent between threads.";
type This<'js> = rquickjs::function::This<OwnedBorrowMut<'js, JsChannel>>;
async fn send<'js>(this: This<'js>, ctx: Ctx<'js>, value: Value<'js> ) {
    let Some(sender) = &this.sender else { panic!("{}", NONE_CHANNEL) };
    let Ok(message) = ctx.json_stringify(value) else {panic!("{}", NO_SERIAL)};
    let Some(message) = message else {panic!("{}", NO_SERIAL)};
    sender.send(message.to_string().unwrap()).await.unwrap();
}
fn async_iterator(this: This) -> Class<JsChannel> {
    this.0.into_inner()
}
async fn next<'js>(ctx: Ctx<'js>, this: This<'js>) -> NextReturn<Value<'js>> {
    let Some(receiver) = &this.receiver else { panic!("{}", NONE_CHANNEL) };
    if let Ok(message) = receiver.recv().await {
        let Ok(value) = ctx.json_parse(message) 
            else {panic!("{}", NO_DESERIAL)};
        return NextReturn::some(value);
    } else { return NextReturn::none(); }
}
