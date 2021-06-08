mod compiler;

use compiler::scanner::{compute_line_starts, lookup_in_unicode_map};
use napi::{JsObject, Result};
use napi_derive::module_exports;

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
    exports.create_named_method("lookupInUnicodeMap", lookup_in_unicode_map)?;
    exports.create_named_method("computeLineStarts", compute_line_starts)?;
    Ok(())
}
