use rquickjs::{Array, Ctx, Object};

pub fn add_api_obj(ctx: &mut Ctx, args: impl IntoIterator<Item = String>) {
    let globals = ctx.globals();
    let api = Object::new(ctx.clone()).unwrap();
    let jargs = get_args_array(ctx.clone(), args);
    api.set("args", jargs).unwrap();
    globals.set("api", api).unwrap();
}

fn get_args_array<'a>(ctx: Ctx<'a>,  args: impl IntoIterator<Item = String>) -> Array<'a> {
    let jargs = Array::new(ctx).unwrap();
    for (i, arg) in args.into_iter().enumerate() {
        jargs.set(i, arg).unwrap();
    }
    jargs
}
