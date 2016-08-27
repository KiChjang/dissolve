extern crate html5ever;
extern crate tendril;

use html5ever::{ParseOpts, parse_document};
use html5ever::rcdom::{RcDom, NodeEnum, Node};
use tendril::TendrilSink;

/// Consumes a string that contains HTML5 tags and spits out a Vec<String>
/// containing the text content inside the tags in a pre-order manner.
///
/// Usage:
/// ```
/// let input = "<html>Hello World!</html>".to_owned();
/// let output = strip_html_tags(input);
/// assert_eq!(output, "Hello World!".to_owned()");
/// ```
pub fn strip_html_tags(input: &str) -> Vec<String> {
    let dom = parse_document(RcDom::default(), ParseOpts::default())
        .from_utf8()
        .one(input.as_bytes());
    let ref doc = *dom.document.borrow();
    get_text(doc)
}

/// Helper function to return text in text nodes in pre-order traversal.
fn get_text(element: &Node) -> Vec<String> {
    match element.node {
        NodeEnum::Text(ref s) => {
            let mut text = vec!((&**s).to_owned());
            for child in &element.children {
                text.append(&mut get_text(&*child.borrow()));
            }
            text
        }
        _ => {
            let mut text = vec!();
            for child in &element.children {
                text.append(&mut get_text(&*child.borrow()));
            }
            text
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_strip_html_tag() {
        let input = "<html>Hello World!</html>";
        let output = strip_html_tags(input);
        assert_eq!(output, vec!["Hello World!".to_owned()]);
    }

    #[test]
    fn test_strip_nested_tags() {
        let input = "<html>Hello<div>World!</div></html>";
        let output = strip_html_tags(input);
        assert_eq!(output, vec!["Hello".to_owned(), "World!".to_owned()]);
    }

    #[test]
    fn test_preorder_traversal() {
        let input = "<html>Hel<div>lo</div>World!</html>";
        let output = strip_html_tags(input);
        assert_eq!(output, vec!["Hel".to_owned(), "lo".to_owned(), "World!".to_owned()]);
    }
}
