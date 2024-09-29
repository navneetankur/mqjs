pub mod output;
pub mod child;
use std::{fs::File, process::{Command, Stdio}};
use child::JsChild;
use output::Output;
use rquickjs::{class::{ClassId, JsClass, OwnedBorrowMut, Trace, Writable}, prelude::This, Class, Function, IntoJs, Object};
const ARG: &str = "arg";
const ARGS: &str = "args";
const ENV: &str = "env";
const ENVS: &str = "envs";
const CURRENT_DIR: &str = "current_dir";
const OUTPUT: &str = "output";
const SPAWN: &str = "spawn";
const STATUS: &str = "status";
const STDIN_NULL: &str = "stdin_null";
const STDIN_FILEPATH: &str = "stdin_filepath";
const STDIN_PIPED: &str = "stdin_piped";
const STDIN_INHERIT: &str = "stdin_inherit";
const STDERR_NULL: &str = "stderr_null";
const STDERR_FILEPATH: &str = "stderr_filepath";
const STDERR_PIPED: &str = "stderr_piped";
const STDERR_INHERIT: &str = "stderr_inherit";
const STDOUT_NULL: &str = "stdout_null";
const STDOUT_FILEPATH: &str = "stdout_filepath";
const STDOUT_PIPED: &str = "stdout_piped";
const STDOUT_INHERIT: &str = "stdout_inherit";
#[cfg(unix)]
const EXEC: &str = "exec";

pub struct JsCommand {
    v: Command
}
impl<'js> Trace<'js> for JsCommand {
    fn trace<'a>(&self, _: rquickjs::class::Tracer<'a, 'js>) {}
}
impl<'js> JsClass<'js> for JsCommand {
    const NAME: &'static str = "Command";

    type Mutable = Writable;

    fn class_id() -> &'static rquickjs::class::ClassId {
        static ID: ClassId = ClassId::new();
        &ID
    }

    fn prototype(ctx: &rquickjs::Ctx<'js>) -> rquickjs::Result<Option<Object<'js>>> {
        let proto = Object::new(ctx.clone())?;
        let args = Function::new(ctx.clone(), args).unwrap();
        proto.set(ARGS, args).unwrap();

        let arg = Function::new(ctx.clone(), arg).unwrap();
        proto.set(ARG, arg).unwrap();

        let spawn = Function::new(ctx.clone(), spawn).unwrap();
        proto.set(SPAWN, spawn).unwrap();

        let current_dir = Function::new(ctx.clone(), current_dir).unwrap();
        proto.set(CURRENT_DIR, current_dir).unwrap();

        let env = Function::new(ctx.clone(), env).unwrap();
        proto.set(ENV, env).unwrap();

        let envs = Function::new(ctx.clone(), envs).unwrap();
        proto.set(ENVS, envs).unwrap();

        let output = Function::new(ctx.clone(), output).unwrap();
        proto.set(OUTPUT, output).unwrap();

        let status = Function::new(ctx.clone(), status).unwrap();
        proto.set(STATUS, status).unwrap();

        let stdin_null = Function::new(ctx.clone(), stdin_null).unwrap();
        proto.set(STDIN_NULL, stdin_null).unwrap();

        let stdin_filepath = Function::new(ctx.clone(), stdin_filepath).unwrap();
        proto.set(STDIN_FILEPATH, stdin_filepath).unwrap();

        let stdin_piped = Function::new(ctx.clone(), stdin_piped).unwrap();
        proto.set(STDIN_PIPED, stdin_piped).unwrap();

        let stdin_inherit = Function::new(ctx.clone(), stdin_inherit).unwrap();
        proto.set(STDIN_INHERIT, stdin_inherit).unwrap();

        let stdout_null = Function::new(ctx.clone(), stdout_null).unwrap();
        proto.set(STDOUT_NULL, stdout_null).unwrap();

        let stdout_filepath = Function::new(ctx.clone(), stdout_filepath).unwrap();
        proto.set(STDOUT_FILEPATH, stdout_filepath).unwrap();

        let stdout_piped = Function::new(ctx.clone(), stdout_piped).unwrap();
        proto.set(STDOUT_PIPED, stdout_piped).unwrap();

        let stdout_inherit = Function::new(ctx.clone(), stdout_inherit).unwrap();
        proto.set(STDOUT_INHERIT, stdout_inherit).unwrap();

        let stderr_null = Function::new(ctx.clone(), stderr_null).unwrap();
        proto.set(STDERR_NULL, stderr_null).unwrap();

        let stderr_filepath = Function::new(ctx.clone(), stderr_filepath).unwrap();
        proto.set(STDERR_FILEPATH, stderr_filepath).unwrap();

        let stderr_piped = Function::new(ctx.clone(), stderr_piped).unwrap();
        proto.set(STDERR_PIPED, stderr_piped).unwrap();

        let stderr_inherit = Function::new(ctx.clone(), stderr_inherit).unwrap();
        proto.set(STDERR_INHERIT, stderr_inherit).unwrap();

        #[cfg(unix)]
        {
            let exec = Function::new(ctx.clone(), exec).unwrap();
            proto.set(EXEC, exec).unwrap();
        }

        return Ok(Some(proto));

    }

    fn constructor(ctx: &rquickjs::Ctx<'js>) -> rquickjs::Result<Option<rquickjs::function::Constructor<'js>>> {
        let newf = Function::new(ctx.clone(), JsCommand::new).unwrap().with_constructor(true).into_value().into_constructor().unwrap();
        Ok(Some(newf))
    }
}
impl<'js> IntoJs<'js> for JsCommand {
    fn into_js(self, ctx: &rquickjs::Ctx<'js>) -> rquickjs::Result<rquickjs::Value<'js>> {
        Class::instance(ctx.clone(), self).into_js(ctx)
    }
}
impl JsCommand {
    fn new(program: String) -> Self {
        Self{v:Command::new(program)}
    }
    fn env(&mut self, key: String, val: String) {
        self.v.env(key, val);
    }
}
fn arg(mut this: This<OwnedBorrowMut<JsCommand>>, arg: String) {
    this.v.arg(arg);
}
fn args(mut this: This<OwnedBorrowMut<JsCommand>>, args: Vec<String>) {
    this.v.args(args);
}
fn current_dir(mut this: This<OwnedBorrowMut<JsCommand>>, dir: String) {
    this.v.current_dir(dir);
}
fn env(mut this: This<OwnedBorrowMut<JsCommand>>, key: String, val: String) {
    this.v.env(key, val);
}
#[allow(clippy::needless_pass_by_value)]
fn envs(mut this: This<OwnedBorrowMut<JsCommand>>, jsobj: Object) {
    for entry in jsobj.props::<String, String>() {
        let Ok((k,v)) = entry else {continue};
        this.env(k, v);
    }
}
fn output(mut this: This<OwnedBorrowMut<JsCommand>>) -> output::Output {
    let out = this.v.output().unwrap();
    Output { out: out.stdout, err: out.stderr, exitcode: out.status.code() }
}
fn status(mut this: This<OwnedBorrowMut<JsCommand>>) -> Option<i32> {
    this.v.status().unwrap().code()
}
fn stdin_null(mut this: This<OwnedBorrowMut<JsCommand>>) {
    this.v.stdin(Stdio::null());
}
fn stdin_filepath(mut this: This<OwnedBorrowMut<JsCommand>>, path: String) {
    let file = File::open(path).unwrap();
    this.v.stdin(file);
}
fn stdin_piped(mut this: This<OwnedBorrowMut<JsCommand>>) {
    this.v.stdin(Stdio::piped());
}
fn stdin_inherit(mut this: This<OwnedBorrowMut<JsCommand>>) {
    this.v.stdin(Stdio::inherit());
}
fn stdout_null(mut this: This<OwnedBorrowMut<JsCommand>>) {
    this.v.stdout(Stdio::null());
}
fn stdout_filepath(mut this: This<OwnedBorrowMut<JsCommand>>, path: String) {
    let file = File::open(path).unwrap();
    this.v.stdout(file);
}
fn stdout_piped(mut this: This<OwnedBorrowMut<JsCommand>>) {
    this.v.stdout(Stdio::piped());
}
fn stdout_inherit(mut this: This<OwnedBorrowMut<JsCommand>>) {
    this.v.stdout(Stdio::inherit());
}
fn stderr_null(mut this: This<OwnedBorrowMut<JsCommand>>) {
    this.v.stderr(Stdio::null());
}
fn stderr_filepath(mut this: This<OwnedBorrowMut<JsCommand>>, path: String) {
    let file = File::open(path).unwrap();
    this.v.stderr(file);
}
fn stderr_piped(mut this: This<OwnedBorrowMut<JsCommand>>) {
    this.v.stderr(Stdio::piped());
}
fn stderr_inherit(mut this: This<OwnedBorrowMut<JsCommand>>) {
    this.v.stderr(Stdio::inherit());
}
fn spawn(mut this: This<OwnedBorrowMut<JsCommand>>) -> std::io::Result<JsChild> {
    this.v.spawn().map(std::convert::Into::into)
}
#[cfg(unix)]
fn exec(mut this: This<OwnedBorrowMut<JsCommand>>) {
    use std::os::unix::process::CommandExt;
    this.v.exec();
}
