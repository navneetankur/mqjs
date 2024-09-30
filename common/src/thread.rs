mod channel;
pub use channel::JsChannel;
use std::thread::JoinHandle;

use rquickjs::{class::{ClassId, JsClass, OwnedBorrowMut, Trace, Writable}, Class, Ctx, Function, IntoJs, Object, Value};

pub struct JsJoinHandle {
    pub v: Option<JoinHandle<Option<String>>>,
    pub channel: JsChannel,
}

impl JsJoinHandle {
    #[must_use]
    pub fn new(v: Option<JoinHandle<Option<String>>>, receiver: Option<async_channel::Receiver<String>>, sender: Option<async_channel::Sender<String>>) -> Self {
        Self { v, channel: JsChannel::new(receiver, sender) }
    }
}
impl<'js> Trace<'js> for JsJoinHandle {
    fn trace<'a>(&self, _: rquickjs::class::Tracer<'a, 'js>) {}
}
impl<'js> IntoJs<'js> for JsJoinHandle {
    fn into_js(self, ctx: &rquickjs::Ctx<'js>) -> rquickjs::Result<rquickjs::Value<'js>> {
        Class::instance(ctx.clone(), self).into_js(ctx)
    }
}
impl<'js> JsClass<'js> for JsJoinHandle {
    const NAME: &'static str = "ThreadJoinHandle";

    type Mutable = Writable;

    fn class_id() -> &'static rquickjs::class::ClassId {
        static ID: ClassId = ClassId::new();
        &ID
    }

    fn prototype(ctx: &rquickjs::Ctx<'js>) -> rquickjs::Result<Option<rquickjs::Object<'js>>> {
        let proto = Object::new(ctx.clone()).unwrap();

        let func = Function::new(ctx.clone(), join).unwrap();
        proto.set("join", func).unwrap();

        let func = Function::new(ctx.clone(), is_finished).unwrap();
        proto.set("is_finished", func).unwrap();

        let func = Function::new(ctx.clone(), unpark).unwrap();
        proto.set("unpark", func).unwrap();

        return Ok(Some(proto));
    }

    fn constructor(_: &rquickjs::Ctx<'js>) -> rquickjs::Result<Option<rquickjs::function::Constructor<'js>>> {
        Ok(None)
    }
}
type This<'js> = rquickjs::function::This<OwnedBorrowMut<'js, JsJoinHandle>>;
static NONE_MESSAGE: &str = "This handle is already given up.";
#[allow(clippy::needless_pass_by_value)]
fn join<'js>(ctx: Ctx<'js>, mut this: This<'js>) -> Value<'js> {
    let Some(handle) = this.v.take() else { panic!("{}", NONE_MESSAGE) };
    let Some(rusty) =  handle.join().unwrap() else {
        return Value::new_undefined(ctx.clone());
    };
    let value = ctx.json_parse(rusty).unwrap();
    return value;
}
#[allow(clippy::needless_pass_by_value)]
fn is_finished(this: This<'_>) -> bool {
    let Some(handle) = &this.v else { panic!("{}", NONE_MESSAGE) };
    handle.is_finished()
}
#[allow(clippy::needless_pass_by_value)]
fn unpark(this: This<'_>) {
    let Some(handle) = &this.v else { panic!("{}", NONE_MESSAGE) };
    handle.thread().unpark();
}

