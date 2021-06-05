mod compiler{
  pub mod scanner;
  pub mod types;
}

use compiler::scanner::lookup_in_unicode_map;
use neon::prelude::{Context, FunctionContext, JsResult, JsString, ModuleContext, NeonResult};

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("lookupInUnicodeMap", lookup_in_unicode_map)?;
    Ok(())
}
