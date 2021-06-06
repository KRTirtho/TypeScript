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
    Keywords(KeywordSyntaxKind)
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
