use std::boxed::FnBox;
use std::fmt;
use std::io;
use std::mem;

use iron::modifier::Modifier;
use iron::response::{Response, ResponseBody, WriteBody};
use maud::Utf8Writer;

pub struct Template(Box<FnBox(&mut fmt::Write) -> fmt::Result + Send>);

impl Template {
    pub fn new<T>(template: T) -> Template where
        T: FnOnce(&mut fmt::Write) -> fmt::Result + Send + 'static
    {
        Template(Box::new(template))
    }
}

impl<'a> FnOnce<(&'a mut fmt::Write,)> for Template {
    type Output = fmt::Result;
    extern "rust-call" fn call_once(self, args: (&mut fmt::Write,)) -> fmt::Result {
        self.0.call_box(args)
    }
}

impl Modifier<Response> for Template {
    fn modify(self, response: &mut Response) {
        let text_html = mime!(Text/Html; Charset=Utf8);
        let write_body = Box::new(Body::Available(self)) as Box<WriteBody + Send>;
        (text_html, write_body).modify(response)
    }
}

enum Body {
    Consumed,
    Available(Template),
}

impl WriteBody for Body {
    fn write_body(&mut self, body: &mut ResponseBody) -> io::Result<()> {
        match mem::replace(self, Body::Consumed) {
            Body::Available(template) => {
                let mut writer = Utf8Writer::new(body);
                let _ = template(&mut writer);
                writer.into_result()
            },
            Body::Consumed => Ok(()),
        }
    }
}
