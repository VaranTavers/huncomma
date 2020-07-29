use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum PlainTextToken {
    // Tokens can be literal strings, of any length.
    #[regex("[0-9]+(,[0-9]+)?")]
    Number,

    #[regex("[.?!]")]
    EndOfSentence,

    #[token(",")]
    Comma,

    #[token("\n")]
    NewLine,

    // Or regular expressions.
    #[regex("[A-Za-zÀ-ÖØ-öø-ÿ]+")]
    Text,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\f\r]+", logos::skip)]
    Error,
}