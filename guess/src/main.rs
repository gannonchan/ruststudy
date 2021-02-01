extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    let self_num = rand::thread_rng().gen_range(1,101);
   // println!("self ： {}", self_num);
    loop{
        let mut a = String::new();
        println!("请输入一个值！");
        io::stdin().read_line(&mut a).expect("需要输入一个数字才能开始额！");
        let a: u32 =match a.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入一个正确的数字。");
                continue;
            } 
        };
        match a.cmp(&self_num) {
            Ordering::Less => println!("你输入了一个小于它的值，继续努力吧！"),
            Ordering::Greater => println!("你输入了一个大于它的值，继续努力吧！"),
            Ordering::Equal => {
                println!("恭喜你！你答对了。");
                break;
            } 
        }
    }
    //println!("Hello, world!");
}
