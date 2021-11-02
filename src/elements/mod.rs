pub mod headings;
use std::collections::HashMap;
use crate::stylesheet::Style;

pub struct Element {
    tag: String,
    value: String,
    options: HashMap<String, String>,
    styles: Vec<Style>
}

impl Element {
    pub fn new() -> Element {
        Element {
            tag: String::new(),
            value: String::new(),
            options: HashMap::new(),
            styles: vec!()
        }
    }
    pub fn get_html(&self) -> String {
        let mut style_str = String::new();
        for style in self.styles.iter() {
            style_str.push_str(&format!("{}: {}; ", style.get_name(), style.get_val()));
        }
        format!(r#"<{} style="{}">{}</{}>"#, self.tag, style_str, self.value, self.tag)
    }
    pub fn set_value(&mut self, val : &str) {
        self.value = String::from(val);
    }
    pub fn add_style(&mut self, style : Style) {
        self.styles.push(style);
    }
}
