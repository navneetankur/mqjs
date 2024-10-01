
use futures_channel::oneshot;
use rquickjs::{atom::PredefinedAtom, class::{ClassId, JsClass, OwnedBorrowMut, Trace, Writable}, Class, Ctx, Function, IntoJs, Object, Value};

use super::JsChannel;

pub struct TaskJoin {
    receiver: Option<oneshot::Receiver<Option<String>>>,
    channel: Option<JsChannel>,
}
static RECIEVER_GONE: &str = "Reciever is missing, did you await twice?";
type This<'js> = rquickjs::function::This<OwnedBorrowMut<'js, TaskJoin>>;
#[allow(clippy::needless_pass_by_value)]
fn then<'js>(ctx: Ctx<'js>, mut this: This<'js>, resolve: Function<'js>, reject: Function<'js>) {
    let receiver = this.receiver.take().expect(RECIEVER_GONE);
    let future = async move {
        match receiver.await {
            Ok(result) => {
                resolve.call::<_,Value>((result,)).unwrap();
            }
            Err(_) => {
                reject.call::<_,Value>(("cancelled",)).unwrap();
            }
        }
    };
    ctx.spawn(future);
}
fn channel(mut this: This<'_>) -> Option<super::channel::JsChannel> {
    return this.channel.take()
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

        proto.set("channel", 
            Function::new(ctx.clone(), channel).unwrap()
        ).unwrap();
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
