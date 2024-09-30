use common::rustdata::RustData;
use rquickjs::{async_with, function::Args, prelude::{Async, Rest}, AsyncContext, Class, Ctx, Function, Module, Value};
static CANNOT_SERIALIZE: &str = "cannot serialize a value, being passed to another thread.";

#[allow(clippy::needless_pass_by_value)]
pub async fn start<'js>(fun: Function<'js>, params: Rest<Value<'js>>) {
    let ctx1 = fun.ctx().clone();
    let fun_name: String = fun.get("name").unwrap();
    let fun_name = fun_name.trim();
    assert!(!fun_name.is_empty(), "unnamed fn {} can't be called from seperate runtime.", common::value_to_string(fun.into_value()));
    let fun_name = fun_name.to_string();
    let params_json: Vec<_> = params.into_inner().into_iter().map(|value| {
        let Ok(value) = ctx1.json_stringify(value) else {
            panic!("{}",CANNOT_SERIALIZE);
        };
        let Some(value) = value else {
            panic!("{}",CANNOT_SERIALIZE);
        };
        value.to_string().unwrap()
    }).collect();
    let rust_data: Class<RustData> = ctx1.globals().get(common::rustdata::RUST_DATA).unwrap();
    let rust_data = rust_data.borrow().clone();

    std::thread::spawn(||{
        futures_lite::future::block_on(
        in_thread(params_json, fun_name, rust_data)
        );
    });
    // in_thread(params_json, fun_name, rust_data).await;
}

async fn in_thread(params_json: Vec<String>, fun_name: String, rust_data: RustData) {
    let rt2 = super::create_runtime().await;
    let context2 = AsyncContext::full(&rt2).await.unwrap();
    async_with!(context2 => |ctx2| {
        api::add_api_obj(&ctx2, []);
        let mut args = Args::new(ctx2.clone(), params_json.len());
        for param_json in params_json {
            let arg = ctx2.json_parse(param_json).unwrap();
            args.push_arg(arg).unwrap();
        }
        run_with_func(&fun_name, args, rust_data, &ctx2).await;
    }).await;
    rt2.idle().await;
}

async fn run_with_func<'js>(fun_name: &str, args: Args<'js>, rust_data: RustData, ctx2: &Ctx<'js>) {
    use super::get_ok_check_err;
    let globals = ctx2.globals();
    let module = unsafe { Module::load(ctx2.clone(), &rust_data.module_byte).unwrap() };
    globals.set(common::rustdata::RUST_DATA, rust_data).unwrap();
    let evaluated_module = super::evaluate_module(ctx2, module).await;
    let fun: Function = evaluated_module.get(fun_name).unwrap();
    get_ok_check_err(ctx2,
        fun.call_arg::<Value>(args)
    );
}

pub fn thread_fn<'js>(ctx: &Ctx<'js>) -> Function<'js> {
    Function::new(ctx.clone(), Async(start)).unwrap()
}
//
