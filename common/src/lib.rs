#![feature(const_type_name)]
pub mod iterator;
pub mod bufread;
pub mod rustdata;
pub mod thread;

use rquickjs::{atom::PredefinedAtom, Ctx, Function, IntoJs, Object, Value};
#[must_use]
pub fn value_to_string(js_value: Value<'_>) -> String {
    use rquickjs::Type;
    match js_value.type_of() {
        Type::String => js_value.into_string().unwrap().to_string().unwrap(),
        Type::Int => js_value.as_int().unwrap().to_string(),
        Type::Float => js_value.as_float().unwrap().to_string(),
        Type::Array => {
            let mut sval = Vec::with_capacity(10);
            sval.push(String::from("["));
            for value in js_value.into_array().unwrap().iter::<Value>() {
                let t = value_to_string(value.unwrap());
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
                return value_to_string(
                    get_toString(js_value.into_object().unwrap())
                );
            }
            let mut sval = Vec::with_capacity(10);
            sval.push("{".to_string());
            for value in object_iter {
                let (k, value) = value.unwrap();
                if is_toString(&k) {
                    return value_to_string(
                        get_toString(js_value.into_object().unwrap())
                    );
                }
                let t = value_to_string(k);
                sval.push(t);
                sval.push(": ".to_string());
                sval.push(value_to_string(value));
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
    if let Ok(toString) = obj.get::<_,rquickjs::Function>(PredefinedAtom::ToString) {
        let str_val = toString.call::<_, Value>((rquickjs::function::This(obj),));
        return str_val.unwrap();
    }
    else {
        let str_val = format!("{obj:?}");
        return str_val.into_js(obj.ctx()).unwrap();
    }
}
#[allow(clippy::needless_pass_by_value)]
pub fn js_print(v: Value<'_>) {
    print!("{}", value_to_string(v));
}
pub fn js_println(v: Value<'_>) {
    js_print(v);
    println!();
}
pub fn add_global_fn(ctx: &mut Ctx) {
    let globals = ctx.globals();
    globals.set("println", Function::new(ctx.clone(), js_println)).unwrap();
}
