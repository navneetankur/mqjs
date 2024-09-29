mod command;
mod file;
mod out;
use command::JsCommand;
use common::bufread::JsBufReader;
use rquickjs::{Array, Class, Ctx, Function, Object};

const PRINTLN: &str = "println";
const PRINT: &str = "print";
const ARGS: &str = "args";
const STDIN: &str = "stdin";
const OPENR: &str = "openr";
const OPENW: &str = "openw";
const OUT: &str = "out";

pub fn add_api_obj(ctx: &Ctx, args: impl IntoIterator<Item = String>) {
    let globals = ctx.globals();
    let api = Object::new(ctx.clone()).unwrap();
    let jargs = get_args_array(ctx.clone(), args);
    api.set(ARGS, jargs).unwrap();
    let println = Function::new(ctx.clone(), common::js_println).unwrap().with_name(PRINTLN).unwrap();
    let print = Function::new(ctx.clone(), common::js_print).unwrap().with_name(PRINT).unwrap();
    api.set(OPENR, file::fileread::js_openr).unwrap();
    api.set(OPENW, file::filewrite::js_openw).unwrap();
    api.set(OUT, out::Out::default()).unwrap();
    let v = std::io::stdin().lock();
    api.set(STDIN, JsBufReader::new(v)).unwrap();
    globals.set(PRINTLN, println).unwrap();
    globals.set(PRINT, print).unwrap();
    globals.set("api", api).unwrap();
    Class::<JsCommand>::define(&globals).unwrap();
}
fn get_args_array(ctx: Ctx<'_>,  args: impl IntoIterator<Item = String>) -> Array<'_> {
    let jargs = Array::new(ctx).unwrap();
    for (i, arg) in args.into_iter().enumerate() {
        jargs.set(i, arg).unwrap();
    }
    jargs
}
