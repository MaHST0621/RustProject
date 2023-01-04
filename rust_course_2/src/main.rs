const MY_FAVORITE_NUM: i32 = 6;
fn main() {
    print!("my favorite number is {}\n", MY_FAVORITE_NUM);
    unassignalble();
    assignalble();
    unuseable_value();
    let_value();
    let_struct_value()
}

fn unassignalble() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn assignalble() {
    let mut x = 7;
    println!("The value of x is: {}", x);
    x = 8;
    println!("The value of x is: {}", x);
}

fn unuseable_value() {
    let _x = 1; //add underline prefix to disable warnings
    let y = 1;
}

fn let_value() {
    let (a, mut b): (bool, bool) = (true, false);
    print!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a,b);
}

struct MyStruct{
    e: i32
}

fn let_struct_value() {
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    [c, .., d, _] = [1, 2, 3, 4, 5, 6, 7];
    MyStruct{e, ..} = MyStruct{e:5};
    assert_eq!([a, b, c, d, e], [1, 2, 1, 6, 5]);
}
