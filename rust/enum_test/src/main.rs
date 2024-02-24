


struct Point {
    x: i32,
    y: i32,
}


fn print_coordinates(&(x,y): &(i32,i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {

    // while let
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for循环模式匹配 解构元组
    let v = vec!['a', 'c', 'd'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at {}", value, index);
    }


    // 函数参数的模式匹配
    let point = (3,5);
    print_coordinates(&point);


    let point_struct = Point{x: 0, y: 19};
    let Point{x: xval, y: yval} = point_struct;
    println!("({}, {})", xval, yval);

    match point_struct {
        Point{x:0, y} => println!("Point {} is on the xasix", y),
        _ => println!("Not fun"),
    }

}
