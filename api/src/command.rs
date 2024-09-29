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
macro_rules! impl_fn {
    ($proto:ident, $($funcs:ident),+) => {
        $(
        let temp = Function::new($proto.ctx().clone(), $funcs).unwrap();
        $proto.set(stringify!($funcs), temp).unwrap();
        )+
    };
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
        impl_fn!(proto, 
            args,
            arg,
            spawn,
            current_dir,
            env,
            envs,
            output,
            status,
            stdin_null,
            stdin_filepath,
            stdin_piped,
            stdin_inherit,
            stdout_null,
            stdout_filepath,
            stdout_piped,
            stdout_inherit,
            stderr_null,
            stderr_filepath,
            stderr_piped,
            stderr_inherit
        );
        #[cfg(unix)]
        impl_fn!(proto, exec);

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
fn arg(mut this: This<OwnedBorrowMut<JsCommand>>, arg: String) -> Class<JsCommand>{
    this.v.arg(arg);
    return this.0.into_inner();
}
fn args(mut this: This<OwnedBorrowMut<JsCommand>>, args: Vec<String>) -> Class<JsCommand>{
    this.v.args(args);
	return this.0.into_inner();
}
fn current_dir(mut this: This<OwnedBorrowMut<JsCommand>>, dir: String) -> Class<JsCommand>{
    this.v.current_dir(dir);
	return this.0.into_inner();
}
fn env(mut this: This<OwnedBorrowMut<JsCommand>>, key: String, val: String) -> Class<JsCommand>{
    this.v.env(key, val);
	return this.0.into_inner();
}
#[allow(clippy::needless_pass_by_value)]
fn envs<'js>(mut this: This<OwnedBorrowMut<'js, JsCommand>>, jsobj: Object<'js>) -> Class<'js, JsCommand>{
    for entry in jsobj.props::<String, String>() {
        let Ok((k,v)) = entry else {continue};
        this.env(k, v);
    }
	return this.0.into_inner();
}
fn stdin_null(mut this: This<OwnedBorrowMut<JsCommand>>) -> Class<JsCommand>{
    this.v.stdin(Stdio::null());
	return this.0.into_inner();
}
fn stdin_filepath(mut this: This<OwnedBorrowMut<JsCommand>>, path: String) -> Class<JsCommand>{
    let file = File::open(path).unwrap();
    this.v.stdin(file);
	return this.0.into_inner();
}
fn stdin_piped(mut this: This<OwnedBorrowMut<JsCommand>>) -> Class<JsCommand>{
    this.v.stdin(Stdio::piped());
	return this.0.into_inner();
}
fn stdin_inherit(mut this: This<OwnedBorrowMut<JsCommand>>) -> Class<JsCommand>{
    this.v.stdin(Stdio::inherit());
	return this.0.into_inner();
}
fn stdout_null(mut this: This<OwnedBorrowMut<JsCommand>>) -> Class<JsCommand>{
    this.v.stdout(Stdio::null());
	return this.0.into_inner();
}
fn stdout_filepath(mut this: This<OwnedBorrowMut<JsCommand>>, path: String) -> Class<JsCommand>{
    let file = File::open(path).unwrap();
    this.v.stdout(file);
	return this.0.into_inner();
}
fn stdout_piped(mut this: This<OwnedBorrowMut<JsCommand>>) -> Class<JsCommand>{
    this.v.stdout(Stdio::piped());
	return this.0.into_inner();
}
fn stdout_inherit(mut this: This<OwnedBorrowMut<JsCommand>>) -> Class<JsCommand>{
    this.v.stdout(Stdio::inherit());
	return this.0.into_inner();
}
fn stderr_null(mut this: This<OwnedBorrowMut<JsCommand>>) -> Class<JsCommand>{
    this.v.stderr(Stdio::null());
	return this.0.into_inner();
}
fn stderr_filepath(mut this: This<OwnedBorrowMut<JsCommand>>, path: String) -> Class<JsCommand>{
    let file = File::open(path).unwrap();
    this.v.stderr(file);
	return this.0.into_inner();
}
fn stderr_piped(mut this: This<OwnedBorrowMut<JsCommand>>) -> Class<JsCommand>{
    this.v.stderr(Stdio::piped());
	return this.0.into_inner();
}
fn stderr_inherit(mut this: This<OwnedBorrowMut<JsCommand>>) -> Class<JsCommand>{
    this.v.stderr(Stdio::inherit());
	return this.0.into_inner();
}
fn spawn(mut this: This<OwnedBorrowMut<JsCommand>>) -> std::io::Result<JsChild> {
    this.v.spawn().map(std::convert::Into::into)
}
#[cfg(unix)]
fn exec(mut this: This<OwnedBorrowMut<JsCommand>>) {
    use std::os::unix::process::CommandExt;
    this.v.exec();
}
fn output(mut this: This<OwnedBorrowMut<JsCommand>>) -> output::Output {
    let out = this.v.output().unwrap();
    Output { out: out.stdout, err: out.stderr, exitcode: out.status.code() }
}
fn status(mut this: This<OwnedBorrowMut<JsCommand>>) -> Option<i32> {
    this.v.status().unwrap().code()
}
