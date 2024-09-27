#![feature(const_type_name)]
pub mod iterator;

use rquickjs::{Ctx, Function, Value};
pub fn js_print(v: Value) {
    use rquickjs::Type;
    match v.type_of() {
        Type::String => print!("{}", v.into_string().unwrap().to_string().unwrap()),
        Type::Int => print!("{}", v.as_int().unwrap()),
        Type::Float => print!("{}", v.as_float().unwrap()),
        Type::Array => {
            print!("[");
            for value in v.into_array().unwrap().iter::<Value>() {
                js_print(value.unwrap());
                print!(", ");
            }
            print!("]");
        },
        _ => print!("{:?}", v),
    }
}
pub fn js_println(v: Value) {
    js_print(v);
    println!();
}
pub fn add_global_fn(ctx: &mut Ctx) {
    let globals = ctx.globals();
    globals.set("println", Function::new(ctx.clone(), js_println)).unwrap();
}
