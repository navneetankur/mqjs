use std::process::{Child, ChildStderr, ChildStdin, ChildStdout};

use rquickjs::class::Trace;

use super::output::Output;

#[rquickjs::class]
pub struct JsChild {
    v: Child,
}

#[rquickjs::methods]
impl JsChild {
    pub fn kill(&mut self) -> i32 {
        if let Ok(()) = self.v.kill() {
            return 0;
        } else {
            return -1;
        }
    }
    pub fn id(&self) -> u32 {self.v.id()}
    pub fn wait(&mut self) -> Option<i32> {
        self.v.wait().unwrap().code()
    }
    pub fn try_wait(&mut self) -> Option<i32> {
        self.v.try_wait().unwrap()?.code()
    }
    pub fn wait_with_output(self) -> Output {
        Output::from(self.v.wait_with_output().unwrap())
    }
}
























impl<'js> Trace<'js> for JsChild {
    fn trace<'a>(&self, tracer: rquickjs::class::Tracer<'a, 'js>) {}
}
#[rquickjs::class]
pub struct JsChildStdin{
    v: ChildStdin,
}
impl<'js> Trace<'js> for JsChildStdin {
    fn trace<'a>(&self, tracer: rquickjs::class::Tracer<'a, 'js>) {}
}
#[rquickjs::class]
pub struct JsChildStdout{
    v: ChildStdout,
}
impl<'js> Trace<'js> for JsChildStdout {
    fn trace<'a>(&self, tracer: rquickjs::class::Tracer<'a, 'js>) {}
}
#[rquickjs::class]
pub struct JsChildStderr{
    v: ChildStderr,
}
impl<'js> Trace<'js> for JsChildStderr {
    fn trace<'a>(&self, tracer: rquickjs::class::Tracer<'a, 'js>) {}
}
