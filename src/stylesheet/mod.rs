pub struct Style {
    name: String,
    val: String
}

impl Style {
    pub fn new() -> Style {
        Style {
            name : String::new(),
            val : String::new()

        }
    }
    pub fn from_args(name : String, val : String) -> Style{
        Style {
            name,
            val
        }
    }
    pub fn get_name (&self) -> String{
        self.name.clone()
    }
    pub fn get_val (&self) -> String {
        self.val.clone()
    }
}

pub struct Stylesheet {
    styles : Vec<Style>
}
impl Stylesheet {
    pub fn new() -> Stylesheet {
        Stylesheet {
            styles : vec!()
        }
    }
    pub fn add_style(&mut self, name: &str, value: &str){
        self.styles.push(Style {
            name: String::from(name),
            val: String::from(value)
        });
    }
}