use colored::{Color, Style};

#[derive(Debug, PartialEq)]
pub struct FormatString {
    nodes: Vec<FormatStringNode>,
}
#[derive(thiserror::Error, Debug)]
pub enum FormatError {
    #[error("Incomplete format string")]
    Incomplete(String),
}
#[derive(Debug, PartialEq)]
enum FormatStringNode {
    String(String),
    Formatted(FormatNode),
}
#[derive(Debug, PartialEq, Default)]
struct FormatNode {
    fg: Option<Color>,
    bg: Option<Color>,
    style: Style,
    children: Vec<FormatStringNode>,
}

#[derive(Debug, PartialEq)]
enum Token {
    OpenContent,
    CloseContent,
    OpenStyle,
    CloseStyle,
    Literal(String),
}

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn tokenize(input: String) -> Self {
        let mut tokens = vec![];
        let mut tmp = String::new();
        let mut escaping = false;
        input.chars().for_each(|ch| match ch {
            '(' | ')' | '[' | ']' if !escaping => {
                if !tmp.is_empty() {
                    tokens.push(Token::Literal(tmp.clone()));
                    tmp.clear();
                }
                match ch {
                    '(' => tokens.push(Token::OpenContent),
                    ')' => tokens.push(Token::CloseContent),
                    '[' => tokens.push(Token::OpenStyle),
                    ']' => tokens.push(Token::CloseStyle),
                    _ => unreachable!(),
                }
            }
            '\\' if !escaping => {
                escaping = true;
            }
            c => {
                tmp.push(c);
                if escaping {
                    escaping = false;
                }
            }
        });
        if !tmp.is_empty() {
            tokens.push(Token::Literal(tmp));
        }
        Self { tokens, pos: 0 }
    }
}

impl TryFrom<String> for FormatString {
    type Error = FormatError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        todo!()
    }
}
