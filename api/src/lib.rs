use std::env::Args;

use rquickjs::{Array, Ctx, Object};

pub fn add_api_obj(ctx: &mut Ctx, args: Args) {
    let globals = ctx.globals();
    let api = Object::new(ctx.clone()).unwrap();
    let jargs = get_args_array(ctx.clone(), args);
    api.set("args", jargs).unwrap();
    globals.set("api", api).unwrap();
}

fn get_args_array<'a>(ctx: Ctx<'a>, args: Args) -> Array<'a> {
    let jargs = Array::new(ctx).unwrap();
    for (i, arg) in args.enumerate() {
        jargs.set(i, arg).unwrap();
    }
    jargs
}
