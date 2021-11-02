use std::fs::File;
use std::io::Write;
pub mod elements;
pub mod stylesheet;
use elements::Element;
use stylesheet::Stylesheet;
pub struct Webpage {
    title : String,
    elements : Vec<Element>
}
impl Webpage {
    pub fn new () -> Webpage{
        Webpage {
            title: String::new(),
            elements: vec!()
        }
    }

    pub fn set_title(&mut self, title :&str) {
        self.title = String::from(title);
    }

    pub fn add( &mut self,  elem : Element ) {
        self.elements.push(elem);
    }

    pub fn add_all( &mut self, element_arr : Vec<Element>){
        for elem in element_arr {
            self.elements.push(elem);
        }
    }

    pub fn build(&self, destination : String) {
        let file = File::create(destination);
        let mut file = match file {
            Ok(f) => f, 
            Err(err) => panic!(err)
        };
        let template = format!("<!DOCTYPE html>
<html>
<head>
<title>{}</title>
</head>
<body>
", self.title);
        file.write((&template).as_bytes()).unwrap();

        for elem in self.elements.iter() {
            file.write((elem.get_html() + "\n").as_bytes()).unwrap();
        }

        file.write("
</body>
</html>".as_bytes()).unwrap();
    }
}