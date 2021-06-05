use neon::prelude::*;

use super::types::{
    CommentDirective, IDiagnosticMessage, JSDocSyntaxKind, JsxTokenSyntaxKind, LanguageVariant,
    PunctuationSyntaxKind, ScriptTarget, SyntaxKind, TokenFlags, TokenSyntaxKind,
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

pub fn lookup_in_unicode_map(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let code = cx
        .argument::<JsNumber>(0)
        .expect("argument of type number expected")
        .value(&mut cx) as usize;
    let map_raw = cx
        .argument::<JsArray>(1)
        .expect("expected map to be an array");
    let map: Vec<usize> = map_raw
        .to_vec(&mut cx)
        .unwrap_or(Vec::new())
        .into_iter()
        .map(|raw_val| {
            raw_val
                .downcast(&mut cx)
                .unwrap_or(JsNumber::new(&mut cx, 0))
                .value(&mut cx) as usize
        })
        .collect();
    if code < map[0] {
        return Ok(cx.boolean(false));
    }

    let mut lo = 0;
    let mut hi = map.len();
    let mut mid: usize;

    while lo + 1 < hi {
        mid = lo + (hi - lo) / 2;
        // mid has to be even to catch a range's beginning
        mid -= mid % 2;
        if map[mid] <= code && code <= map[mid + 1] {
            return Ok(cx.boolean(true));
        }

        if code < map[mid] {
            hi = mid;
        } else {
            lo = mid + 2;
        }
    }

    Ok(cx.boolean(false))
}
