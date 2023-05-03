use html5ever::tendril::TendrilSink;
use html5ever::{parse_document, ParseOpts};
use markup5ever_rcdom::{Node, NodeData, RcDom};

/// Consumes a string that contains HTML5 tags and outputs a `String` containing the text content
/// inside the tags.
///
/// Basic usage:
///
/// ```rust
/// # use dissolve::strip_html_tags;
/// let input = "<html>Hello World!</html>";
/// let output = strip_html_tags(input);
/// assert_eq!(output, "Hello World!");
/// ```
pub fn strip_html_tags(input: &str) -> String {
    let dom = parse_document(RcDom::default(), ParseOpts::default()).one(input);
    let doc = dom.document;
    let mut texts = String::new();
    push_texts(&doc, &mut texts);
    texts
}

/// Helper function to return text in text nodes in pre-order traversal.
fn push_texts(element: &Node, texts: &mut String) {
    if let NodeData::Text { contents } = &element.data {
        texts.push_str(&contents.borrow());
    }
    for child in &*element.children.borrow() {
        push_texts(child, texts);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_strip_html_tag() {
        let input = "<html>Hello World!</html>";
        let output = strip_html_tags(input);
        assert_eq!(output, "Hello World!");
    }

    #[test]
    fn test_strip_nested_tags() {
        let input = "<html>Hello<div>World!</div></html>";
        let output = strip_html_tags(input);
        assert_eq!(output, "HelloWorld!");
    }

    #[test]
    fn test_preorder_traversal() {
        let input = "<html>Hel<div>lo</div>World!</html>";
        let output = strip_html_tags(input);
        assert_eq!(output, "HelloWorld!");
    }
}
