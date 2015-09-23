use std::fmt;
use std::io;

use iron::response::{ResponseBody, WriteBody};
use maud::Utf8Writer;

pub struct Maud<T>(pub T);

impl<T> Maud<T> where
    T: FnMut(&mut fmt::Write) -> fmt::Result + Send + 'static
{
    pub fn new(template: T) -> Box<WriteBody + Send> {
        Box::new(Maud(template))
    }
}

impl<T> WriteBody for Maud<T> where
    T: FnMut(&mut fmt::Write) -> fmt::Result
{
    fn write_body(&mut self, body: &mut ResponseBody) -> io::Result<()> {
        let mut writer = Utf8Writer::new(body);
        let _ = (self.0)(&mut writer);
        writer.into_result()
    }
}
