extern crate html5ever;
extern crate tendril;

mod tables;

use html5ever::{ParseOpts, parse_document};
use html5ever::rcdom::{RcDom, Node, NodeData};
use tendril::TendrilSink;

//! Dissolve, the solution one uses to melt away webs.
//!
//! The basic usage of dissolve is to retrieve texts from HTML text nodes
//! via the strip_html_tags function.
//! ```rust
//! use dissolve::strip_html_tags;
//!
//! let input = "<html><body><div>Hello, this is the web!</div></body></html>";
//! let output = strip_html_tags(input);
//! assert_eq!(output, vec!["Hello, this is the web!".to_owned()]);
//! ```

/// Consumes a string that contains HTML tags and outputs a Vec<String>
/// containing the text content inside the tags in a pre-order manner.
///
/// Basic usage:
/// ```rust
/// use dissolve::strip_html_tags;
///
/// let input = "<html>Hello World!</html>";
/// let output = strip_html_tags(input);
/// assert_eq!(output, vec!["Hello World!".to_owned()]);
/// ```
pub fn strip_html_tags(input: &str) -> Vec<String> {
    let dom = parse_document(RcDom::default(), ParseOpts::default())
        .from_utf8()
        .one(input.as_bytes());
    let doc = dom.document;
    get_text(&doc)
}

/// Helper function to return text in text nodes in pre-order traversal.
fn get_text(element: &Node) -> Vec<String> {
    match element.data {
        NodeData::Text { ref contents } => {
            let mut text = vec!((&**contents.borrow()).to_owned());
            for child in &*element.children.borrow() {
                text.append(&mut get_text(child));
            }
            text
        }
        _ => {
            let mut text = vec!();
            for child in &*element.children.borrow() {
                text.append(&mut get_text(child));
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
