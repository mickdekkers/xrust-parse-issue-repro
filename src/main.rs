use std::{fs, rc::Rc};
use xrust::parser::xml;
use xrust::trees::smite::Node as SmiteNode;

fn main() {
    let data = fs::read_to_string("data/content.xml").unwrap();
    let source = Rc::new(SmiteNode::new());

    xml::parse(source.clone(), &data, None).unwrap();
}
