use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数游戏！");
    let secret_num = rand::thread_rng().gen_range(1..101);
    //println!("神秘数字是: {}", secret_num);

    loop {
        println!("猜测一个数字");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取");

        // shadow
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜测的数是: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("too small"), //arm
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            },
        }
    }
}
