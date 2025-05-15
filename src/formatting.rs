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

#[cfg(test)]
mod unit_tests {
    use colored::Styles;

    use super::*;

    #[test]
    fn test_tokenizer() {
        let test = "hi :3 (this is part of a format string (and another)[underline] )[bold] and this is not".to_string();
        let result = Parser::tokenize(test);
        assert_eq!(
            result.tokens,
            [
                Token::Literal("hi :3 ".to_string()),
                Token::OpenContent,
                Token::Literal("this is part of a format string ".to_string()),
                Token::OpenContent,
                Token::Literal("and another".to_string()),
                Token::CloseContent,
                Token::OpenStyle,
                Token::Literal("underline".to_string()),
                Token::CloseStyle,
                Token::Literal(" ".to_string()),
                Token::CloseContent,
                Token::OpenStyle,
                Token::Literal("bold".to_string()),
                Token::CloseStyle,
                Token::Literal(" and this is not".to_string())
            ]
        )
    }
    #[test]
    fn test_tokenizer_escaped() {
        let test = "hi :3 (this is part of a format string (and another)[underline] )[bold] and this is not \\(this is escaped\\)".to_string();
        let result = tokenize(test);
        assert_eq!(
            result,
            [
                Token::Literal("hi :3 ".to_string()),
                Token::OpenContent,
                Token::Literal("this is part of a format string ".to_string()),
                Token::OpenContent,
                Token::Literal("and another".to_string()),
                Token::CloseContent,
                Token::OpenStyle,
                Token::Literal("underline".to_string()),
                Token::CloseStyle,
                Token::Literal(" ".to_string()),
                Token::CloseContent,
                Token::OpenStyle,
                Token::Literal("bold".to_string()),
                Token::CloseStyle,
                Token::Literal(" and this is not (this is escaped)".to_string())
            ]
        )
    }

    #[test]
    fn basic_from_string() {
        let test = "hi guys".to_string();
        let test_struct = FormatString {
            nodes: vec![FormatStringNode::String("hi guys".to_string())],
        };
        let formatted = FormatString::try_from(test);
        assert_eq!(formatted.unwrap(), test_struct);
    }
    #[test]
    fn basic_from_formatted_string() {
        let test = "hi guys (this one is green)[fg:green] i am not green".to_string();
        let test_struct = FormatString {
            nodes: vec![
                FormatStringNode::String("hi guys ".to_string()),
                FormatStringNode::Formatted(FormatNode {
                    children: vec![FormatStringNode::String("this one is green".to_string())],
                    fg: Some(Color::Green),
                    ..Default::default()
                }),
                FormatStringNode::String(" i am not green".to_string()),
            ],
        };
        let formatted = FormatString::try_from(test);
        assert_eq!(formatted.unwrap(), test_struct);
    }
    #[test]
    fn basic_from_formatted_nested_string() {
        let test = "hi (guys)[underline] (this one is green( and this one is also underlined and bold!)[underline bold])[fg:green] i am not green".to_string();
        let test_struct = FormatString {
            nodes: vec![
                FormatStringNode::String("hi guys ".to_string()),
                FormatStringNode::Formatted(FormatNode {
                    children: vec![
                        FormatStringNode::String("this one is green".to_string()),
                        FormatStringNode::Formatted(FormatNode {
                            children: vec![FormatStringNode::String(
                                " and this one is also underlined and bold!".to_string(),
                            )],
                            style: Style::from_iter([Styles::Underline, Styles::Bold]),
                            ..Default::default()
                        }),
                    ],
                    fg: Some(Color::Green),
                    ..Default::default()
                }),
                FormatStringNode::String(" i am not green".to_string()),
            ],
        };
        let formatted = FormatString::try_from(test);
        assert_eq!(formatted.unwrap(), test_struct);
    }
}
