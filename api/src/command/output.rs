use rquickjs::class::Trace;

#[rquickjs::class]
pub struct Output {
    pub out: Vec<u8>,
    pub err: Vec<u8>,
    pub exitcode: Option<i32>,
}

impl From<std::process::Output> for Output {
    fn from(value: std::process::Output) -> Self {
        Self { out: value.stdout, err: value.stderr, exitcode: value.status.code() }
    }
}

impl<'js> Trace<'js> for Output {
    fn trace<'a>(&self, _: rquickjs::class::Tracer<'a, 'js>) {}
}
#[allow(non_snake_case)]
#[rquickjs::methods]
impl Output {
    #[qjs(get)]
    pub fn out(&self) -> &Vec<u8> {&self.out}
    #[qjs(get)]
    pub fn err(&self) -> &Vec<u8> {&self.err}
    #[qjs(get)]
    pub fn exitcode(&self) -> Option<i32> {
        self.exitcode
    }
    pub fn stdout(&self) -> String {
        String::from_utf8(self.out.clone()).unwrap()
    }
    pub fn stderr(&self) -> String {
        String::from_utf8(self.err.clone()).unwrap()
    }
}
