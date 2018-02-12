use std::collections::HashMap;

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

    // 字符串
    op_string();

    // hash map
    hash_map();
}

fn op_string() {
    let mut s = String::new();
    println!("s.len: {}", s.len());

    let data = "init string";
    let s1 = data.to_string();
    println!("s1: {}", s1);

    println!("string from to_string: {}", "init content".to_string());

    println!("from string: {}", String::from(s1));

    // 增长字符串
    let mut s2 = String::from("from string");
    s2.push_str("spring");
    s2.push_str("festival");
    println!("{}", s2);

    let mut s3 = String::from("lo");
    s3.push('l');
    println!("s3: {}", s3);

    let str_data1 = "hello ".to_string();
    let str_data2 = "world!".to_string();
    let str_data3 = str_data1 + &str_data2;
    println!("{}", str_data3);

    // 连接字符串
    let str_data4 = format!("{}-{}-{}", "tic".to_string(), "tao".to_string(), "toc".to_string());
    println!("{}", str_data4);
}

fn hash_map() {
    let mut scores = HashMap::new();

    let team_name1 = String::from("Blue");
    let team_name2 = String::from("Yellow");
    scores.insert(team_name1, 10);
    scores.insert(team_name2, 15);

    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    let bule_key = "Blue".to_string();

    let score = scores.get(&bule_key);
    println!("score: {:?}", score);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let init_scores = vec![10, 50];

    let scores2: HashMap<_,_> = teams.iter().zip(init_scores.iter()).collect();

    println!("{:?}", scores2.get(&bule_key));
    let text = "hello world wonderful world!";
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", word_count);
}