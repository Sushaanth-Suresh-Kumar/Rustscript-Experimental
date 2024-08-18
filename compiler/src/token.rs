#[derive(Debug)]
pub struct Token {
    // literal: &'a str,
    pub kind: TokenKind, // temporary pub
    // pub len: usize,
    pub span: Span, // might have to add file path/name later on
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Token {
        //len: usize,
        Token {
            // literal,
            kind,
            // len, // change it to span
            span,
        }
    }

    // pub fn is_keyword(literal: &str) -> bool {
    //     false
    // }

    pub fn literal_to_keyword(literal: &str) -> Option<KeywordKind> {
        match literal {
            // actual keywords
            "let" => Some(KeywordKind::Let),
            "const" => Some(KeywordKind::Const),
            "function" => Some(KeywordKind::Function),
            "fn" => Some(KeywordKind::Fn),
            "while" => Some(KeywordKind::While),
            "do" => Some(KeywordKind::Do),
            "for" => Some(KeywordKind::For),
            "continue" => Some(KeywordKind::Continue),
            "break" => Some(KeywordKind::Break),
            "return" => Some(KeywordKind::Return),
            "if" => Some(KeywordKind::If),
            "else" => Some(KeywordKind::Else),
            "struct" => Some(KeywordKind::Struct),
            "enum" => Some(KeywordKind::Enum),
            "switch" => Some(KeywordKind::Switch),
            "case" => Some(KeywordKind::Case),
            "true" => Some(KeywordKind::True),
            "false" => Some(KeywordKind::False),

            // types
            "int" => Some(KeywordKind::IntegerType),
            "float" => Some(KeywordKind::FloatType),
            "str" => Some(KeywordKind::StringType),
            "bool" => Some(KeywordKind::BooleanType),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Span {
    pub low: usize,
    pub high: usize,
}

impl Span {
    pub(crate) fn set(low: usize, high: usize) -> Span {
        Span { low, high }
    }
}

#[derive(PartialEq, Debug)]
pub enum TokenKind {
    // Multi Character Tokens
    /// "// comment"
    // TODO: Check how Javascript documentation works
    LineComment,

    /// "/* block comment */"
    // TODO: Check how Javascript documentation works
    BlockComment {
        terminated: bool,
    },

    /// " "
    /// All Whitespace characters
    Whitespace,

    /// Ident or Keyword
    ///
    /// example: "x", "let"  
    /// All Ident and Keywords are considered Ident
    Ident {
        name: String,
    },

    Keyword {
        kind: KeywordKind,
    },

    /// Invalid Identifier
    ///
    /// same as above but invalid unicode codepoints
    InvalidIdent,

    /// examples: "12u", "45i", "12f" (this one is considered int
    /// by default, since there is no floating point to begin with)
    /// , ""Hello World"", "1.9f", "23.2e-12", "12e+2"
    /// '_' inbetween shall be ignored, if placed inbetween digits
    /// but suffix shall not start with '_'
    Literal {
        kind: LiteralKind,
        // suffix_start: u32,
    },

    InvalidDecimal,
    InvalidExponent,

    /// "::"
    Scope,
    /// "->"
    Arrow,
    /// ||
    Or,
    /// &&
    And,
    /// !=
    NotEq,
    /// ==
    EqEq,
    /// >=
    GtEq,
    /// <=
    LtEq,

    /// "+="
    PlusEq,
    /// "-="
    MinusEq,
    /// "*="
    AsteriskEq,
    /// "/="
    SlashEq,
    /// "%="
    PercentEq,
    /// "^="
    CaretEq,

    // One Character Tokens
    /// ";"
    Semi,
    /// ":"
    Colon,
    // not adding " or ' since used by strings
    /// ","
    Comma,
    /// "."
    Dot,
    /// "/"
    Slash,
    /// "?"
    Question,
    ///"<"
    Lt,
    /// ">"
    Gt,
    /// "{"
    OpenBrace,
    /// "}"
    CloseBrace,
    /// "["
    OpenBracket,
    /// "]"
    CloseBracket,
    /// "|"
    Pipe,
    /// "\"
    BSlash,
    /// "~"
    Tilde,
    /// "`"
    Btick,
    /// "!"
    Bang,
    /// "@"
    At,
    /// "#"
    Sharp,
    /// "$"
    Dollar,
    /// "%"
    Percent,
    /// "^"
    Caret,
    /// "&"
    Ampersand,
    /// "*"
    Asterisk,
    /// "("
    OpenPara,
    /// ")"
    ClosePara,
    /// "-"
    Minus,
    // "_" maybe used by Identifiers
    /// "+"
    Plus,
    /// "="
    Eq,

    // finally we have to handle invalid character states
    /// Unknown token
    Unknown,

    /// End of Input
    Eof,
}

#[derive(Debug, PartialEq)]
pub enum LiteralKind {
    /// numbers with `\d+` are considered Int
    Int {
        // base: Base, // remove later
        value: isize,
    }, // TODO: there is one more field called 'empty_int' not sure why its used
    /// numbers with decimal or exponent are considered Float
    Float {
        // base: Base, // remove later
        value: f64,
    }, // TODO: there is one more field called 'empty_exponent' not sure why its used
    Str {
        terminated: bool,
        start: char,
        value: String,
    },
}
// TODO: notice there is no true or false in literal, also no void
// TODO: maybe add Complex type later if possible

// #[derive(Debug, PartialEq)]
// pub enum Base {
//     Binary = 2,
//     Octal = 8,
//     /// no prefix for decimal
//     Decimal = 10,
//     Hexadecimal = 16,
// }

#[derive(PartialEq, Debug)]
pub(crate) enum KeywordKind {
    Let,
    Const,
    Function,
    Fn,
    While,
    Do,
    For,
    Continue,
    Break,
    Return,
    If,
    Else,
    Struct,
    Enum,
    Switch,
    Case,
    IntegerType,
    FloatType,
    StringType,
    BooleanType,
    True,
    False,
}
