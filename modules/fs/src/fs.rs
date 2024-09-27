use rquickjs::{module::ModuleDef, Function};

pub struct Fs;
impl ModuleDef for Fs {
    fn declare<'js>(decl: &rquickjs::module::Declarations<'js>) -> rquickjs::Result<()> {
        decl.declare("read_file").unwrap();
        Ok(())
    }

    fn evaluate<'js>(ctx: &rquickjs::Ctx<'js>, exports: &rquickjs::module::Exports<'js>) -> rquickjs::Result<()> {
        exports.export("read_file", Function::new(ctx.clone(), read_file)?.with_name("read_file")).unwrap();
        Ok(())
    }
}

fn read_file(path: String) -> String {
    println!("got here");
    let rv = std::fs::read_to_string(path).unwrap();
    println!("done reading");
    return rv;
}
