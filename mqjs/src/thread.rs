mod pool;
use core::time::Duration;

use common::thread::{JsChannel, JsJoinHandle};
use rquickjs::{async_with, function::Args, prelude::Rest, AsyncContext, AsyncRuntime, Ctx, Function, Module, Object, Value};
static CANNOT_SERIALIZE: &str = "cannot serialize a value, being passed to another thread.";

pub fn add_thread_objects(global: &mut Object) {
    let ctx = global.ctx();
    let api = global.get("api").unwrap_or(
        Object::new(ctx.clone()).unwrap()
    );
    let thread = Object::new(ctx.clone()).unwrap();
    thread.set("start", Function::new(ctx.clone(), start)).unwrap();

    thread.set("start_with_channel", js_start_with_channel).unwrap();

    thread.set("pool", 
        Function::new(ctx.clone(), pool::ThreadPool::new)).unwrap();
    let sleep = |secs: f32| {
        std::thread::sleep(Duration::from_secs_f32(secs));
    };
    thread.set("sleep", Function::new(ctx.clone(), sleep)).unwrap();

    api.set("thread", thread).unwrap();
}

#[allow(clippy::needless_pass_by_value)]
pub fn start<'js>(fun: Function<'js>, params: Rest<Value<'js>>) -> common::thread::JsJoinHandle {
    let (fun_name, params_json) = setup_task(fun, params);

    let join = std::thread::spawn(||{
        let rt = super::create_runtime();
        in_thread(&rt, params_json, fun_name, None)
    });
    return JsJoinHandle::new(Some(join), None);
    // in_thread(params_json, fun_name, rust_data).await;
}
#[rquickjs::function]
#[allow(clippy::needless_pass_by_value)]
pub fn start_with_channel<'js>(fun: Function<'js>, params: Rest<Value<'js>>) -> common::thread::JsJoinHandle {
    let (fun_name, params_json) = setup_task(fun, params);

    let [channel0, channel1] = JsChannel::pair();
    let join = std::thread::spawn(move ||{
        let rt = super::create_runtime();
        in_thread(&rt, params_json, fun_name, Some(channel0))
    });
    return JsJoinHandle::new(Some(join), Some(channel1));
    // in_thread(params_json, fun_name, rust_data).await;
}

fn setup_task<'js>(fun: Function<'js>, params: Rest<Value<'js>>) -> (String, Vec<String>) {
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
    (fun_name, params_json)
}

async fn in_thread_async(rt2: &AsyncRuntime, params_json: Vec<String>, fun_name: String, channel: Option<JsChannel>) -> Option<String>{
    let context2 = AsyncContext::full(rt2).await.unwrap();
    let rv = async_with!(context2 => |ctx2| {
        api::add_api_obj(&ctx2, []);
        let mut args = Args::new(ctx2.clone(), params_json.len());
        if let Some(channel) = channel {
            args.push_arg(channel).unwrap();
        }
        for param_json in params_json {
            let arg = ctx2.json_parse(param_json).unwrap();
            args.push_arg(arg).unwrap();
        }
        run_with_func(&fun_name, args, &ctx2).await
    }).await;
    rt2.idle().await;
    return rv;
}
fn in_thread(rt2: &AsyncRuntime, params_json: Vec<String>, fun_name: String, channel: Option<JsChannel>) -> Option<String>{
    futures_lite::future::block_on(
        in_thread_async(rt2, params_json, fun_name, channel)
    )
}

async fn run_with_func<'js>(fun_name: &str, args: Args<'js>, ctx2: &Ctx<'js>) -> Option<String> {
    use super::get_ok_check_err;
    let module = {
        let rust_data = super::RUST_DATA.read().unwrap();
        let rust_data = rust_data.as_ref().unwrap();
        unsafe { Module::load(ctx2.clone(), &rust_data.module_byte).unwrap() }
    };
    let evaluated_module = super::evaluate_module(ctx2, module).await;
    let fun: Function = evaluated_module.get(fun_name).unwrap();
    let rv = get_ok_check_err(ctx2,
        fun.call_arg::<Value>(args)
    );
    return Some(ctx2
        .json_stringify(rv)
        .unwrap()?
        .to_string()
        .unwrap());
}

// pub fn thread_fn<'js>(ctx: &Ctx<'js>) -> Function<'js> {
//     Function::new(ctx.clone(), Async(start)).unwrap()
// }
//
