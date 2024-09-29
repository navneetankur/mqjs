use std::io::{StdoutLock, Write};

use rquickjs::class::Trace;

#[rquickjs::class]
pub struct Out {
    v: StdoutLock<'static>,
}
impl<'js> Trace<'js> for Out{
    fn trace<'a>(&self, _: rquickjs::class::Tracer<'a, 'js>) {}
}
#[rquickjs::methods]
impl Out {
    #[allow(clippy::needless_pass_by_value)]
    pub fn write(&mut self, data: String) {
        self.v.write_all(data.as_bytes()).unwrap();
    }
    #[allow(clippy::needless_pass_by_value)]
    pub fn log(&mut self, data: String) {
        self.println(data);
    }
    #[allow(clippy::needless_pass_by_value)]
    pub fn print(&mut self, data: String) {
        self.write(data);
    }
    #[allow(clippy::needless_pass_by_value)]
    pub fn println(&mut self,data: String) {
        self.write(data);
        self.v.write_all(b"\n").unwrap();
    }
}
impl Default for Out {
    fn default() -> Self {
        Self { v: std::io::stdout().lock() }
    }
}
