# Dissolve
Melt away HTML tags and retrieve the text contents that you care about!

## Basic usage
The basic functionality of dissolve is to remove HTML tags, leaving behind the texts inside of them:
```rust
use dissolve::strip_html_tags;

let input = "<html>Hello World!</html>";
let output = strip_html_tags(input);
assert_eq!(output, vec!["Hello World!".to_owned()]);
```

## Improvements
* Allow selection of in-order and post-order traversals
* Consider reporting errors during parsing
* Consider an option to retain HTML attributes, together or without text nodes
