
use futures_channel::oneshot;
use rquickjs::{atom::PredefinedAtom, class::{ClassId, JsClass, OwnedBorrowMut, Trace, Writable}, prelude::Rest, Class, Ctx, Function, IntoJs, Object, Value};

use super::JsChannel;

pub struct TaskJoin {
    receiver: Option<oneshot::Receiver<Option<String>>>,
    channel: Option<JsChannel>,
}

impl TaskJoin {
    #[must_use]
    pub fn new(receiver: Option<oneshot::Receiver<Option<String>>>, channel: Option<JsChannel>) -> Self {
        Self { receiver, channel }
    }
}
static RECIEVER_GONE: &str = "Reciever is missing, did you await twice?";
type This<'js> = rquickjs::function::This<OwnedBorrowMut<'js, TaskJoin>>;
#[allow(clippy::needless_pass_by_value)]
fn then<'js>(ctx: Ctx<'js>, mut this: This<'js>, resolve: Function<'js>, reject: Rest<Function<'js>>) {
    let receiver = this.receiver.take().expect(RECIEVER_GONE);
    let ctxf = ctx.clone();
    let future = async move {
        match receiver.await {
            Ok(result) => {
                if let Some(result) = result {
                    let result = ctxf.json_parse(result).expect("can't be deserialized");
                    resolve.call::<_,Value>((result,)).unwrap();
                }
                else {
                    resolve.call::<_,Value>(()).unwrap();
                }
            }
            Err(_) => {
                if let Some(reject) = reject.into_inner().drain(..).next() {
                    reject.call::<_,Value>(("cancelled",)).unwrap();
                }
            }
        }
    };
    ctx.spawn(future);
}
#[rquickjs::function]
fn channel<'js>(mut this: This<'js>) -> JsChannel {
    this.channel.take()
        .expect("Channel not present, already gone or never setup.")
}

impl<'js> Trace<'js> for TaskJoin {
    fn trace<'a>(&self, _: rquickjs::class::Tracer<'a, 'js>) {}
}
impl<'js> JsClass<'js> for TaskJoin {
    const NAME: &'static str = "TaskJoin";

    type Mutable = Writable;

    fn class_id() -> &'static rquickjs::class::ClassId {
        static ID: ClassId = ClassId::new();
        &ID
    }

    fn prototype(ctx: &rquickjs::Ctx<'js>) -> rquickjs::Result<Option<rquickjs::Object<'js>>> {
        let proto = Object::new(ctx.clone()).unwrap();
        let func = Function::new(ctx.clone(), then).unwrap();
        proto.set(PredefinedAtom::Then, func).unwrap();

        proto.set("channel", js_channel).unwrap();
        return Ok(Some(proto));
    }

    fn constructor(_: &rquickjs::Ctx<'js>) -> rquickjs::Result<Option<rquickjs::function::Constructor<'js>>> {
        Ok(None)
    }
}
impl<'js> IntoJs<'js> for TaskJoin
{
    fn into_js(self, ctx: &Ctx<'js>) -> rquickjs::Result<Value<'js>> {
        Class::instance(ctx.clone(), self).into_js(ctx)
    }
}
