use std::env::Args;

use rquickjs::{loader::{BuiltinLoader, FileResolver, ModuleLoader, NativeLoader, ScriptLoader}, AsyncContext, AsyncRuntime, CatchResultExt, Ctx, Function, Value};
const MODULE_PATH_JS: &str = "/home/navn/bin/mqjs/modules/js";
const MODULE_PATH_SO: &str = "/home/navn/bin/mqjs/modules/so";
const WORKSPACE_TEMP: &str = "/home/navn/workspace/rust/mqjs/target/debug";


pub async fn realmain(mut args: Args) -> u8 {
    args.next(); //get rid of this program name.

    let resolver = FileResolver::default()
        .with_paths(
            ["./", MODULE_PATH_JS, MODULE_PATH_SO,
            #[cfg(debug_assertions)]
            WORKSPACE_TEMP,
            ]
        ).with_native();
    let loader = (
        BuiltinLoader::default(), ModuleLoader::default(),
        NativeLoader::default(), ScriptLoader::default(),
    );
    let rt = AsyncRuntime::new().unwrap();
    rt.set_loader(resolver, loader).await;
    let context = AsyncContext::full(&rt).await.unwrap();
    let script_name = args.next().expect("No script file provided.");
    let rv = context.with(|mut ctx| {
        api::add_api_obj(&mut ctx, args);
        add_global_fn(&mut ctx);
        let rv = run_js_file(&mut ctx, script_name);
        if rv.is_int() {
            return rv.as_int().unwrap() as u8;
        }
        else {
            return 0;
        }
    }).await;
    return rv;
}

fn run_js_file<'js>(ctx: &mut Ctx<'js>, file: String) -> Value<'js> {
    match ctx.eval_file::<Value,_>(file).catch(ctx) {
        Ok(v) => return v,
        Err(e) => {
            panic!("{e:?}");
        }
    }
}

fn println(v: Value) {
    if let Some(v) = v.as_string() {
        println!("{}", v.to_string().unwrap());
    } 
    else if let Some(v) = v.as_number() {
        println!("{v}");
    }
    else if let Some(v) = v.as_bool() {
        println!("{v}");
    }
    else {
        println!("{v:?}");
    }
}

fn add_global_fn(ctx: &mut Ctx) {
    let globals = ctx.globals();
    globals.set("println", Function::new(ctx.clone(), println)).unwrap();
}
