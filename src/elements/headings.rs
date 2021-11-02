use crate::Element;
use std::collections::HashMap;

pub struct h1 {
    
}
pub struct h2 {
    
}
pub struct h3 {
    
}
pub struct h4 {
    
}
pub struct h5 {
    
}
pub struct h6 {
    
}
impl h1 {
    pub fn new () -> Element {
        Element {
            tag: String::from("h1"),
            value: String::from(""),
            options: HashMap::new(),
            styles: vec!()
        }
    }
}
impl h2 {
    pub fn new () -> Element {
        Element {
            tag: String::from("h2"),
            value: String::from(""),
            options: HashMap::new(),
            styles: vec!()
        }
    }
}
impl h3 {
    pub fn new () -> Element {
        Element {
            tag: String::from("h3"),
            value: String::from(""),
            options: HashMap::new(),
            styles: vec!()
        }
    }
}
impl h4 {
    pub fn new () -> Element {
        Element {
            tag: String::from("h4"),
            value: String::from(""),
            options: HashMap::new(),
            styles: vec!()
        }
    }
}
impl h5 {
    pub fn new () -> Element {
        Element {
            tag: String::from("h5"),
            value: String::from(""),
            options: HashMap::new(),
            styles: vec!()
        }
    }
}
impl h6 {
    pub fn new () -> Element {
        Element {
            tag: String::from("h6"),
            value: String::from(""),
            options: HashMap::new(),
            styles: vec!()
        }
    }
}
