use rquickjs::{Array, Ctx, Function, Object};

const PRINTLN: &str = "println";
const PRINT: &str = "print";
const ARGS: &str = "args";

pub fn add_api_obj(ctx: &Ctx, args: impl IntoIterator<Item = String>) {
    let globals = ctx.globals();
    let api = Object::new(ctx.clone()).unwrap();
    let jargs = get_args_array(ctx.clone(), args);
    api.set(ARGS, jargs).unwrap();
    let println = Function::new(ctx.clone(), common::js_println).unwrap().with_name(PRINTLN).unwrap();
    let print = Function::new(ctx.clone(), common::js_print).unwrap().with_name(PRINT).unwrap();
    api.set(PRINT, print.clone()).unwrap();
    api.set(PRINTLN, println.clone()).unwrap();
    globals.set(PRINTLN, println).unwrap();
    globals.set(PRINT, print).unwrap();
    globals.set("api", api).unwrap();
}
fn get_args_array(ctx: Ctx<'_>,  args: impl IntoIterator<Item = String>) -> Array<'_> {
    let jargs = Array::new(ctx).unwrap();
    for (i, arg) in args.into_iter().enumerate() {
        jargs.set(i, arg).unwrap();
    }
    jargs
}
