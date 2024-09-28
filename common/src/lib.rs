#![feature(const_type_name)]
pub mod iterator;

use rquickjs::{atom::PredefinedAtom, Ctx, Function, Object, Value};
#[must_use]
pub fn value_to_string(ctx: &Ctx, v: Value) -> String {
    use rquickjs::Type;
    match v.type_of() {
        Type::String => v.into_string().unwrap().to_string().unwrap(),
        Type::Int => v.as_int().unwrap().to_string(),
        Type::Float => v.as_float().unwrap().to_string(),
        Type::Array => {
            let mut sval = Vec::with_capacity(10);
            sval.push(String::from("["));
            for value in v.into_array().unwrap().iter::<Value>() {
                let t = value_to_string(ctx, value.unwrap());
                sval.push(t);
                sval.push(String::from(", "));
            }
            sval.push(String::from("]"));
            return sval.concat();
        },
        Type::Object => {
            let v_obj_ref = v.as_object().unwrap();
            let object_iter = v_obj_ref.props::<Value, Value>();
            if object_iter.len() == 0 {
                return get_toString(ctx, v.into_object().unwrap());
            }
            let mut sval = Vec::with_capacity(10);
            sval.push("{".to_string());
            for value in object_iter {
                let (k, v) = value.unwrap();
                if is_toString(&k) {
                    return get_toString(ctx, v.into_object().unwrap());
                }
                let t = value_to_string(ctx, k);
                sval.push(t);
                sval.push(": ".to_string());
                sval.push(value_to_string(ctx, v));
                sval.push(", ".to_string());
            }
            sval.push("}".to_string());
            return sval.concat();
        },
        _ => format!("{:?}", v),
    }
}

#[allow(non_snake_case)]
fn is_toString(k: &Value<'_>) -> bool {
    if let Some(key) = k.as_string() {
        if key.to_string().unwrap() == "toString" {
            return true;
        }
    }
    return false;
}
#[allow(non_snake_case)]
fn get_toString(ctx: &Ctx, obj: Object<'_>) -> String {
    let toString: rquickjs::Function = obj.get(PredefinedAtom::ToString).unwrap();
    let str_val = toString.call((rquickjs::prelude::This(obj),));
    if let Err(str_val) = &str_val {
        assert!(!str_val.is_exception(), "{:?}", ctx.catch());
    }
    return str_val.unwrap();
}
#[allow(clippy::needless_pass_by_value)]
pub fn js_print(ctx: Ctx, v: Value) {
    print!("{}", value_to_string(&ctx, v));
}
pub fn js_println(ctx: Ctx, v: Value) {
    js_print(ctx, v);
    println!();
}
pub fn add_global_fn(ctx: &mut Ctx) {
    let globals = ctx.globals();
    globals.set("println", Function::new(ctx.clone(), js_println)).unwrap();
}
