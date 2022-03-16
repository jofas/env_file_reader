use logos::{Logos, SpannedIter};

use std::error::Error;

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
  #[regex(r#"[^\s='`"\#]+"#, |lex| lex.slice().parse())]
  Ident(String),
  #[regex(r"'[^']*'", remove_quotes)]
  #[regex(r"`[^`]*`", remove_quotes)]
  #[regex(r#""([^"]|\\")*""#, remove_quotes)]
  QuotedString(String),
  #[error]
  #[regex(r"#.*", logos::skip)]
  #[regex(r"\s+", logos::skip)]
  Error,
}

pub(crate) struct Lexer<'input> {
  token_stream: SpannedIter<'input, Token>,
}

impl<'input> Lexer<'input> {
  pub fn new(input: &'input str) -> Self {
    Self {
      token_stream: Token::lexer(input).spanned(),
    }
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
      None => None,
    }
  }
}
