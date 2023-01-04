fn main() {
    let a: i32 = 100;

    let aa = 100i32;

    let aaa = 100_i32;


    print!("sum of a & aa & aaa = {}",add(a, aa, aaa));
}

fn add(i: i32, j: i32, k: i32) ->i32{
    i + j + k
}