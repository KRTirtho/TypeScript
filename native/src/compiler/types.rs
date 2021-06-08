use std::collections::HashMap;

pub enum TriviaSyntaxKind {
    SingleLineCommentTrivia,
    MultiLineCommentTrivia,
    NewLineTrivia,
    WhitespaceTrivia,
    // We detect and preserve #! on the first line
    ShebangTrivia,
    // We detect and provide better error recovery when we encounter a git merge marker.  This
    // allows us to edit files with git-conflict markers in them in a much more pleasant manner.
    ConflictMarkerTrivia,
}

pub enum LiteralSyntaxKind {
    NumericLiteral,
    BigIntLiteral,
    StringLiteral,
    JsxText,
    JsxTextAllWhiteSpaces,
    RegularExpressionLiteral,
    NoSubstitutionTemplateLiteral,
}

pub enum PseudoLiteralSyntaxKind {
    TemplateHead,
    TemplateMiddle,
    TemplateTail,
}

pub enum PunctuationSyntaxKind {
    OpenBraceToken,
    CloseBraceToken,
    OpenParenToken,
    CloseParenToken,
    OpenBracketToken,
    CloseBracketToken,
    DotToken,
    DotDotDotToken,
    SemicolonToken,
    CommaToken,
    QuestionDotToken,
    LessThanToken,
    LessThanSlashToken,
    GreaterThanToken,
    LessThanEqualsToken,
    GreaterThanEqualsToken,
    EqualsEqualsToken,
    ExclamationEqualsToken,
    EqualsEqualsEqualsToken,
    ExclamationEqualsEqualsToken,
    EqualsGreaterThanToken,
    PlusToken,
    MinusToken,
    AsteriskToken,
    AsteriskAsteriskToken,
    SlashToken,
    PercentToken,
    PlusPlusToken,
    MinusMinusToken,
    LessThanLessThanToken,
    GreaterThanGreaterThanToken,
    GreaterThanGreaterThanGreaterThanToken,
    AmpersandToken,
    BarToken,
    CaretToken,
    ExclamationToken,
    TildeToken,
    AmpersandAmpersandToken,
    BarBarToken,
    QuestionToken,
    ColonToken,
    AtToken,
    QuestionQuestionToken,
    /** Only the JSDoc scanner produces BacktickToken. The normal scanner produces NoSubstitutionTemplateLiteral and related kinds. */
    BacktickToken,
    /** Only the JSDoc scanner produces HashToken. The normal scanner produces PrivateIdentifier. */
    HashToken,
    // Assignments
    EqualsToken,
    PlusEqualsToken,
    MinusEqualsToken,
    AsteriskEqualsToken,
    AsteriskAsteriskEqualsToken,
    SlashEqualsToken,
    PercentEqualsToken,
    LessThanLessThanEqualsToken,
    GreaterThanGreaterThanEqualsToken,
    GreaterThanGreaterThanGreaterThanEqualsToken,
    AmpersandEqualsToken,
    BarEqualsToken,
    BarBarEqualsToken,
    AmpersandAmpersandEqualsToken,
    QuestionQuestionEqualsToken,
    CaretEqualsToken,
}

pub enum ModifierSyntaxKind {
    AbstractKeyword,
    AsyncKeyword,
    ConstKeyword,
    DeclareKeyword,
    DefaultKeyword,
    ExportKeyword,
    PrivateKeyword,
    ProtectedKeyword,
    PublicKeyword,
    ReadonlyKeyword,
    OverrideKeyword,
    StaticKeyword,
}

pub enum KeywordTypeSyntaxKind {
    AnyKeyword,
    BigIntKeyword,
    VoidKeyword,
    BooleanKeyword,
    IntrinsicKeyword,
    NeverKeyword,
    NumberKeyword,
    ObjectKeyword,
    StringKeyword,
    SymbolKeyword,
    UndefinedKeyword,
    UnknownKeyword,
}

pub enum KeywordSyntaxKind {
    // Reserved words
    BreakKeyword,
    CaseKeyword,
    CatchKeyword,
    ClassKeyword,
    ContinueKeyword,
    DebuggerKeyword,
    DeleteKeyword,
    DoKeyword,
    ElseKeyword,
    EnumKeyword,
    ExtendsKeyword,
    FalseKeyword,
    FinallyKeyword,
    ForKeyword,
    FunctionKeyword,
    IfKeyword,
    ImportKeyword,
    InKeyword,
    InstanceOfKeyword,
    NewKeyword,
    NullKeyword,
    ReturnKeyword,
    SuperKeyword,
    SwitchKeyword,
    ThisKeyword,
    ThrowKeyword,
    TrueKeyword,
    TryKeyword,
    TypeOfKeyword,
    VarKeyword,

    WhileKeyword,
    WithKeyword,

    Modifiers(ModifierSyntaxKind),

    KeywordType(KeywordTypeSyntaxKind),

    // Strict mode reserved words
    ImplementsKeyword,
    InterfaceKeyword,
    LetKeyword,
    PackageKeyword,
    YieldKeyword,
    // Contextual keywords
    AsKeyword,
    AssertsKeyword,

    AwaitKeyword,

    ConstructorKeyword,
    GetKeyword,
    InferKeyword,

    IsKeyword,
    KeyOfKeyword,
    ModuleKeyword,
    NamespaceKeyword,

    RequireKeyword,

    SetKeyword,

    TypeKeyword,

    UniqueKeyword,

    FromKeyword,
    GlobalKeyword,

    OfKeyword, // LastKeyword and LastToken and LastContextualKeyword
}

pub enum TypeNodeSyntaxKind {
    TypePredicate,
    TypeReference,
    FunctionType,
    ConstructorType,
    TypeQuery,
    TypeLiteral,
    ArrayType,
    TupleType,
    OptionalType,
    RestType,
    UnionType,
    IntersectionType,
    ConditionalType,
    InferType,
    ParenthesizedType,
    ThisType,
    TypeOperator,
    IndexedAccessType,
    MappedType,
    LiteralType,
    NamedTupleMember,
    TemplateLiteralType,
    TemplateLiteralTypeSpan,
    ImportType,
    ExpressionWithTypeArguments,
    JSDocTypeExpression,
    JSDocAllType,     // The * type
    JSDocUnknownType, // The ? type
    JSDocNullableType,
    JSDocNonNullableType,
    JSDocOptionalType,
    JSDocFunctionType,
    JSDocVariadicType,
    JSDocNamepathType, // https://jsdoc.app/about-namepaths.html
    JSDocTypeLiteral,
    JSDocSignature,
}

pub enum TokenSyntaxKind {
    Unknown,
    EndOfFileToken,
    Identifier,
    Trivias(TriviaSyntaxKind),
    Literals(LiteralSyntaxKind),
    PseudoLiterals(PseudoLiteralSyntaxKind),
    Punctuations(PunctuationSyntaxKind),
    Keywords(KeywordSyntaxKind),
}

pub enum JsxTokenSyntaxKind {
    LessThanSlashToken,
    EndOfFileToken,
    ConflictMarkerTrivia,
    JsxText,
    JsxTextAllWhiteSpaces,
    OpenBraceToken,
    LessThanToken,
}

pub enum JSDocSyntaxKind {
    EndOfFileToken,
    WhitespaceTrivia,
    AtToken,
    NewLineTrivia,
    AsteriskToken,
    OpenBraceToken,
    CloseBraceToken,
    LessThanToken,
    GreaterThanToken,
    OpenBracketToken,
    CloseBracketToken,
    EqualsToken,
    CommaToken,
    DotToken,
    Identifier,
    BacktickToken,
    HashToken,
    Unknown,
    Keywords(KeywordSyntaxKind),
}

pub enum SyntaxKind {
    Trivias(TriviaSyntaxKind),

    // Literals
    Literals(LiteralSyntaxKind),
    // Pseudo-literals
    PseudoLiterals(PseudoLiteralSyntaxKind),
    // Punctuation
    Punctuations(PunctuationSyntaxKind),
    // Identifiers and PrivateIdentifiers
    PrivateIdentifier,

    // Keywords
    Keywords(KeywordSyntaxKind),

    // Names
    QualifiedName,
    ComputedPropertyName,
    // Signature elements
    TypeParameter,
    Parameter,
    Decorator,
    // TypeMember
    PropertySignature,
    PropertyDeclaration,
    MethodSignature,
    MethodDeclaration,
    Constructor,
    GetAccessor,
    SetAccessor,
    CallSignature,
    ConstructSignature,
    IndexSignature,
    // Type
    TypeNodes(TypeNodeSyntaxKind),

    // token syntax
    Tokens(TokenSyntaxKind),

    // Binding patterns
    ObjectBindingPattern,
    ArrayBindingPattern,
    BindingElement,
    // Expression
    ArrayLiteralExpression,
    ObjectLiteralExpression,
    PropertyAccessExpression,
    ElementAccessExpression,
    CallExpression,
    NewExpression,
    TaggedTemplateExpression,
    TypeAssertionExpression,
    ParenthesizedExpression,
    FunctionExpression,
    ArrowFunction,
    DeleteExpression,
    TypeOfExpression,
    VoidExpression,
    AwaitExpression,
    PrefixUnaryExpression,
    PostfixUnaryExpression,
    BinaryExpression,
    ConditionalExpression,
    TemplateExpression,
    YieldExpression,
    SpreadElement,
    ClassExpression,
    OmittedExpression,

    AsExpression,
    NonNullExpression,
    MetaProperty,
    SyntheticExpression,

    // Misc
    TemplateSpan,
    SemicolonClassElement,
    // Element
    Block,
    EmptyStatement,
    VariableStatement,
    ExpressionStatement,
    IfStatement,
    DoStatement,
    WhileStatement,
    ForStatement,
    ForInStatement,
    ForOfStatement,
    ContinueStatement,
    BreakStatement,
    ReturnStatement,
    WithStatement,
    SwitchStatement,
    LabeledStatement,
    ThrowStatement,
    TryStatement,
    DebuggerStatement,
    VariableDeclaration,
    VariableDeclarationList,
    FunctionDeclaration,
    ClassDeclaration,
    InterfaceDeclaration,
    TypeAliasDeclaration,
    EnumDeclaration,
    ModuleDeclaration,
    ModuleBlock,
    CaseBlock,
    NamespaceExportDeclaration,
    ImportEqualsDeclaration,
    ImportDeclaration,
    ImportClause,
    NamespaceImport,
    NamedImports,
    ImportSpecifier,
    ExportAssignment,
    ExportDeclaration,
    NamedExports,
    NamespaceExport,
    ExportSpecifier,
    MissingDeclaration,

    // Module references
    ExternalModuleReference,

    // JSX
    JsxElement,
    JsxSelfClosingElement,
    JsxOpeningElement,
    JsxClosingElement,
    JsxFragment,
    JsxOpeningFragment,
    JsxClosingFragment,
    JsxAttribute,
    JsxAttributes,
    JsxSpreadAttribute,
    JsxExpression,

    // Clauses
    CaseClause,
    DefaultClause,
    HeritageClause,
    CatchClause,

    // Property assignments
    PropertyAssignment,
    ShorthandPropertyAssignment,
    SpreadAssignment,

    // Enum
    EnumMember,
    // Unparsed
    UnparsedPrologue,
    UnparsedPrepend,
    UnparsedText,
    UnparsedInternalText,
    UnparsedSyntheticReference,

    // Top-level nodes
    SourceFile,
    Bundle,
    UnparsedSource,
    InputFiles,

    // JSDoc nodes
    JSDocNameReference,
    JSDocMemberName, // C#p

    JSDocComment,
    JSDocText,

    JSDocLink,
    JSDocLinkCode,
    JSDocLinkPlain,
    JSDocTag,
    JSDocAugmentsTag,
    JSDocImplementsTag,
    JSDocAuthorTag,
    JSDocDeprecatedTag,
    JSDocClassTag,
    JSDocPublicTag,
    JSDocPrivateTag,
    JSDocProtectedTag,
    JSDocReadonlyTag,
    JSDocOverrideTag,
    JSDocCallbackTag,
    JSDocEnumTag,
    JSDocParameterTag,
    JSDocReturnTag,
    JSDocThisTag,
    JSDocTypeTag,
    JSDocTemplateTag,
    JSDocTypedefTag,
    JSDocSeeTag,
    JSDocPropertyTag,

    // Synthesized list
    SyntaxList,

    // Transformation nodes
    NotEmittedStatement,
    PartiallyEmittedExpression,
    CommaListExpression,
    MergeDeclarationMarker,
    EndOfDeclarationMarker,
    SyntheticReferenceExpression,

    // Enum value count
    Count,
    // Markers (turned of due to rust's difference of Enum with Typescript)
    // FirstAssignment = SyntaxKind::EqualsToken,
    // LastAssignment = SyntaxKind::CaretEqualsToken,
    // FirstCompoundAssignment = SyntaxKind::PlusEqualsToken,
    // LastCompoundAssignment = SyntaxKind::CaretEqualsToken,
    // FirstReservedWord = SyntaxKind::BreakKeyword,
    // LastReservedWord = SyntaxKind::WithKeyword,
    // FirstKeyword = SyntaxKind::BreakKeyword,
    // LastKeyword = SyntaxKind::OfKeyword,
    // FirstFutureReservedWord = SyntaxKind::ImplementsKeyword,
    // LastFutureReservedWord = SyntaxKind::YieldKeyword,
    // FirstTypeNode = SyntaxKind::TypePredicate,
    // LastTypeNode = SyntaxKind::ImportType,
    // FirstPunctuation = SyntaxKind::OpenBraceToken,
    // LastPunctuation = SyntaxKind::CaretEqualsToken,
    // FirstToken = SyntaxKind::Unknown,
    // LastToken = SyntaxKind::LastKeyword,
    // FirstTriviaToken = SyntaxKind::SingleLineCommentTrivia,
    // LastTriviaToken = SyntaxKind::ConflictMarkerTrivia,
    // FirstLiteralToken = SyntaxKind::NumericLiteral,
    // LastLiteralToken = SyntaxKind::NoSubstitutionTemplateLiteral,
    // FirstTemplateToken = SyntaxKind::NoSubstitutionTemplateLiteral,
    // LastTemplateToken = SyntaxKind::TemplateTail,
    // FirstBinaryOperator = SyntaxKind::LessThanToken,
    // LastBinaryOperator = SyntaxKind::CaretEqualsToken,
    // FirstStatement = SyntaxKind::VariableStatement,
    // LastStatement = SyntaxKind::DebuggerStatement,
    // FirstNode = SyntaxKind::QualifiedName,
    // FirstJSDocNode = SyntaxKind::JSDocTypeExpression,
    // LastJSDocNode = SyntaxKind::JSDocPropertyTag,
    // FirstJSDocTagNode = SyntaxKind::JSDocTag,
    // LastJSDocTagNode = SyntaxKind::JSDocPropertyTag,
    // /* @internal */ FirstContextualKeyword = SyntaxKind::AbstractKeyword,
    // /* @internal */ LastContextualKeyword = SyntaxKind::OfKeyword,
}

pub enum TokenFlags {
    None = 0,
    /* @internal */
    PrecedingLineBreak = 1 << 0,
    /* @internal */
    PrecedingJSDocComment = 1 << 1,
    /* @internal */
    Unterminated = 1 << 2,
    /* @internal */
    ExtendedUnicodeEscape = 1 << 3,
    Scientific = 1 << 4,      // e.g. `10e2`
    Octal = 1 << 5,           // e.g. `0777`
    HexSpecifier = 1 << 6,    // e.g. `0x00000000`
    BinarySpecifier = 1 << 7, // e.g. `0b0110010000000000`
    OctalSpecifier = 1 << 8,  // e.g. `0o777`
    /* @internal */
    ContainsSeparator = 1 << 9, // e.g. `0b1100_0101`
    /* @internal */
    UnicodeEscape = 1 << 10,
    /* @internal */
    ContainsInvalidEscape = 1 << 11, // e.g. `\uhello`
    /* @internal */
    BinaryOrOctalSpecifier = 1 << 7 | 1 << 8,
    /* @internal */
    NumericLiteralFlags = 1 << 4 | 1 << 5 | 1 << 6 | 1 << 7 | 1 << 8 | 1 << 9,
    /* @internal */
    // TemplateLiteralLikeFlags = 1 << 11,
}

pub enum DiagnosticCategory {
    Warning,
    Error,
    Suggestion,
    Message,
}

pub struct IDiagnosticMessage<'a> {
    key: &'a str,
    category: DiagnosticCategory,
    code: u16,
    message: String,
    reports_unnecessary: Option<HashMap<&'a str, &'a str>>,
    reports_deprecated: Option<HashMap<&'a str, &'a str>>,
    /* @internal */
    elided_in_compatibility_pyramid: Option<bool>,
}

pub struct TextRange {
    pub pos: usize,
    pub end: usize,
}

pub enum CommentDirectiveType {
    ExpectError,
    Ignore,
}

pub struct CommentDirective {
    pub range: TextRange,
    pub _type: CommentDirectiveType,
}

pub enum ScriptTarget {
    ES3 = 0,
    ES5 = 1,
    ES2015 = 2,
    ES2016 = 3,
    ES2017 = 4,
    ES2018 = 5,
    ES2019 = 6,
    ES2020 = 7,
    ES2021 = 8,
    ESNext = 99,
    JSON = 100,
}

pub enum LanguageVariant {
    Standard,
    JSX,
}
pub mod CharacterCodes {
    pub const NULL_CHARACTER: u32 = 0;
    pub const MAX_ASCII_CHARACTER: u32 = 0x7F;
    pub const LINE_FEED: u32 = 0x0A; // \
    pub const CARRIAGE_RETURN: u32 = 0x0D; // \
    pub const LINE_SEPARATOR: u32 = 0x2028;
    pub const PARAGRAPH_SEPARATOR: u32 = 0x2029;
    pub const NEXT_LINE: u32 = 0x0085;
    pub const SPACE: u32 = 0x0020; // " "
    pub const NON_BREAKING_SPACE: u32 = 0x00A0; //
    pub const EN_QUAD: u32 = 0x2000;
    pub const EM_QUAD: u32 = 0x2001;
    pub const EN_SPACE: u32 = 0x2002;
    pub const EM_SPACE: u32 = 0x2003;
    pub const THREE_PER_EM_SPACE: u32 = 0x2004;
    pub const FOUR_PER_EM_SPACE: u32 = 0x2005;
    pub const SIX_PER_EM_SPACE: u32 = 0x2006;
    pub const FIGURE_SPACE: u32 = 0x2007;
    pub const PUNCTUATION_SPACE: u32 = 0x2008;
    pub const THIN_SPACE: u32 = 0x2009;
    pub const HAIR_SPACE: u32 = 0x200A;
    pub const ZERO_WIDTH_SPACE: u32 = 0x200B;
    pub const NARROW_NO_BREAK_SPACE: u32 = 0x202F;
    pub const IDEOGRAPHIC_SPACE: u32 = 0x3000;
    pub const MATHEMATICAL_SPACE: u32 = 0x205F;
    pub const OGHAM: u32 = 0x1680;
    pub const LODASH: u32 = 0x5F;
    pub const DOLLAR: u32 = 0x24;
    pub const _0: u32 = 0x30;
    pub const _1: u32 = 0x31;
    pub const _2: u32 = 0x32;
    pub const _3: u32 = 0x33;
    pub const _4: u32 = 0x34;
    pub const _5: u32 = 0x35;
    pub const _6: u32 = 0x36;
    pub const _7: u32 = 0x37;
    pub const _8: u32 = 0x38;
    pub const _9: u32 = 0x39;
    pub const AA: u32 = 0x61;
    pub const BB: u32 = 0x62;
    pub const CC: u32 = 0x63;
    pub const DD: u32 = 0x64;
    pub const EE: u32 = 0x65;
    pub const FF: u32 = 0x66;
    pub const GG: u32 = 0x67;
    pub const HH: u32 = 0x68;
    pub const II: u32 = 0x69;
    pub const JJ: u32 = 0x6A;
    pub const KK: u32 = 0x6B;
    pub const LL: u32 = 0x6C;
    pub const MM: u32 = 0x6D;
    pub const NN: u32 = 0x6E;
    pub const OO: u32 = 0x6F;
    pub const PP: u32 = 0x70;
    pub const QQ: u32 = 0x71;
    pub const RR: u32 = 0x72;
    pub const SS: u32 = 0x73;
    pub const TT: u32 = 0x74;
    pub const UU: u32 = 0x75;
    pub const VV: u32 = 0x76;
    pub const WW: u32 = 0x77;
    pub const XX: u32 = 0x78;
    pub const YY: u32 = 0x79;
    pub const ZZ: u32 = 0x7A;
    pub const A: u32 = 0x41;
    pub const B: u32 = 0x42;
    pub const C: u32 = 0x43;
    pub const D: u32 = 0x44;
    pub const E: u32 = 0x45;
    pub const F: u32 = 0x46;
    pub const G: u32 = 0x47;
    pub const H: u32 = 0x48;
    pub const I: u32 = 0x49;
    pub const J: u32 = 0x4A;
    pub const K: u32 = 0x4B;
    pub const L: u32 = 0x4C;
    pub const M: u32 = 0x4D;
    pub const N: u32 = 0x4E;
    pub const O: u32 = 0x4F;
    pub const P: u32 = 0x50;
    pub const Q: u32 = 0x51;
    pub const R: u32 = 0x52;
    pub const S: u32 = 0x53;
    pub const T: u32 = 0x54;
    pub const U: u32 = 0x55;
    pub const V: u32 = 0x56;
    pub const W: u32 = 0x57;
    pub const X: u32 = 0x58;
    pub const Y: u32 = 0x59;
    pub const Z: u32 = 0x5a;
    pub const AMPERSAND: u32 = 0x26; // &
    pub const ASTERISK: u32 = 0x2A; // *
    pub const AT: u32 = 0x40; // @
    pub const BACKSLASH: u32 = 0x5C; // \
    pub const BACKTICK: u32 = 0x60; // `
    pub const BAR: u32 = 0x7C; // |
    pub const CARET: u32 = 0x5E; // ^
    pub const CLOSE_BRACE: u32 = 0x7D; // }
    pub const CLOSE_BRACKET: u32 = 0x5D; // ]
    pub const CLOSE_PAREN: u32 = 0x29; // )
    pub const COLON: u32 = 0x3A; // :
    pub const COMMA: u32 = 0x2C; // ,
    pub const DOT: u32 = 0x2E; // .
    pub const DOUBLE_QUOTE: u32 = 0x22; // "
    pub const EQUALS: u32 = 0x3D; // =
    pub const EXCLAMATION: u32 = 0x21; // !
    pub const GREATER_THAN: u32 = 0x3E; // >
    pub const HASH: u32 = 0x23; // #
    pub const LESS_THAN: u32 = 0x3C; // <
    pub const MINUS: u32 = 0x2D; // -
    pub const OPEN_BRACE: u32 = 0x7B; // {
    pub const OPEN_BRACKET: u32 = 0x5B; // [
    pub const OPEN_PAREN: u32 = 0x28; // (
    pub const PERCENT: u32 = 0x25; // %
    pub const PLUS: u32 = 0x2B; // +
    pub const QUESTION: u32 = 0x3F; // ?
    pub const SEMICOLON: u32 = 0x3B; // ;
    pub const SINGLE_QUOTE: u32 = 0x27; // '
    pub const SLASH: u32 = 0x2F; // /
    pub const TILDE: u32 = 0x7E; // ~
    pub const BACKSPACE: u32 = 0x08; // \b
    pub const FORM_FEED: u32 = 0x0C; // \f
    pub const BYTE_ORDER_MARK: u32 = 0xFEFF;
    pub const TAB: u32 = 0x09; // \t
    pub const VERTICAL_TAB: u32 = 0x0B; // \v
}
