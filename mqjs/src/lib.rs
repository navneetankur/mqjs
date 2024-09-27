use std::{env::Args, path::{Path, PathBuf}};

use rquickjs::{async_with, loader::{BuiltinLoader, FileResolver, ModuleLoader, NativeLoader, Resolver, ScriptLoader}, AsyncContext, AsyncRuntime, CatchResultExt, Ctx, Function, Module, Value};
const MODULE_PATH_JS: &str = "/home/navn/bin/mqjs/modules/js";
const MODULE_PATH_SO: &str = "/home/navn/bin/mqjs/modules/so";
const WORKSPACE_TEMP: &str = "/home/navn/workspace/rust/mqjs/target/debug";


pub async fn realmain(args: Args) {
    let mut args = args.peekable();
    args.next(); //get rid of this program name.

    let resolver = 
        SimpleResolver::default().with_paths(
            [MODULE_PATH_JS, MODULE_PATH_SO, WORKSPACE_TEMP]
        );
    let loader = (
        BuiltinLoader::default(),
        ModuleLoader::default(),
        NativeLoader::default(),
        ScriptLoader::default(),
    );
    let rt = AsyncRuntime::new().unwrap();
    rt.set_loader(resolver, loader).await;
    let context = AsyncContext::full(&rt).await.unwrap();
    let script_name = args.peek().expect("No script file provided.").clone();
    // context.with(|mut ctx| {
    //     fun_name(ctx, args, script_name);
    // }).await;
    // context.async_with(|mut ctx| {
    //     core::pin::Pin::new(Box::new(fun_name(ctx, args, script_name)))
    // }).await;
    async_with!(context => |ctx| {
        process_and_run(ctx, args, script_name).await;
    }).await;
    rt.idle().await;
}

async fn process_and_run(mut ctx: Ctx<'_>, args: std::iter::Peekable<Args>, script_name: String) {
    api::add_api_obj(&mut ctx, args);
    run_js_file(&mut ctx, script_name).await;
}

async fn run_js_file<'js>(ctx: &mut Ctx<'js>, file: String) {
    let source = std::fs::read(&file).unwrap();
    // Module::evaluate(ctx.clone(),file, source).catch(ctx).unwrap();
    let mod_evaluation = Module::evaluate(ctx.clone(),file, source).unwrap().into_future::<Value>().await;
    if let Err(e) = mod_evaluation {
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
impl Resolver for SimpleResolver {
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

