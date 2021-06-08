use std::convert::TryInto;

use napi::{CallContext, JsBoolean, JsFunction, JsNumber, JsObject, JsString, Result};
use napi_derive::js_function;

use super::types::{
    CharacterCodes, CommentDirective, IDiagnosticMessage, JSDocSyntaxKind, JsxTokenSyntaxKind,
    LanguageVariant, PunctuationSyntaxKind, ScriptTarget, SyntaxKind, TokenFlags, TokenSyntaxKind,
};

pub type ErrorCallback<'a> = dyn Fn(IDiagnosticMessage<'a>) -> &'a str;

pub fn tokenIsIdentifierOrKeyword(token: SyntaxKind) -> bool {
    match token {
        SyntaxKind::Tokens(TokenSyntaxKind::Identifier) => true,
        SyntaxKind::Keywords(_) => true,
        _ => false,
    }
}

pub fn token_is_identifier_or_keyword_or_greater_than(token: SyntaxKind) -> bool {
    if let SyntaxKind::Punctuations(PunctuationSyntaxKind::GreaterThanToken) = token {
        return true;
    }
    tokenIsIdentifierOrKeyword(token)
}

pub trait Scanner {
    fn get_start_pos() -> usize;
    fn get_token() -> SyntaxKind;
    fn get_text_pos() -> usize;
    fn get_token_pos() -> usize;
    fn get_token_text() -> String;
    fn get_token_value() -> String;
    fn has_unicode_escape() -> bool;
    fn has_extended_unicode_escape() -> bool;
    fn has_preceding_line_break() -> bool;
    /* @internal */
    fn has_preceding_jsdoc_comment() -> bool;
    fn is_identifier() -> bool;
    fn is_reserved_word() -> bool;
    fn is_unterminated() -> bool;
    /* @internal */
    fn get_numeric_literal_flags() -> TokenFlags;
    /* @internal */
    fn get_comment_directives() -> Option<CommentDirective>;
    /* @internal */
    fn get_token_flags() -> TokenFlags;
    fn re_scan_greater_token() -> SyntaxKind;
    fn re_scan_slash_token() -> SyntaxKind;
    fn re_scan_asterisk_equals_token() -> SyntaxKind;
    fn re_scan_template_token(is_tagged_template: bool) -> SyntaxKind;
    fn re_scan_template_head_or_no_substitution_template() -> SyntaxKind;
    fn scan_jsx_identifier() -> SyntaxKind;
    fn scan_jsx_attribute_value() -> SyntaxKind;
    fn re_scan_jsx_attribute_value() -> SyntaxKind;
    fn re_scan_jsx_token(allow_multiline_jsx_text: Option<bool>) -> JsxTokenSyntaxKind;
    fn re_scan_less_than_token() -> SyntaxKind;
    fn re_scan_hash_tokenken() -> SyntaxKind;
    fn re_scan_question_token() -> SyntaxKind;
    fn re_scan_invalid_identifier() -> SyntaxKind;
    fn scan_jsx_token() -> JsxTokenSyntaxKind;
    fn scan_js_doc_token() -> JSDocSyntaxKind;
    fn scan() -> SyntaxKind;

    fn get_text() -> String;
    /* @internal */
    fn clear_comment_directives();
    // Sets the text for the scanner to scan.  An optional subrange
    // can be provided to have the scanner only scan a portion of the
    fn set_text(text: Option<String>, start: Option<usize>, length: Option<usize>);
    fn set_on_error(on_error: Option<Box<ErrorCallback>>);
    fn set_script_target(script_target: ScriptTarget);
    fn set_language_variant(variant: LanguageVariant);
    fn set_text_pos(text_pos: Option<usize>);
    /* @internal */
    fn set_in_jsdoc_type(in_type: bool);
    // Invokes the provided callback then unconditionally restores the
    // was in immediately prior to invoking the callback.  The result
    // is returned from this function.
    fn look_ahead<T>(callback: dyn Fn() -> T) -> T;

    // Invokes the callback with the scanner set to scan the specified
    // returns, the scanner is restored to the state it was in before
    fn scan_range<T>(start: usize, length: usize, callback: dyn Fn() -> T) -> T;

    // Invokes the provided callback.  If the callback returns
    // the scanner to the state it was in immediately prior to
    // callback returns something truthy, then the scanner state is
    // of invoking the callback is returned from this function.
    fn try_scan<T>(callback: dyn Fn() -> T) -> T;
}

const COMMENT_DIRECTIVE_REG_EX_SINGLE_LINE: &str = "^///?\\s*@(ts-expect-error|ts-ignore)";

const COMMENT_DIRECTIVE_REG_EX_MULTI_LINE: &str = "^(?:/|*)*\\s*@(ts-expect-error|ts-ignore)";

#[js_function(2)]
pub fn lookup_in_unicode_map(cx: CallContext) -> Result<JsBoolean> {
    let code: u32 = cx
        .get::<JsNumber>(0)?
        .try_into()
        .expect("argument of type number expected");
    let array_of_number_err = "expected an array";
    let map_raw = cx.get::<JsObject>(1).expect(array_of_number_err);
    let mut map: Vec<u32> = Vec::new();
    let validating_err = "error on array validating";
    if map_raw.is_array().expect(validating_err) {
        for index in 0..map_raw.get_array_length().expect(validating_err) {
            map.push(
                map_raw
                    .get_element::<JsNumber>(index)
                    .expect("element in has correct index but doesn't exist")
                    .try_into()
                    .expect(array_of_number_err),
            );
        }
    }
    if code < map[0] {
        return cx.env.get_boolean(false);
    }

    let mut lo = 0;
    let mut hi = map.len();
    let mut mid: usize;

    while lo + 1 < hi {
        mid = lo + (hi - lo) / 2;
        // mid has to be even to catch a range's beginning
        mid -= mid % 2;
        if map[mid] <= code && code <= map[mid + 1] {
            return cx.env.get_boolean(true);
        }

        if code < map[mid] {
            hi = mid;
        } else {
            lo = mid + 2;
        }
    }

    cx.env.get_boolean(true)
}

fn is_line_break(ch: u32) -> bool {
    ch == CharacterCodes::LINE_FEED
        || ch == CharacterCodes::CARRIAGE_RETURN
        || ch == CharacterCodes::LINE_SEPARATOR
        || ch == CharacterCodes::PARAGRAPH_SEPARATOR
}

#[js_function(2)]
pub fn compute_line_starts(cx: CallContext) -> Result<JsObject> {
    let str_err = "expected a string";
    let text: String = cx
        .get::<JsString>(0)
        .expect(str_err)
        .into_utf8()
        .expect(str_err)
        .as_str()
        .expect(str_err)
        .to_string();
    let u32_err = "failure at converting u32 to JsNumber";

    let mut result = cx.env.create_array()?;
    let mut res_index = 0;

    for (index, arg) in text.chars().enumerate() {
        let ch = arg as u32;
        if index == 0 {
            result
                .set_element(res_index, cx.env.create_uint32(0).expect(u32_err))
                .expect(u32_err);
            res_index += 1;
        }
        if (ch == CharacterCodes::LINE_FEED)
            || ch > CharacterCodes::MAX_ASCII_CHARACTER && is_line_break(ch)
        {
            result
                .set_element(
                    res_index,
                    cx.env.create_uint32((index + 1) as u32).expect(u32_err),
                )
                .expect(u32_err);
            res_index += 1;
            continue;
        }
    }

    Ok(result)
}
