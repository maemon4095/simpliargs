use std::{borrow::Cow, str::FromStr};

/// Specific location of tokens
/// ```text
/// Ex) The Location points below
///             ↓
/// $ bin.exe args bbb ccc
///           │ └ offset_in_token: count of bytes before the character. In this example, it will be 2 == 'a'.len_utf8() + 'r'.len_utf8()
///           └───── token_offset: count of tokens before the token. In this example, token "args" is first argument and it will be 0
/// ```
pub struct Location {
    /// count of bytes before the character.
    pub token_offset: usize,
    /// count of tokens before the token.
    pub offset_in_token: usize,
}

pub trait Input: Sized {
    type Token: AsRef<str>;
    type TokenIter: Iterator<Item = Self::Token>;

    fn tokens(&self) -> Self::TokenIter;
    /// advance specific count of token
    fn advance(self, count: usize) -> Self;
    /// Location points the char at specific offset in current first token.
    fn location(&self, offset: usize) -> Location;
}

pub trait Parse: Sized {
    fn parse<T: Input>(input: T) -> Result<(T, Self), Error>;
}

impl<S: FromStr> Parse for S
where
    S::Err: 'static + std::error::Error,
{
    fn parse<T: Input>(input: T) -> Result<(T, Self), Error> {
        let mut tokens = input.tokens();
        let start = input.location(0);

        let Some(token) = tokens.next() else {
            return Err(Error::UnexpectedEndOfInput {
                span: (start..).into(),
                description: Cow::Borrowed("unexpected end of input."),
            });
        };

        match S::from_str(token.as_ref()) {
            Ok(v) => {
                drop(tokens);
                Ok((input.advance(1), v))
            }
            Err(e) => Err(Error::InvalidValue {
                span: (start).into(),
                inner: Box::new(e),
            }),
        }
    }
}

pub enum Span {
    Range(Location, Location),
    From(Location),
    Pointed(Location),
}

impl Span {
    pub fn new(range: impl Into<Self>) -> Self {
        range.into()
    }
}

impl From<Location> for Span {
    fn from(value: Location) -> Self {
        Span::Pointed(value)
    }
}

impl From<std::ops::Range<Location>> for Span {
    fn from(value: std::ops::Range<Location>) -> Self {
        Self::Range(value.start, value.end)
    }
}

impl From<std::ops::RangeFrom<Location>> for Span {
    fn from(value: std::ops::RangeFrom<Location>) -> Self {
        Self::From(value.start)
    }
}

pub enum Error {
    MissingRequiredOption {
        span: Span,
        name: Cow<'static, str>,
        description: Cow<'static, str>,
    },
    MissingRequiredArgument {
        span: Span,
        name: Cow<'static, str>,
        description: Cow<'static, str>,
    },
    InvalidValue {
        span: Span,
        inner: Box<dyn std::error::Error>,
    },
    UnexpectedEndOfInput {
        span: Span,
        description: Cow<'static, str>,
    },
    UnexpectedInput {
        span: Span,
        description: Cow<'static, str>,
    },
}
