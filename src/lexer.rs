use logos::{Logos, SpannedIter};

use std::error::Error;

/// The error returned in case an environment fill is ill-formatted.
///
/// Read the [crate's](crate) documentation for information about the
/// format that `env-file-reader` supports.
///
/// **Example:**
///
/// ```rust
/// use env_file_reader::read_str;
///
/// fn main() {
///   let err = read_str("badly formatted env file")
///     .err()
///     .unwrap();
///
///   assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
///   assert_eq!(err.to_string(), "ParseError");
/// }
/// ```
///
#[derive(Debug)]
pub struct ParseError;

impl std::fmt::Display for ParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "ParseError")
  }
}

impl Error for ParseError {}

fn remove_quotes(lex: &mut logos::Lexer<Token>) -> String {
  let slice = lex.slice();

  if slice.len() <= 2 {
    return String::new();
  }

  let quote = &slice[..1];

  let escaped_quote_pattern = format!("\\{}", quote);
  let escaped_newline_pattern = "\\n";

  slice[1..slice.len() - 1]
    .replace(&escaped_quote_pattern, &quote)
    .replace(escaped_newline_pattern, "\n")
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
  #[token("=")]
  Eq,
  #[token("export")]
  Export,
  #[regex(r"\n\s*")]
  NewLine,
  #[regex(r#"[^\s='`"\#]+"#, |lex| lex.slice().parse())]
  Ident(String),
  #[regex(r"'[^']*'", remove_quotes)]
  #[regex(r"`[^`]*`", remove_quotes)]
  #[regex(r#""([^"]|\\")*""#, remove_quotes)]
  QuotedString(String),
  EOF,
  #[error]
  #[regex(r"#.*", logos::skip)]
  #[regex(r"\s+", logos::skip)]
  Error,
}

pub(crate) struct Lexer<'input> {
  token_stream: SpannedIter<'input, Token>,
  finished: bool,
}

impl<'input> Lexer<'input> {
  pub fn new(input: &'input str) -> Self {
    Self {
      token_stream: Token::lexer(input).spanned(),
      finished: false,
    }
  }

  fn finish(&mut self) {
    self.finished = true;
  }

  fn finished(&self) -> bool {
    self.finished
  }
}

impl<'input> Iterator for Lexer<'input> {
  type Item = Result<(usize, Token, usize), ParseError>;

  fn next(&mut self) -> Option<Self::Item> {
    match self.token_stream.next() {
      Some((token, span)) => {
        // TODO: remove
        println!("token: {:?}", token);
        match token {
          Token::Error => Some(Err(ParseError)),
          _ => Some(Ok((span.start, token, span.end))),
        }
      }
      None => {
        if self.finished() {
          None
        } else {
          self.finish();
          Some(Ok((0, Token::EOF, 0)))
        }
      }
    }
  }
}
