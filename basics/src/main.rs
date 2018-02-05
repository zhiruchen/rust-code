fn main() {
    let x = true;
    let y: bool = false;

    println!("bool value x: {}, y:{}", x, y);

    let my_char = 'x';
    let your_char = '😂';
    println!("mychar: {}, your char: {}", my_char, your_char);


    let num = 42;
    let fnum = 1.0;
    println!("num: {}, fnum: {}", num, fnum);

    let a = [1,2,3];
    let m = [1,2,3];
    println!("a: {:?}, m: {:?}", a, m);
    println!("a.len: {}, m.len: {}", a.len(), m.len());


    let a = [0; 20]; // [i32; 20]
    
    let names = ["a", "b", "morning"];
    println!("{}", names[2]);

    // 切片
    let complete = &names[..];
    println!("{},{},{}", complete[0],complete[1],complete[2]);
    let middle = &a[6..13];
    println!("{}", middle[0]);
    
    // 元组
    let x = (1, "hello");
    println!("tuple: {:?}", x);

    let mut z = (100000, "ha");
    z = x;  // 相同类型的元组可以赋值
    println!("{:?}", z);

    let (number, string) = z;
    println!("number is: {}, string is: {}", number, string);

    // 元组可以按照索引访问 
    println!("tuple 0: {}", z.0);
    println!("tuple 1: {}", z.1);

    // 函数类型
 
    // if
    let anum = 5;
    if anum == 5 {
        println!("{}", anum)
    } else if anum == 6 {
        println!("anum is six!");
    } else {
        println!("{}", anum+1111)
    }

    let bnum = if anum == 5 {
        10
    } else {
        15
    };
    println!("bnum: {}", bnum);

    let cnum = if anum == 5 { 10 } else { 15 };
    println!("cnun: {}", cnum);

    loop {
        println!("loop!");
        break;
    }

    let mut done = false;
    while !done {
        println!("while loop");
        done = true
    }

    //for loop
    for i in 1..10 {
        println!("i: {}", i) // 1-9
    }

    for (index, value) in (5..10).enumerate() {
        println!("index = {}, value = {}", index, value);
    }

    for (linenum, line) in "hello\nworld".lines().enumerate() {
        println!("{}: {}", linenum, line);
    }

    let mut val = 5;
    loop {
        val += val - 3;
        println!("{}", val);

        if val % 5 == 0 { break; }
    }

    // loop label
    'outer: for v in 0..10 {
        'inner: for vv in 0..10 {
            if v % 2 == 0 { continue 'outer; }
            if vv % 2 == 0 { continue 'inner; }
            println!("v: {}, vv: {}", v, vv);
        }
    }

    // vector 动态可增长的数组
    let vec = vec![1,2,3,4,5];
    println!("vec: {:?}", vec);    
    println!("第3个元素值: {}", vec[2]);

    // 可以多次遍历vec的引用
    for i in &vec {
        println!("take ownership of the vector and its element {}", i);
    }
    for i in &vec {
        println!("take ownership of the vector and its element {}", i);
    }

    // 处理越界
    match vec.get(5) {
        Some(x) => println!("第7个元素: {}", x),
        None => println!("访问越界!")
    }

    let mut vec1 = vec![0; 10];
    println!("vec1: {:?}", vec1);

    // 遍历vec
    let mut vec2 = vec![1,1,1,1,4,5,5,6,7];
    for i in &vec {
        println!("a reference to {}", i);
    }

    for i in &mut vec2 {
        println!("a mutable reference to {}", i);
    }

    for i in vec2 {
        println!("take ownership of the vector and its element {}", i);
    }

    // 所有权

    let myv = vec![0,1,2];
    let myv1 = myv;
    // println!("myv is : {:?}", myv); // move occurs because `myv` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
    // 基本类型实现了copy trait
    let val1 = 1;
    let val2 = val1;
    println!("val1 is: {}", val1);

    //借用所有权
    fn sum_vec(v: &Vec<i32>) -> i32 {
        return v.iter().fold(0, |a, &b| a+b);
    }

    // 通过引用借用所有权
    fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
        let s1 = sum_vec(v1);
        let s2 = sum_vec(v2);

        s1 + s2
    }

    let vec_v1 = vec![1,2,23];
    let vec_v2 = vec![100,123,0];
    let answer = foo(&vec_v1, &vec_v2);
    println!("answer: {}", answer);

    // 可变引用
    let mut val_x = 5;
    {
        let val_y = &mut val_x; // 可变引用，允许改变借用的资源
        *val_y += 1;
    }

    println!("{}", val_x);
}