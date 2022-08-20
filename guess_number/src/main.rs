use std::{io, cmp::Ordering};
use rand::Rng;
fn main() {
    println!("欢迎来到猜字游戏!");
    let secreat_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("请输入您的数字");
        
        let mut customer_number = String::new();
        io::stdin().read_line(&mut customer_number).expect("Failed to input customer number");
        let customer_number:u32 = match customer_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match customer_number.cmp(&secreat_number) {
            Ordering::Equal => {
                println!("恭喜您猜对了,答案是{secreat_number}");
                break;
            },
            Ordering::Greater => println!("更小"),
            Ordering::Less => println!("更大"),
        }
    }
}
