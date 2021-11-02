use Oxide::elements;
use elements::headings::*;
use chrono;
use Oxide::stylesheet::Style;

fn main () {
    rebuild();
}

fn rebuild() {
    let mut page1 = Oxide::Webpage::new();
    page1.set_title("A page");
    let mut h1 = h1::new();
    let mut h2 = h2::new();
    let mut h3 = h3::new();
    let mut h4 = h4::new();
    let mut h5 = h5::new();
    let mut h6 = h6::new();
    let date : String = format!("Site created on {}", chrono::Utc::now());
    h1.set_value(&date);
    let sty = Style::from_args(String::from("color"), String::from("red"));
    h1.add_style(sty);
    h2.set_value(&"Heres a heading");
    h3.set_value(&"Heres a heading");
    h4.set_value(&"Heres a heading");
    h5.set_value(&"Heres a heading");
    h6.set_value(&"Heres a heading");
    page1.add_all(vec![h1, h2, h3, h4, h5, h6]);
    page1.build(String::from("./none.html"));
}