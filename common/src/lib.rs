#![feature(const_type_name)]
pub mod iterator;

use rquickjs::{atom::PredefinedAtom, Ctx, Function, Object, Value};
#[must_use]
pub fn value_to_string<'js>(ctx: &Ctx<'js>, js_value: Value<'js>) -> String {
    use rquickjs::Type;
    match js_value.type_of() {
        Type::String => js_value.into_string().unwrap().to_string().unwrap(),
        Type::Int => js_value.as_int().unwrap().to_string(),
        Type::Float => js_value.as_float().unwrap().to_string(),
        Type::Array => {
            let mut sval = Vec::with_capacity(10);
            sval.push(String::from("["));
            for value in js_value.into_array().unwrap().iter::<Value>() {
                let t = value_to_string(ctx, value.unwrap());
                sval.push(t);
                sval.push(String::from(", "));
            }
            sval.push(String::from("]"));
            return sval.concat();
        },
        Type::Object => {
            let v_obj_ref = js_value.as_object().unwrap();
            let object_iter = v_obj_ref.props::<Value, Value>();
            if object_iter.len() == 0 {
                return value_to_string(ctx, 
                    get_toString(js_value.into_object().unwrap())
                );
            }
            let mut sval = Vec::with_capacity(10);
            sval.push("{".to_string());
            for value in object_iter {
                let (k, value) = value.unwrap();
                if is_toString(&k) {
                    return value_to_string(ctx, 
                        get_toString(js_value.into_object().unwrap())
                    );
                }
                let t = value_to_string(ctx, k);
                sval.push(t);
                sval.push(": ".to_string());
                sval.push(value_to_string(ctx, value));
                sval.push(", ".to_string());
            }
            sval.push("}".to_string());
            return sval.concat();
        },
        _ => format!("{:?}", js_value),
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
fn get_toString(obj: Object<'_>) -> Value<'_> {
    let toString: rquickjs::Function = obj.get(PredefinedAtom::ToString).unwrap();
    let str_val = toString.call::<_, Value>((rquickjs::function::This(obj),));
    return str_val.unwrap();
}
#[allow(clippy::needless_pass_by_value)]
pub fn js_print<'js>(ctx: Ctx<'js>, v: Value<'js>) {
    print!("{}", value_to_string(&ctx, v));
}
pub fn js_println<'js>(ctx: Ctx<'js>, v: Value<'js>) {
    js_print(ctx, v);
    println!();
}
pub fn add_global_fn(ctx: &mut Ctx) {
    let globals = ctx.globals();
    globals.set("println", Function::new(ctx.clone(), js_println)).unwrap();
}
