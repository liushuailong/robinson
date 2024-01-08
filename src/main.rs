use std::collections::HashMap;

use robinson::dom::{
    text,
    elem,
};
fn main() {
    let test_node = text("I am robinson.".to_string());
    println!("{:?}", &test_node);
    let mut attrs = HashMap::new();
    attrs.insert("id".to_string(), "1".to_string());
    let elem_node = elem("html".to_string(), attrs, vec![test_node,]);
    println!("{:?}", &elem_node);
    println!("Hello, world!");
}
