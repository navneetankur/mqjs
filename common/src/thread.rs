use std::thread::JoinHandle;

use rquickjs::{class::{ClassId, JsClass, OwnedBorrowMut, Trace, Writable}, Class, Ctx, IntoJs, Value};

pub struct JsJoinHandle<T: Send + 'static> {
    pub v: Option<JoinHandle<T>>,
    pub channel: JsChannel,
}

impl<T: Send + 'static> JsJoinHandle<T> {
    #[must_use]
    pub fn new(v: Option<JoinHandle<T>>, receiver: Option<async_channel::Receiver<String>>, sender: Option<async_channel::Sender<String>>) -> Self {
        Self { v, channel: JsChannel::new(receiver, sender) }
    }
}
impl<'js, T: Send + 'static> Trace<'js> for JsJoinHandle<T> {
    fn trace<'a>(&self, _: rquickjs::class::Tracer<'a, 'js>) {}
}
impl<'js, T: Send + 'static> IntoJs<'js> for JsJoinHandle<T> {
    fn into_js(self, ctx: &rquickjs::Ctx<'js>) -> rquickjs::Result<rquickjs::Value<'js>> {
        Class::instance(ctx.clone(), self).into_js(ctx)
    }
}
impl<'js, T: Send + 'static> JsClass<'js> for JsJoinHandle<T> {
    const NAME: &'static str = "ThreadJoinHandle";

    type Mutable = Writable;

    fn class_id() -> &'static rquickjs::class::ClassId {
        static ID: ClassId = ClassId::new();
        &ID
    }

    fn prototype(_ctx: &rquickjs::Ctx<'js>) -> rquickjs::Result<Option<rquickjs::Object<'js>>> {
        todo!()
    }

    fn constructor(_: &rquickjs::Ctx<'js>) -> rquickjs::Result<Option<rquickjs::function::Constructor<'js>>> {
        Ok(None)
    }
}
type This<'js, T> = rquickjs::function::This<OwnedBorrowMut<'js, JsJoinHandle<T>>>;
static NONE_MESSAGE: &str = "This handle is already given up.";
fn join<'js, T: Send + IntoJs<'js> + 'static>(mut this: This<'js, T>) -> T {
    let Some(handle) = this.v.take() else { panic!("{}", NONE_MESSAGE) };
    return handle.join().unwrap();
}
fn is_finished<'js, T: Send + 'static>(this: This<'js, T>) -> bool {
    let Some(handle) = &this.v else { panic!("{}", NONE_MESSAGE) };
    handle.is_finished()
}
fn unpark<'js, T: Send + 'static>(mut this: This<'js, T>) {
    let Some(handle) = &this.v else { panic!("{}", NONE_MESSAGE) };
    handle.thread().unpark();
}

#[rquickjs::class]
pub struct JsChannel {
    pub receiver: Option<async_channel::Receiver<String>>,
    pub sender: Option<async_channel::Sender<String>>,
}
impl<'js> Trace<'js> for JsChannel {
    fn trace<'a>(&self, _: rquickjs::class::Tracer<'a, 'js>) {}
}
impl JsChannel {
    pub fn new(receiver: Option<async_channel::Receiver<String>>, sender: Option<async_channel::Sender<String>>) -> Self {
        Self { receiver, sender }
    }
}
static NONE_CHANNEL: &str = "Channel has not been setup.";
static NO_SERIAL: &str = "Message cannot be serialized. So it can't be sent between threads.";
static NO_DESERIAL: &str = "Message cannot be serialized. So it can't be sent between threads.";

#[rquickjs::methods]
impl JsChannel {
    pub async fn send<'js>(&self, ctx: Ctx<'js>, value: Value<'js> ) {
        let Some(sender) = &self.sender else { panic!("{}", NONE_CHANNEL) };
        let Ok(message) = ctx.json_stringify(value) else {panic!("{}", NO_SERIAL)};
        let Some(message) = message else {panic!("{}", NO_SERIAL)};
        sender.send(message.to_string().unwrap()).await.unwrap();
    }
    //receiver has to be iterator.
}
