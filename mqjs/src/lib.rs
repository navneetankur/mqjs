mod thread;
static RUST_DATA :RwLock<Option<RustData>> = RwLock::new(None);
thread_local! {
    static RUNTIME: AsyncRuntime = {
        create_runtime()
    }
}

use std::{env::Args, fs::File, io::Read, path::{Path, PathBuf}, sync::RwLock};
use common::rustdata::RustData;
use rquickjs::{async_with, loader::{NativeLoader, ScriptLoader}, AsyncContext, AsyncRuntime, Ctx, Function, Module, Value};
const MODULE_PATH_JS: &str = "/home/navn/bin/lib/mqjs/modules/js";
const MODULE_PATH_SO: &str = "/home/navn/bin/lib/mqjs/modules/so";
#[cfg(debug_assertions)]
const WORKSPACE_TEMP: &str = "/home/navn/workspace/rust/mqjs/target/debug";

pub fn realmain(args: Args) {
    let mut args = args.peekable();
    args.next(); //get rid of this program name.
    let script_name = args.peek().expect("No script file provided.").clone();
    let source = get_source(&script_name);
    process_and_run(&source, &script_name, args);
}

fn get_source(file_name: &str) -> Vec<u8> {
    // let use_strict = b"\"use strict;\"\n";
    let use_strict = b"";
    let mut file = File::open(file_name).unwrap();
    let size = file.metadata().map(|m| m.len().try_into().unwrap() ).ok();
    let mut bytes = Vec::new();
    bytes.reserve_exact(use_strict.len() + size.unwrap_or(0));
    bytes.extend_from_slice(use_strict);
    file.read_to_end(&mut bytes).unwrap();
    return bytes;
}

fn create_runtime() -> rquickjs::AsyncRuntime {
    let rt = AsyncRuntime::new().unwrap();
    let resolver = 
        SimpleResolver::default().with_paths(
            [
            #[cfg(debug_assertions)]
            WORKSPACE_TEMP,
            MODULE_PATH_JS, MODULE_PATH_SO, 
            ]
        );
    let loader = (
        // BuiltinLoader::default(),
        // ModuleLoader::default(),
        NativeLoader::default(),
        ScriptLoader::default(),
    );
    futures_lite::future::block_on(
        rt.set_loader(resolver, loader)
    );
    return rt;
}

fn process_and_run(source: &[u8], file_name: &str, args: impl IntoIterator<Item = String>) {
    let rt = create_runtime();
    futures_lite::future::block_on(
        process_and_run_async(&rt, source, file_name, args)
    );
    // rt.idle().await;
}
async fn process_and_run_async(rt: &AsyncRuntime, source: &[u8], file_name: &str, args: impl IntoIterator<Item = String>) {
    let context = AsyncContext::full(rt).await.unwrap();
    async_with!(context => |ctx| {
        api::add_api_obj(&ctx, args);
        signal::add_signal_props(&ctx.globals());
        run_js_source(&ctx, source, file_name).await;
    }).await;
    rt.idle().await;
}
async fn run_js_source<'js>(ctx: &Ctx<'js>, source: &[u8], file_name: &str) {
    let module_decl = Module::declare(ctx.clone(), file_name, source);
    let module_decl = get_ok_check_err(ctx, module_decl);
    let module_bytes = get_ok_check_err(ctx, 
        module_decl.write(false)
    );
    let mut globals = ctx.globals();
    thread::add_thread_objects(&mut globals);
    {
        let mut rust_data = RUST_DATA.write().unwrap();
        *rust_data = Some(RustData::new(module_bytes));
    }
    let module_evaluated = evaluate_module(ctx, module_decl).await;
    let Ok(main) = module_evaluated.get::<_,Function>("main") else { return ; };
    let main_return = get_ok_check_err(ctx, 
        main.call::<_, Value>(())
    );
    loop_until_promise_fulfilled(ctx, main_return).await;
}
async fn loop_until_promise_fulfilled<'js>(ctx: &Ctx<'js>, mut value: Value<'js>) {
    while let Some(promise) = value.into_promise() {
        value = get_ok_check_err(ctx,
            promise.into_future().await
        );
    }
}
async fn evaluate_module<'js>(ctx: &Ctx<'js>, module: Module<'js>) -> Module<'js, rquickjs::module::Evaluated> {
    let (module_evaluated, module_evaluation) = get_ok_check_err(ctx, 
        module.eval()
    );
    let evaluation_result = module_evaluation.into_future::<Value>().await;
    let value = get_ok_check_err(ctx, evaluation_result);
    loop_until_promise_fulfilled(ctx, value).await;
    return module_evaluated;
}
fn get_ok_check_err<V>(ctx: &Ctx, result: rquickjs::Result<V>) -> V {
    match result {
        Ok(v) => v,
        Err(e) => {check_err(e, ctx); unreachable!()},
    }

}

fn check_err(e: rquickjs::Error, ctx: &Ctx) {
    match e {
        rquickjs::Error::Exception => {
            let catch = ctx.catch();
            let Some(ex) = catch.as_exception() else {return};
            panic!("{ex:?}");
        },
        other => {
            panic!("{other:?}");
        }
    }
}

#[derive(Default)]
pub struct SimpleResolver {
    paths: Vec<PathBuf>,
}
impl SimpleResolver {
    pub fn add(&mut self, path: impl AsRef<Path>) {
        self.paths.push(path.as_ref().into());
    }
    pub fn with(mut self, path: impl AsRef<Path>) -> Self {
        self.add(path);
        return self;
    }
    pub fn with_paths(mut self, paths: impl IntoIterator<Item = impl AsRef<Path>>) -> Self {
        self.paths.extend(paths.into_iter().map(|v| v.as_ref().into()));
        return self;
    }
}
impl rquickjs::loader::Resolver for SimpleResolver {
    fn resolve<'js>(&mut self, _: &Ctx<'js>, base: &str, name: &str) -> rquickjs::Result<String> {
        let find1 = PathBuf::from(name);
        if find1.is_file() { return Ok(find1.to_str().unwrap().to_string()) }
        let mut find2 = find1.clone();
        find2.set_extension("js");
        let mut find3 = find1.clone();
        find3.set_extension("so");
        let mut find4 = find1.clone();
        let stem = find4.file_stem().unwrap().to_str().unwrap();
        let stem = String::from("lib") + stem;
        find4.set_file_name(stem);
        find4.set_extension("so");
        let finds = [find1, find2, find3, find4];

        for path in &self.paths {
            for to_find in &finds {
                let to_look = path.join(to_find);
                if to_look.is_file() {
                    return Ok(to_look.into_os_string().into_string().unwrap());
                }
            }
        }
        return Err(rquickjs::Error::new_resolving(base, name));
    }
}

