mod fs;

#[allow(non_snake_case)]
#[rquickjs::module]
mod outer {
    use std::{fs::File, io::{BufRead, BufReader}};
    use common::iterator::JsIterator;
    #[rquickjs::function]
    pub fn readFile(path: String) -> String {
        let rv = std::fs::read_to_string(path).unwrap();
        return rv;
    }
    #[rquickjs::function]
    pub fn readFileLines(filename: String) -> JsIterator<std::io::Lines<BufReader<File>>> {
        let file = File::open(filename).unwrap();
        let br = BufReader::new(file);
        let lines = br.lines();
        let jsiter = JsIterator::new(lines);
        return jsiter;
    }
}
rquickjs::module_init!(js_outer);
