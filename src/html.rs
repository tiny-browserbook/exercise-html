use crate::dom::{AttrMap, Element, Node};
use combine::error::ParseError;
use combine::parser::char::char;
use combine::{parser, Parser, Stream};

/// `attribute` consumes `name="value"`.
fn attribute<Input>() -> impl Parser<Input, Output = (String, String)>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    todo!("you need to implement this combinator");
    (char(' ')).map(|_| ("".to_string(), "".to_string()))
}

/// `attributes` consumes `name1="value1" name2="value2" ... name="value"`
fn attributes<Input>() -> impl Parser<Input, Output = AttrMap>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    todo!("you need to implement this combinator");
    (char(' ')).map(|_| AttrMap::new())
}

/// `open_tag` consumes `<tag_name attr_name="attr_value" ...>`.
fn open_tag<Input>() -> impl Parser<Input, Output = (String, AttrMap)>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    todo!("you need to implement this combinator");
    (char(' ')).map(|_| ("".to_string(), AttrMap::new()))
}

/// close_tag consumes `</tag_name>`.
fn close_tag<Input>() -> impl Parser<Input, Output = String>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    todo!("you need to implement this combinator");
    (char(' ')).map(|_| ("".to_string()))
}

// `nodes_` (and `nodes`) tries to parse input as Element or Text.
fn nodes_<Input>() -> impl Parser<Input, Output = Vec<Box<Node>>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    todo!("you need to implement this combinator");
    (char(' ')).map(|_| vec![Element::new("".into(), AttrMap::new(), vec![])])
}

/// `text` consumes input until `<` comes.
fn text<Input>() -> impl Parser<Input, Output = Box<Node>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    todo!("you need to implement this combinator");
    (char(' ')).map(|_| Element::new("".into(), AttrMap::new(), vec![]))
}

/// `element` consumes `<tag_name attr_name="attr_value" ...>(children)</tag_name>`.
fn element<Input>() -> impl Parser<Input, Output = Box<Node>>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    todo!("you need to implement this combinator");
    (char(' ')).map(|_| Element::new("".into(), AttrMap::new(), vec![]))
}

parser! {
    fn nodes[Input]()(Input) -> Vec<Box<Node>>
    where [Input: Stream<Token = char>]
    {
        nodes_()
    }
}

pub fn parse(raw: &str) -> Box<Node> {
    let mut nodes = parse_raw(raw);
    if nodes.len() == 1 {
        nodes.pop().unwrap()
    } else {
        Element::new("html".to_string(), AttrMap::new(), nodes)
    }
}

pub fn parse_raw(raw: &str) -> Vec<Box<Node>> {
    let (nodes, _) = nodes().parse(raw).unwrap();
    nodes
}
#[cfg(test)]
mod tests {
    use crate::dom::Text;

    use super::*;

    // parsing tests of attributes
    #[test]
    fn test_parse_attribute() {
        assert_eq!(
            attribute().parse("test=\"foobar\""),
            Ok((("test".to_string(), "foobar".to_string()), ""))
        );

        assert_eq!(
            attribute().parse("test = \"foobar\""),
            Ok((("test".to_string(), "foobar".to_string()), ""))
        )
    }

    #[test]
    fn test_parse_attributes() {
        let mut expected_map = AttrMap::new();
        expected_map.insert("test".to_string(), "foobar".to_string());
        expected_map.insert("abc".to_string(), "def".to_string());
        assert_eq!(
            attributes().parse("test=\"foobar\" abc=\"def\""),
            Ok((expected_map, ""))
        );

        assert_eq!(attributes().parse(""), Ok((AttrMap::new(), "")))
    }

    #[test]
    fn test_parse_open_tag() {
        {
            assert_eq!(
                open_tag().parse("<p>aaaa"),
                Ok((("p".to_string(), AttrMap::new()), "aaaa"))
            );
        }
        {
            let mut attributes = AttrMap::new();
            attributes.insert("id".to_string(), "test".to_string());
            assert_eq!(
                open_tag().parse("<p id=\"test\">"),
                Ok((("p".to_string(), attributes), ""))
            )
        }

        {
            let result = open_tag().parse("<p id=\"test\" class=\"sample\">");
            let mut attributes = AttrMap::new();
            attributes.insert("id".to_string(), "test".to_string());
            attributes.insert("class".to_string(), "sample".to_string());
            assert_eq!(result, Ok((("p".to_string(), attributes), "")));
        }

        {
            assert!(open_tag().parse("<p id>").is_err());
        }
    }

    // parsing tests of close tags
    #[test]
    fn test_parse_close_tag() {
        let result = close_tag().parse("</p>");
        assert_eq!(result, Ok(("p".to_string(), "")))
    }

    #[test]
    fn test_parse_element() {
        assert_eq!(
            element().parse("<p></p>"),
            Ok((Element::new("p".to_string(), AttrMap::new(), vec![]), ""))
        );

        assert_eq!(
            element().parse("<p>hello world</p>"),
            Ok((
                Element::new(
                    "p".to_string(),
                    AttrMap::new(),
                    vec![Text::new("hello world".to_string())]
                ),
                ""
            ))
        );

        assert_eq!(
            element().parse("<div><p>hello world</p></div>"),
            Ok((
                Element::new(
                    "div".to_string(),
                    AttrMap::new(),
                    vec![Element::new(
                        "p".to_string(),
                        AttrMap::new(),
                        vec![Text::new("hello world".to_string())]
                    )],
                ),
                ""
            ))
        );

        assert!(element().parse("<p>hello world</div>").is_err());
    }

    #[test]
    fn test_parse_text() {
        {
            assert_eq!(
                text().parse("Hello World"),
                Ok((Text::new("Hello World".to_string()), ""))
            );
        }
        {
            assert_eq!(
                text().parse("Hello World<"),
                Ok((Text::new("Hello World".to_string()), "<"))
            );
        }
    }
}
