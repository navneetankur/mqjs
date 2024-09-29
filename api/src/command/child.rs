use std::process::{Child, ChildStderr, ChildStdin, ChildStdout};

use rquickjs::class::Trace;

#[rquickjs::class]
pub struct JsChild {
    v: Child,
}

#[rquickjs::methods]
impl JsChild {
    pub fn stdout(&mut self) -> std::option::Option<JsChildStdout> {
        self.v.stdout.take().map(std::convert::Into::into)
    }
    pub fn stdin(&mut self) -> std::option::Option<JsChildStdin> {
        self.v.stdin.take().map(std::convert::Into::into)
    }
    pub fn stderr(&mut self) -> std::option::Option<JsChildStderr> {
        self.v.stderr.take().map(std::convert::Into::into)
    }
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
}

impl From<Child> for JsChild {
    fn from(value: Child) -> Self {
        Self { v: value }
    }
}
impl<'js> Trace<'js> for JsChild {
    fn trace<'a>(&self, _: rquickjs::class::Tracer<'a, 'js>) {}
}
#[rquickjs::class]
pub struct JsChildStdin{
    v: ChildStdin,
}
impl From<ChildStdin> for JsChildStdin {
    fn from(value: ChildStdin) -> Self {
        Self { v: value }
    }
}
impl<'js> Trace<'js> for JsChildStdin {
    fn trace<'a>(&self, _: rquickjs::class::Tracer<'a, 'js>) {}
}
#[rquickjs::class]
pub struct JsChildStdout{
    v: ChildStdout,
}
impl From<ChildStdout> for JsChildStdout {
    fn from(value: ChildStdout) -> Self {
        Self { v: value }
    }
}
impl<'js> Trace<'js> for JsChildStdout {
    fn trace<'a>(&self, _: rquickjs::class::Tracer<'a, 'js>) {}
}
#[rquickjs::class]
pub struct JsChildStderr{
    v: ChildStderr,
}
impl From<ChildStderr> for JsChildStderr {
    fn from(value: ChildStderr) -> Self {
        Self { v: value }
    }
}
impl<'js> Trace<'js> for JsChildStderr {
    fn trace<'a>(&self, _: rquickjs::class::Tracer<'a, 'js>) {}
}
