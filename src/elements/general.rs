use crate::Element;
use std::collections::HashMap;

pub struct div {

}
pub struct p {

}
impl div {
    pub fn new () -> Element {
        Element {
            tag: String::from("div"),
            value: String::from(""),
            options: HashMap::new(),
            styles: vec!()
        }
    }
}
impl p {
    pub fn new () -> Element {
        Element {
            tag: String::from("p"),
            value: String::from(""),
            options: HashMap::new(),
            styles: vec!()
        }
    }
}