use std::io::{BufRead, Read, StdinLock};

use common::bufread::JsBufRead;

pub struct JsStdin(pub StdinLock<'static>);

impl JsBufRead for JsStdin {
    fn read_line(&mut self, buffer: &mut String) -> std::io::Result<usize> {
        self.0.read_line(buffer)
    }

    fn read_to_string(&mut self, buffer: &mut String) -> std::io::Result<usize> {
        self.0.read_to_string(buffer)
    }
}
