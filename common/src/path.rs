use std::{os::unix::fs::PermissionsExt, path::{Path, PathBuf}};
use rquickjs::{class::{Borrow, OwnedBorrow, OwnedBorrowMut}, Ctx, IntoJs, Object, Value};

use crate::{class_chore, object_fn};

class_chore!(JsPath, get_proto);
pub struct JsPath {
    v: PathBuf,
}
type ThisMut<'js> = rquickjs::function::This<OwnedBorrowMut<'js, JsPath>>;
type This<'js> = rquickjs::function::This<OwnedBorrow<'js, JsPath>>;
type ThisClass<'js> = rquickjs::class::Class<'js, JsPath>;

fn clear(mut this: ThisMut) -> ThisClass {
    this.v.clear();
    return this.0.into_inner();
}
fn reserve(mut this: ThisMut, additional: usize) -> ThisClass {
    this.v.reserve(additional);
    return this.0.into_inner();
}
fn shrink_to_fit(mut this: ThisMut) -> ThisClass {
    this.v.shrink_to_fit();
    return this.0.into_inner();
}
fn push<'js>(mut this: ThisMut<'js>, value: Value<'js>) -> ThisClass<'js> {
    this.v.push(value_to_path(&value).path());
    return this.0.into_inner();
}
fn pop(mut this: ThisMut) -> ThisClass {
    this.v.pop();
    return this.0.into_inner();
}
fn set_filename<'js>(mut this: ThisMut<'js>, value: Value<'js>) -> ThisClass<'js> {
    this.v.set_file_name(value_to_path(&value).path());
    return this.0.into_inner();
}
fn set_extension<'js>(mut this: ThisMut<'js>, value: Value<'js>) -> ThisClass<'js> {
    this.v.set_extension(value_to_path(&value).path());
    return this.0.into_inner();
}
fn is_absolute(this: This) -> bool {
    this.v.is_absolute()
}
fn is_relative(this: This) -> bool {
    this.v.is_relative()
}
fn parent<'js>(ctx: Ctx<'js>, this: This<'js>) -> Option<Value<'js>> {
    let parent = this.v.parent()?;
    let jspath = JsPath::from(parent);
    return Some(jspath.into_js(&ctx).unwrap());
}
fn filename(this: This) -> Option<String> {
    Some(this.v.file_name()?.to_str().unwrap().to_string())
}
fn strip_prefix<'js>(ctx: Ctx<'js>, this: This<'js>, base: Value<'js>) -> Option<Value<'js>> {
    let pathy = value_to_path(&base);
    let base = pathy.path();
    let stripped = this.v.strip_prefix(base).ok()?;
    let jsp = JsPath::from(stripped);
    return Some(jsp.into_js(&ctx).unwrap());
}
fn starts_with<'js>(this: This<'js>, base: Value<'js>) -> bool {
    let pathy = value_to_path(&base);
    return this.v.starts_with(pathy.path());
}
fn ends_with<'js>(this: This<'js>, base: Value<'js>) -> bool {
    let pathy = value_to_path(&base);
    return this.v.ends_with(pathy.path());
}
fn file_stem<'js>(ctx: Ctx<'js>, this: This<'js>) -> Option<Value<'js>> {
    let stem = this.v.file_stem()?;
    let jspath = JsPath::from(PathBuf::from(stem));
    return Some(jspath.into_js(&ctx).unwrap());
}
fn extension<'js>(ctx: Ctx<'js>, this: This<'js>) -> Option<Value<'js>> {
    let extension = this.v.extension()?;
    let jspath = JsPath::from(PathBuf::from(extension));
    return Some(jspath.into_js(&ctx).unwrap());
}
fn join<'js>(this: This<'js>, base: Value<'js>) -> JsPath {
    let pathy = value_to_path(&base);
    return this.v.join(pathy.path()).into()
}
fn components<'js>(this: This<'js>) -> Vec<JsPath> {
    this.v.iter().map(|v| JsPath::from(PathBuf::from(v)) ).collect()
}
fn canonicalize<'js>(this: This<'js>) -> Option<JsPath> {
    Some(this.v.canonicalize().ok()?.as_path().into())
}
fn readlink<'js>(this: This<'js>) -> Option<JsPath> {
    Some(this.v.read_link().ok()?.as_path().into())
}
fn exists<'js>(this: This<'js>) -> bool {
    this.v.exists()
}
fn is_file<'js>(this: This<'js>) -> bool {
    this.v.is_file()
}
fn is_dir<'js>(this: This<'js>) -> bool {
    this.v.is_dir()
}
fn is_symlink<'js>(this: This<'js>) -> bool {
    this.v.is_symlink()
}
fn string<'js>(this: This<'js>) -> String {
    this.v.to_str().unwrap().to_string()
}
fn to_string<'js>(this: This<'js>) -> String {
    this.v.to_str().unwrap().to_string()
}
fn str<'js>(this: This<'js>) -> String {
    this.v.to_str().unwrap().to_string()
}
fn permission<'js>(this: This<'js>) -> Option<u32> {
    Some(this.v.metadata().ok()?.permissions().mode())
}
fn modified<'js>(this: This<'js>) -> Option<std::time::SystemTime> {
    this.v.metadata().ok()?.modified().ok()
}
fn accessed<'js>(this: This<'js>) -> Option<std::time::SystemTime> {
    this.v.metadata().ok()?.accessed().ok()
}
fn created<'js>(this: This<'js>) -> Option<std::time::SystemTime> {
    this.v.metadata().ok()?.created().ok()
}

enum Pathy<'a,'js> {
    Buf(PathBuf),
    Ref(Borrow<'a,'js, JsPath>),
}
impl<'a,'js> Pathy<'a,'js> {
    fn path(&self) -> &Path {
        use Pathy::{Buf, Ref};
        match self {
            Buf(path) => path,
            Ref(path) => &path.v
        }
    }
}

fn value_to_path<'a, 'js: 'a>(value: &'a Value<'js>) -> Pathy<'a,'js> {
    if let Some(path) = value.as_string() {
        // let path = path.to_string().unwrap();
        let path = PathBuf::from(path.to_string().unwrap());
        return Pathy::Buf(path);
    }
    else if let Some(path) = value.as_object() {
        let path = path.as_class::<JsPath>().expect("not a path");
        return Pathy::Ref(path.borrow());
    }
    else {
        panic!("not a path");
    }
}

fn get_proto<'js>(ctx: &Ctx<'js>) -> Object<'js> {
    let proto = Object::new(ctx.clone()).unwrap();
    object_fn!(proto,
        clear,
        reserve,
        shrink_to_fit,
        push,
        pop,
        set_filename,
        set_extension,
        is_absolute,
        is_relative,
        parent,
        filename,
        strip_prefix,
        starts_with,
        ends_with,
        file_stem,
        extension,
        join,
        components,
        canonicalize,
        readlink,
        exists,
        is_file,
        is_dir,
        is_symlink,
        string,
        to_string,
        str,
        permission,
        modified,
        accessed,
        created,
    );
    return proto;
}

impl From<PathBuf> for JsPath {
    fn from(v: PathBuf) -> Self {
        Self { v }
    }
}
impl From<&Path> for JsPath {
    fn from(value: &Path) -> Self {
        Self { v: value.to_path_buf() }
    }
}
