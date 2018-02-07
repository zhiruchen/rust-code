#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct PointRef<'a> {
    x: &'a mut i32,
    y: &'a mut i32,
}

#[derive(Debug)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

struct Color(i32, i32, i32);

fn main() {
    let mut origin = Point { x: 0, y: 0 };
    println!("origin: {}, {}", origin.x, origin.y);

    {
        let p = PointRef { x: &mut origin.x, y: &mut origin.y };
        *p.x += 1;
        *p.y += 2;
    }

    assert_eq!(1, origin.x);
    assert_eq!(2, origin.y);
    println!("origin: {}, {}", origin.x, origin.y);

    // 更新struct
    let mut point3d = Point3D { x:0, y:0, z:0 };
    point3d = Point3D {y: 10, .. point3d};
    println!("point3d: {}, {}, {}", point3d.x, point3d.y, point3d.z);

    // struct tuple
    let color = Color(1,1,1);
    let black_r = color.0;
    let Color(_, c1, c2) = color;
    println!("black: {}", black_r);
    println!("c1: {}, c2: {}", c1, c2);

    struct ProgrammingLang<'a>(&'a str);
    let rust = ProgrammingLang("rust");
    let ProgrammingLang(which_pl) = rust;
    println!("rust: {}, which_pl: {}", rust.0, which_pl);

    // 创建一种的新的类型
    struct AStructUnit;
}
