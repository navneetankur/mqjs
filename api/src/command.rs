pub mod output;
pub mod child;
use std::{fs::File, process::{Command, Stdio}};
use output::Output;
use rquickjs::{class::Trace, Object};

pub struct JsCommand {
    v: Command
}
impl<'js> Trace<'js> for JsCommand {
    fn trace<'a>(&self, tracer: rquickjs::class::Tracer<'a, 'js>) {}
}

impl JsCommand {
    fn new(program: String) -> Self {
        Self{v:Command::new(program)}
    }
    fn arg(&mut self, arg: String) {
        self.v.arg(arg);
    }
    fn args(&mut self, args: Vec<String>) {
        self.v.args(args);
    }
    fn dir(&mut self, dir: String) {
        self.v.current_dir(dir);
    }
    fn env(&mut self, key: String, val: String) {
        self.v.env(key, val);
    }
    fn envs(&mut self, jsobj: Object) {
        for entry in jsobj.props::<String, String>() {
            let Ok((k,v)) = entry else {continue};
            self.env(k, v);
        }
    }
    fn output(&mut self) -> output::Output {
        let out = self.v.output().unwrap();
        Output { out: out.stdout, err: out.stderr, exitcode: out.status.code() }
    }
    fn status(&mut self) -> Option<i32> {
        self.v.status().unwrap().code()
    }
    fn stdin_null(&mut self) {
        self.v.stdin(Stdio::null());
    }
    fn stdin_filepath(&mut self, path: String) {
        let file = File::open(path).unwrap();
        self.v.stdin(file);
    }
    fn stdin_piped(&mut self) {
        self.v.stdin(Stdio::piped());
    }
    fn stdout_null(&mut self) {
        self.v.stdout(Stdio::null());
    }
    fn stdout_filepath(&mut self, path: String) {
        let file = File::open(path).unwrap();
        self.v.stdout(file);
    }
    fn stdout_piped(&mut self) {
        self.v.stdout(Stdio::piped());
    }
    fn stderr_null(&mut self) {
        self.v.stderr(Stdio::null());
    }
    fn stderr_filepath(&mut self, path: String) {
        let file = File::open(path).unwrap();
        self.v.stderr(file);
    }
    fn stderr_piped(&mut self) {
        self.v.stderr(Stdio::piped());
    }
}
