use std::{env::Args, path::{Path, PathBuf}};

use rquickjs::{async_with, loader::{BuiltinLoader, ModuleLoader, NativeLoader, Resolver, ScriptLoader}, AsyncContext, AsyncRuntime, Ctx, Module, Value};
const MODULE_PATH_JS: &str = "/home/navn/bin/lib/mqjs/modules/js";
const MODULE_PATH_SO: &str = "/home/navn/bin/lib/mqjs/modules/so";
const WORKSPACE_TEMP: &str = "/home/navn/workspace/rust/mqjs/target/debug";


pub async fn realmain(args: Args) {
    let mut args = args.peekable();
    args.next(); //get rid of this program name.
    let script_name = args.peek().expect("No script file provided.").clone();
    let rt = AsyncRuntime::new().unwrap();
    let source = std::fs::read(&script_name).unwrap();
    process_and_run(rt, &source, &script_name, args).await;
}

async fn process_and_run(rt: AsyncRuntime, source: &[u8], file_name: &str, args: impl IntoIterator<Item = String>) {
    let resolver = 
        SimpleResolver::default().with_paths(
            [MODULE_PATH_JS, MODULE_PATH_SO, 
            #[cfg(debug_assertions)]
            WORKSPACE_TEMP
            ]
        );
    let loader = (
        // BuiltinLoader::default(),
        // ModuleLoader::default(),
        NativeLoader::default(),
        ScriptLoader::default(),
    );
    rt.set_loader(resolver, loader).await;
    let context = AsyncContext::full(&rt).await.unwrap();
    async_with!(context => |ctx| {
        api::add_api_obj(&ctx, args);
        run_js_source(&ctx, source, file_name).await;
    }).await;
    rt.idle().await;
}

async fn run_js_source<'js>(ctx: &Ctx<'js>, source: &[u8], file_name: &str) {
    let mod_evaluation = Module::evaluate(ctx.clone(),file_name, source).unwrap().into_future::<Value>().await;
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

