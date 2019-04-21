use std::fmt;
// use std::str::Chars;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
enum Token {
  OpenParen,
  CloseParen,
  Atom(String),
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Token::OpenParen => write!(f, "("),
      Token::CloseParen => write!(f, ")"),
      Token::Atom(s) => write!(f, "{}", s),
    }
  }
}

#[cfg(test)]
fn atom(s: &str) -> Token {
  Token::Atom(s.to_owned())
}

fn is_ascii_whitespace(b: u8) -> bool {
  b == b' ' || b == b'\t' || b == b'\n' || b == b'\r'
}

fn is_valid_name_char(b: u8) -> bool {
  !(is_ascii_whitespace(b) || b == b'(' || b == b')')
}

fn parse_tokens(s: &str) -> Vec<Token> {
  use Token::*;
  // Note we assume ASCII and operate on bytes throughout.
  let mut result = vec![];
  let bytes = s.as_bytes();
  let mut i: usize = 0;
  while i < bytes.len() {
    if is_ascii_whitespace(bytes[i]) {
      i += 1;
      continue;
    }
    match bytes[i] {
      b'(' => {
        result.push(OpenParen);
        i += 1;
      }
      b')' => {
        result.push(CloseParen);
        i += 1;
      }
      _ => {
        assert!(is_valid_name_char(bytes[i]));
        let start = i;
        while i < bytes.len() && is_valid_name_char(bytes[i]) {
          i += 1
        }
        result.push(Atom(
          std::str::from_utf8(&bytes[start..i]).unwrap().to_owned(),
        ))
      }
    }
  }
  result
}

#[test]
fn test_parse_tokens() {
  use Token::*;

  assert_eq!(parse_tokens(""), vec![]);
  assert_eq!(
    parse_tokens("(A S (D)F)"),
    vec![
      OpenParen,
      atom("A"),
      atom("S"),
      OpenParen,
      atom("D"),
      CloseParen,
      atom("F"),
      CloseParen
    ]
  );
  assert_eq!(parse_tokens("       (     "), vec![OpenParen]);
  assert_eq!(parse_tokens("asdf"), vec![atom("asdf")]);
  assert_eq!(
    parse_tokens("(np~asdf~det wiggily)  woo)"),
    vec![
      OpenParen,
      atom("np~asdf~det"),
      atom("wiggily"),
      CloseParen,
      atom("woo"),
      CloseParen
    ]
  );
}

#[derive(PartialEq, Eq)]
pub struct Node {
  head: String,
  kids: Vec<Node>,
}

impl fmt::Debug for Node {
  // TODO: This could be better.
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.kids.is_empty() {
      write!(f, "{}", self.head)
    } else {
      write!(f, "({} {:?})", self.head, self.kids)
    }
  }
}

fn leaf(head: &str) -> Node {
  Node {
    head: head.to_owned(),
    kids: vec![],
  }
}

#[cfg(test)]
fn node(head: &str, kids: Vec<Node>) -> Node {
  Node {
    head: head.to_owned(),
    kids: kids,
  }
}

#[cfg(test)]
fn cons(head: &str, kid: &str) -> Node {
  node(head, vec![leaf(kid)])
}

pub type ParseErr = String;

fn fail<A>(s: &str) -> Result<A, ParseErr> {
  Err(s.to_owned())
}

struct Parser<'a> {
  tokens: &'a [Token],
}

impl<'a> Parser<'a> {
  fn done(&self) -> bool {
    self.tokens.is_empty()
  }

  // Removes and returns next token.
  fn next_token(&mut self) -> Option<&Token> {
    if self.done() {
      None
    } else {
      let t = &self.tokens[0];
      self.tokens = &self.tokens[1..];
      Some(t)
    }
  }

  fn expect_atom(&mut self) -> Result<String, ParseErr> {
    match self.next_token() {
      None => fail("Expected word token but found end of input"),
      // TODO: Get rid of this copy?
      Some(Token::Atom(s)) => Ok(s.to_owned()),
      Some(t) => Err(format!("Expected word token, found {}", t)),
    }
  }

  fn parse(&mut self) -> Result<Node, ParseErr> {
    let mut node: Node;
    match self.next_token() {
      None => {
        return fail("Expected '(' or word, found end of input");
      }
      Some(Token::Atom(s)) => {
        return Ok(leaf(s));
      }
      Some(Token::OpenParen) => {
        node = Node {
          head: self.expect_atom()?,
          kids: vec![],
        };
      }
      Some(t) => return Err(format!("Expected '(' or word, found {}", t)),
    }
    // Parse kids.
    loop {
      if self.done() {
        return fail("Expected ), found end of input");
      }
      match self.tokens[0] {
        Token::CloseParen => {
          self.next_token();
          return Ok(node);
        }
        _ => node.kids.push(self.parse()?),
      }
    }
  }
}

impl FromStr for Node {
  type Err = ParseErr;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let tokens = parse_tokens(s);
    let mut parser = Parser { tokens: &tokens };
    let result = parser.parse()?;
    if !parser.done() {
      Err(format!(
        "Failed to consume entire input: {:?}",
        parser.tokens
      ))
    } else {
      Ok(result)
    }
  }
}

#[test]
fn test_parse() {
  assert_eq!(Node::from_str("bob"), Ok(leaf("bob")));
  assert_eq!(
    Node::from_str("(A B C)"),
    Ok(Node {
      head: "A".to_owned(),
      kids: vec![leaf("B"), leaf("C")]
    })
  );

  assert_eq!(
    Node::from_str(
      r#"
    (TOP
      (S (NP (DT The) (NN luxury) (NN auto) (NN maker))
      (NP (JJ last) (NN year))
      (VP (VBD sold) (NP (CD 1,214) (NNS cars))
      (PP (IN in) (NP (DT the) (NNP U.S.))))))
  "#
    ),
    Ok(node(
      "TOP",
      vec![node(
        "S",
        vec![
          node(
            "NP",
            vec![
              cons("DT", "The"),
              cons("NN", "luxury"),
              cons("NN", "auto"),
              cons("NN", "maker"),
            ]
          ),
          node("NP", vec![cons("JJ", "last"), cons("NN", "year"),]),
          node(
            "VP",
            vec![
              cons("VBD", "sold"),
              node("NP", vec![cons("CD", "1,214"), cons("NNS", "cars"),]),
              node(
                "PP",
                vec![
                  cons("IN", "in"),
                  node("NP", vec![cons("DT", "the"), cons("NNP", "U.S."),]),
                ]
              )
            ]
          )
        ]
      )]
    ))
  );
}
