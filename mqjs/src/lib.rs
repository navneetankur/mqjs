use std::{env::Args, path::{Path, PathBuf}};

use rquickjs::{loader::{BuiltinLoader, FileResolver, ModuleLoader, NativeLoader, Resolver, ScriptLoader}, AsyncContext, AsyncRuntime, CatchResultExt, Ctx, Function, Module, Value};
const MODULE_PATH_JS: &str = "/home/navn/bin/mqjs/modules/js";
const MODULE_PATH_SO: &str = "/home/navn/bin/mqjs/modules/so";
const WORKSPACE_TEMP: &str = "/home/navn/workspace/rust/mqjs/target/debug";


pub async fn realmain(mut args: Args) {
    args.next(); //get rid of this program name.

    let resolver = 
        (
        SimpleResolver::default().with_paths(
            [MODULE_PATH_JS, MODULE_PATH_SO, WORKSPACE_TEMP]
        ),
        FileResolver::default()
        .with_paths(
            [MODULE_PATH_JS, MODULE_PATH_SO,
            // ["./", MODULE_PATH_JS, MODULE_PATH_SO,
            // #[cfg(debug_assertions)]
            WORKSPACE_TEMP,
            ]
        ).with_native(),
        );
    let loader = (
        BuiltinLoader::default(), ModuleLoader::default(),
        NativeLoader::default(), ScriptLoader::default(),
    );
    let rt = AsyncRuntime::new().unwrap();
    rt.set_loader(resolver, loader).await;
    let context = AsyncContext::full(&rt).await.unwrap();
    let script_name = args.next().expect("No script file provided.");
    context.with(|mut ctx| {
        api::add_api_obj(&mut ctx, args);
        add_global_fn(&mut ctx);
        run_js_file(&mut ctx, script_name);
    }).await;
    rt.idle().await;
}

fn run_js_file<'js>(ctx: &mut Ctx<'js>, file: String) {
    let source = std::fs::read(&file).unwrap();
    Module::evaluate(ctx.clone(),file, source).catch(ctx).unwrap();
}
// async fn js_runner<'js>(ctx: &mut Ctx<'js>, file: String) {
//     run_js_file(ctx, file).await
// }

fn print(v: Value) {
    use rquickjs::Type;
    match v.type_of() {
        Type::String => print!("{}", v.into_string().unwrap().to_string().unwrap()),
        Type::Int => print!("{}", v.as_int().unwrap()),
        Type::Float => print!("{}", v.as_float().unwrap()),
        Type::Array => {
            print!("[");
            for value in v.into_array().unwrap().iter::<Value>() {
                print(value.unwrap());
                print!(", ");
            }
            print!("]");
        },
        _ => print!("{:?}", v),
    }
}
fn println(v: Value) {
    print(v);
    println!();
}

fn add_global_fn(ctx: &mut Ctx) {
    let globals = ctx.globals();
    globals.set("println", Function::new(ctx.clone(), println)).unwrap();
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
        let to_find1 = PathBuf::from(name);
        let mut to_find2 = to_find1.clone();
        to_find2.set_extension("js");
        let mut to_find3 = to_find1.clone();
        to_find3.set_extension("so");
        let mut to_find4 = to_find1.clone();
        let stem = to_find4.file_stem().unwrap().to_str().unwrap();
        let stem = String::from("lib") + stem;
        to_find4.set_file_name(stem);
        to_find4.set_extension("so");
        let finds = [to_find1, to_find2, to_find3, to_find4];

        for path in &self.paths {
            for to_find in &finds {
                let to_look = path.join(to_find);
                if to_look.is_file() {
                    return Ok(to_look.to_string_lossy().into_owned());
                }

            }
        }
        return Err(rquickjs::Error::new_resolving(base, name));
    }
}

