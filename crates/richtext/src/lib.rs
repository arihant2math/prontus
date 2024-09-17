use comrak::nodes::{AstNode, NodeValue};
use comrak::{parse_document, Arena, Options};
use std::io;

pub fn format_ipc<'a>(
    root: &'a AstNode<'a>,
    options: &Options,
    output: &mut serde_json::Value,
) -> io::Result<()> {
    let mut children = vec![];
    for node in root.children() {
        let mut object_children = vec![];
        node.children().for_each(|child| {
            let mut value = serde_json::Value::Null;
            format_ipc(child, options, &mut value).unwrap();
            object_children.push(value);
        });
        let data = node.data.borrow().value.clone();
        let mut object = serde_json::Map::new();
        object.insert("data".to_string(), serde_json::to_value(&data)?);
        object.insert(
            "children".to_string(),
            serde_json::Value::Array(object_children.clone()),
        );
        children.push(serde_json::Value::Object(object));
    }
    let mut prep = serde_json::Map::new();
    prep.insert(
        "data".to_string(),
        serde_json::to_value(&root.data.borrow().value)?,
    );
    prep.insert("children".to_string(), serde_json::Value::Array(children));
    *output = serde_json::Value::Object(prep);
    Ok(())
}

pub fn parse(markdown: &str) -> serde_json::Value {
    let arena = Arena::new();

    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.autolink = true;
    options.extension.math_code = true;
    options.extension.table = true;
    options.extension.math_dollars = true;
    options.extension.footnotes = false;
    options.extension.tasklist = true;
    options.extension.superscript = true;
    options.extension.underline = true;

    let root = parse_document(&arena, markdown, &options);

    // for node in root.descendants() {
    //     if let NodeValue::Text(ref mut text) = node.data.borrow_mut().value {
    //         let s = text.to_string();
    //         let re = regex::Regex::new(r"<@\d*>").unwrap();
    //         *text = text.replace("", "");
    //     }
    // }

    let mut value = serde_json::Value::Null;
    format_ipc(root, &Options::default(), &mut value).unwrap();
    value
}
