#[rquickjs::module]
mod fs {
    #[rquickjs::function]
    pub fn read_file(path: String) -> String {
        let rv = std::fs::read_to_string(path).unwrap();
        return rv;
    }
}
rquickjs::module_init!(js_fs);
