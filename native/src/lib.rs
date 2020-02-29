#[macro_use]
extern crate neon;

use neon::prelude::*;
use neon::result::Throw;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

struct T {}

impl Task for T {
    type Output = String;
    type Error = ();
    type JsEvent = JsString;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        Ok(String::from("hello node"))
    }

    fn complete(
        self,
        mut cx: TaskContext,
        result: Result<Self::Output, Self::Error>,
    ) -> JsResult<Self::JsEvent> {
        match result {
            Ok(v) => Ok(cx.string(v)),
            _ => unreachable!(),
        }
    }
}

fn hello_async(mut cx: FunctionContext) -> JsResult<JsValue> {
    T {}.schedule(cx.argument(0)?)?;

    Ok(cx.undefined().upcast())
}

register_module!(mut cx, {
    cx.export_function("greet", hello)?;
    cx.export_function("greetAsync", hello_async)?;
});
