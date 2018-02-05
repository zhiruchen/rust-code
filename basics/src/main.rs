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

}