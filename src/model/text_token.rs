use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum PlainTextToken {
    // Tokens can be literal strings, of any length.
    #[regex("[0-9]+(,[0-9]+)?")]
    Number,

    #[regex("[.?!]")]
    EndOfSentence,

    #[regex("[;,]")]
    Comma,

    #[token("\n")]
    NewLine,

    #[regex("[[()]{}]+")]
    Parentheses,

    // matches any kind of letter from any language
    #[regex("\\pL+")]
    Text,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\f\r]+", logos::skip)]
    Error,

}