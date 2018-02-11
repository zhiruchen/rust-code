fn main() {
    let mut v = Vec::new();

    v.push(2);
    v.push(6);
    v.push(3);
    v.push(1);
    v.push(7);

    {
        let first = &v[0];
    }
    v.push(10);

    {
        let third = v[2];
        let third_option = v.get(2);
        println!("third: {}, third_option: {:?}", third, third_option);
    }
    
    for e in &v {
        println!("e: {}", e);
    }

    for e in &mut v {
        *e += 5;
    }

    for e in &v {
        println!("e: {}", e);
    }

    {
        let vv = vec![1,2,3];
    }  // 丢弃vv
    println!("Hello, world!");

    enum_vec();
}

fn enum_vec(){
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Flaot(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Flaot(12.12),
        SpreadsheetCell::Text(String::from("enum vec")),
    ];

    println!("{:?}", row);
}