use logos::{Logos, SpannedIter};

#[derive(Debug)]
pub struct LexicalError;

impl std::fmt::Display for LexicalError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "LexicalError")
  }
}

fn remove_quotes(lex: &mut logos::Lexer<Token>) -> Option<String> {
  let slice = lex.slice();

  if slice.len() <= 2 {
    return Some(String::new());
  } else {
    Some(slice[1..slice.len() - 1].to_string())
  }
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
  #[regex(r"=")]
  Eq,
  #[token("export", priority = 4)]
  Export,
  #[regex(r"[\w\-]+", |lex| lex.slice().parse(), priority = 3)]
  Ident(String),
  #[regex(r"'[\w [^']]*'", remove_quotes)]
  #[regex(r"`[\w [^`]]*`", remove_quotes)]
  #[regex(r#""[\w [^"]]*""#, remove_quotes)]
  QuotedString(String),
  #[error]
  #[regex(r"#.*\n?", logos::skip)]
  #[regex(r"\s+", logos::skip)]
  Error,
}

impl std::fmt::Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

pub struct Lexer<'input> {
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
  type Item = Result<(usize, Token, usize), LexicalError>;

  fn next(&mut self) -> Option<Self::Item> {
    match self.token_stream.next() {
      Some((token, span)) => {
        println!("token: {:?}", token);
        match token {
          Token::Error => Some(Err(LexicalError)),
          _ => Some(Ok((span.start, token, span.end))),
        }
      }
      None => None,
    }
  }
}
