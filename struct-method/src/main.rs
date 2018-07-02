#[derive(Debug)]
struct Programmer {
    programming_lang: String,
    name: String
}

trait NameGetter {
    fn get_name(&self) -> String;
}

impl NameGetter for Programmer {
    fn get_name(&self) -> String {
        format!("{}", self.name)
    }
}

impl Programmer {
	// new Associated Function
	fn new(lang: String, name: String) -> Programmer {
	    Programmer {
	    	programming_lang:lang,
	    	name: name
	    }
	}
    fn print(&self) -> String {
    	format!("A {} programmer", self.programming_lang)
    }
}


fn main() {
	let prorgrammer = Programmer{programming_lang:String::from("Rust"), name:String::from("bob")};
	
    println!("{}", prorgrammer.print());
    println!("{}", prorgrammer.get_name());

    let prorgrammer2 = Programmer::new(String::from("Golang"), String::from("Tony"));
    println!("{}", prorgrammer2.print());
    println!("{}", prorgrammer2.get_name());
}
