use logos::{Lexer, Logos};

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum LexicalError {
    #[default]
    UnknownToken,
}

fn string_slice(lex: &mut Lexer<'_, Tok>) -> String {
    lex.slice().to_string()
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = LexicalError)]
#[logos(skip r"[ \t\r\n\f]+")]
#[logos(skip r"//[^\r\n]*(\r\n|\n)?")] // single-line comments
#[logos(skip r"/\*([^*]|\*[^/])+\*/")] // block comments
pub enum Tok {
    #[token("auto")]
    Auto,
    #[token("break")]
    Break,
    #[token("case")]
    Case,
    #[token("char")]
    Char,
    #[token("const")]
    Const,
    #[token("continue")]
    Continue,
    #[token("default")]
    Default,
    #[token("do")]
    Do,
    #[token("double")]
    Double,
    #[token("else")]
    Else,
    #[token("enum")]
    Enum,
    #[token("extern")]
    Extern,
    #[token("float")]
    Float,
    #[token("for")]
    For,
    #[token("goto")]
    Goto,
    #[token("if")]
    If,
    #[token("int")]
    Int,
    #[token("long")]
    Long,
    #[token("register")]
    Register,
    #[token("return")]
    Return,
    #[token("short")]
    Short,
    #[token("signed")]
    Signed,
    #[token("sizeof")]
    Sizeof,
    #[token("static")]
    Static,
    #[token("struct")]
    Struct,
    #[token("switch")]
    Switch,
    #[token("typedef")]
    Typedef,
    #[token("union")]
    Union,
    #[token("unsigned")]
    Unsigned,
    #[token("void")]
    Void,
    #[token("volatile")]
    Volatile,
    #[token("while")]
    While,
    #[token("...")]
    Ellipsis,
    #[token(">>=")]
    RightAssign,
    #[token("<<=")]
    LeftAssign,
    #[token("+=")]
    AddAssign,
    #[token("-=")]
    SubAssign,
    #[token("*=")]
    MulAssign,
    #[token("/=")]
    DivAssign,
    #[token("%=")]
    ModAssign,
    #[token("&=")]
    AndAssign,
    #[token("^=")]
    XorAssign,
    #[token("|=")]
    OrAssign,
    #[token(">>")]
    RightOp,
    #[token("<<")]
    LeftOp,
    #[token("++")]
    IncOp,
    #[token("--")]
    DecOp,
    #[token("->")]
    PtrOp,
    #[token("&&")]
    AndOp,
    #[token("||")]
    OrOp,
    #[token("<=")]
    LeOp,
    #[token(">=")]
    GeOp,
    #[token("==")]
    EqOp,
    #[token("!=")]
    NeOp,
    #[token(";")]
    Semicolon,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token("=")]
    Assign,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token(".")]
    Dot,
    #[token("&")]
    Ampersand,
    #[token("!")]
    Exclamation,
    #[token("~")]
    Tilde,
    #[token("-")]
    Minus,
    #[token("+")]
    Plus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,
    #[token("^")]
    Caret,
    #[token("|")]
    Pipe,
    #[token("?")]
    Question,

    #[regex(r"[a-zA-Z_]([a-zA-Z0-9_])*", string_slice)]
    Identifier(String),

    #[regex(r"0[xX][a-fA-F0-9_]+[UuLl]?", string_slice)]
    #[regex(r"0[0-9]+[UuLl]?", string_slice)]
    #[regex(r"[1-9][0-9]*[UuLl]?", string_slice)]
    // TODO: Wtf is L?'(\\.|[^\\'])+'?? See: https://www.lysator.liu.se/c/ANSI-C-grammar-l.html
    #[regex(r"[0-9]+[eE][+-]?[0-9]+[FfLl]?", string_slice)]
    #[regex(r"[0-9]*\.[0-9]+[eE][+-]?[0-9]+[FfLl]?", string_slice, priority = 6)]
    #[regex(r"[0-9]+\.[0-9]*[eE][+-]?[0-9]+[FfLl]?", string_slice)]
    Constant(String),

    #[regex(r#"[a-zA-Z_]?"([^"\\]|\\[\s\S])*""#, string_slice)]
    StringLiteral(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests for the actual Logos lexer without our wrapper
    mod direct {
        use super::*;

        #[test]
        fn works_as_expected() {
            assert_eq!(
                Tok::lexer(concat!(
                "auto break case char const continue default do double else enum extern float for",
                " goto if int long register return short signed sizeof static struct switch",
                " typedef union unsigned void volatile while ... >>= <<= += -= *= /= %= &= ^= |=",
                " >> << ++ -- -> && || <= >= == != ; { } , : = ( ) [ ] . & ! ~ - + * / % < > ^ | ?",
                " ident _ident _1 0xFF 0xFFu 0XFL 09L 42e3 10e-3 4e+5 4e4L 0.1e+17f r\"abc\"",
                "\"def\" \"ghi\njkl\""
            ))
                .collect::<Vec<_>>(),
                {
                    use Tok::*;
                    vec![
                        Ok(Auto),
                        Ok(Break),
                        Ok(Case),
                        Ok(Char),
                        Ok(Const),
                        Ok(Continue),
                        Ok(Default),
                        Ok(Do),
                        Ok(Double),
                        Ok(Else),
                        Ok(Enum),
                        Ok(Extern),
                        Ok(Float),
                        Ok(For),
                        Ok(Goto),
                        Ok(If),
                        Ok(Int),
                        Ok(Long),
                        Ok(Register),
                        Ok(Return),
                        Ok(Short),
                        Ok(Signed),
                        Ok(Sizeof),
                        Ok(Static),
                        Ok(Struct),
                        Ok(Switch),
                        Ok(Typedef),
                        Ok(Union),
                        Ok(Unsigned),
                        Ok(Void),
                        Ok(Volatile),
                        Ok(While),
                        Ok(Ellipsis),
                        Ok(RightAssign),
                        Ok(LeftAssign),
                        Ok(AddAssign),
                        Ok(SubAssign),
                        Ok(MulAssign),
                        Ok(DivAssign),
                        Ok(ModAssign),
                        Ok(AndAssign),
                        Ok(XorAssign),
                        Ok(OrAssign),
                        Ok(RightOp),
                        Ok(LeftOp),
                        Ok(IncOp),
                        Ok(DecOp),
                        Ok(PtrOp),
                        Ok(AndOp),
                        Ok(OrOp),
                        Ok(LeOp),
                        Ok(GeOp),
                        Ok(EqOp),
                        Ok(NeOp),
                        Ok(Semicolon),
                        Ok(LeftBrace),
                        Ok(RightBrace),
                        Ok(Comma),
                        Ok(Colon),
                        Ok(Assign),
                        Ok(LeftParen),
                        Ok(RightParen),
                        Ok(LeftBracket),
                        Ok(RightBracket),
                        Ok(Dot),
                        Ok(Ampersand),
                        Ok(Exclamation),
                        Ok(Tilde),
                        Ok(Minus),
                        Ok(Plus),
                        Ok(Star),
                        Ok(Slash),
                        Ok(Percent),
                        Ok(LessThan),
                        Ok(GreaterThan),
                        Ok(Caret),
                        Ok(Pipe),
                        Ok(Question),
                        Ok(Identifier("ident".to_string())),
                        Ok(Identifier("_ident".to_string())),
                        Ok(Identifier("_1".to_string())),
                        Ok(Constant("0xFF".to_string())),
                        Ok(Constant("0xFFu".to_string())),
                        Ok(Constant("0XFL".to_string())),
                        Ok(Constant("09L".to_string())),
                        Ok(Constant("42e3".to_string())),
                        Ok(Constant("10e-3".to_string())),
                        Ok(Constant("4e+5".to_string())),
                        Ok(Constant("4e4L".to_string())),
                        Ok(Constant("0.1e+17f".to_string())),
                        Ok(StringLiteral("r\"abc\"".to_string())),
                        Ok(StringLiteral("\"def\"".to_string())),
                        Ok(StringLiteral("\"ghi\njkl\"".to_string())),
                    ]
                }
            );
        }
    }
}
