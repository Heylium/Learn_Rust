use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101); //0.8版本改为[a..b)形式，不再用[a,b)
    // println!("随机数字为{}",secret_number);

    loop {
        println!("输入一个数字:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("错误的输入。");
        let guess: u32 = guess.trim().parse()
            .expect("错误的输入。");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
