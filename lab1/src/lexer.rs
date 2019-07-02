pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Clone, PartialEq, Eq, Copy, Debug)]
pub enum Tok {
    /// "//..." or "/*...*/"
    Comment,

    /// "int"
    Int,

    /// "return"
    Return,

    /// ;
    Semi,

    /// -
    Minus,
    /// +
    Add,
    /// *
    Star,
    /// /
    Slash,
    /// %
    Percent, 

    /// \n
    Newline,
    /// =
    Equal,
    /// +=
    PlusEqual,
    /// -=
    MinusEqual,
    /// *=
    StarEqual,
    /// /=
    SlashEqual,
    /// %=
    PercentEqual,

    /// (
    LeftParen,
    /// )
    RightParen,
    /// {
    LeftBrace,
    /// }
    RightBrace,

    Name {
        name: String
    },
    
    Num {
        value: i64
    },

    // reserved
    /// "--" for now
    ReservedSymbol,
    /// struct typedef if else while for continue break return assert true false NULL alloc alloc_array bool void char string (int)
    ReservedKeyword,
}

#[derive(Debug)]
pub enum LexicalError {
    UsedReservedKeyword {
        used: String
    },
}

pub struct Lexer<'input> {
    src: 
}
