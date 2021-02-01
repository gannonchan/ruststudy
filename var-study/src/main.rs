use std::collections::HashMap;
use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;
fn main() {
    println!("打印标量类型默认值：");
    let a: u32;
    let b: i32;
    let c: f64;
    let d: bool;
    let e = ' ';
    a = 1;
    b = 1;
    c = 1.0;
    d = true;
    println!("u32: {}",a);
    println!("i32: {}",b);
    println!("f64: {}",c);
    println!("bool: {}",d);
    println!("char: {}",e);
    let mut hash_map = HashMap::new();
    let first_name = String::from("0");
    let last_name = String::from("张三");
  //  hash_map.insert(first_name,last_name);
    hash_map.insert(1,"李四");
    hash_map.insert(2,"gannon");
    hash_map.insert(3,"lily");
    let score = hash_map.get(&2);
    match score {
        Some(a) => println!("Result: {}",a),
        None => println!("空空空空空空空空空空")
    }
    //元组
    let tup:(i32,f64,bool) = (39,32.1,false);
    //使用模式匹配进行拆分元组
    //将元组拆分成三份分别将值赋予三个变量x.y.z
    let (x,y,z) = tup;
    println!("tup第1个值为: {}", x);
    println!("tup第2个值为: {}", y);
    println!("tup第3个值为: {}", z);
    //使用元组名.索引的方式将该索引的指向值赋予变量
    //元组的索引和数组一样默认从0开始
    let y1=tup.1;
    println!("tup第2个值为: {}", y1);
    //也可以直接通过元组名.索引的方式进行获得该索引所引用的值
    println!("tup第3个值为: {}", tup.2);
    let num = 3;
    match num {
        1 => println!("{}", num),
        2 => println!("{}", num),
        3 => println!("{}", num),
        _ => println!("{}", num),
    }
    //let mut str = "hello";
    //str = "world";//当前str为world；在这里是因为变量可变，所以相当于对变量重新赋值。
    let mut str = String::from("hello");
    str.push_str(", world!");//该方法是将字符串参数追加到字符串调用者的末尾。
    println!("{}", str);
    let str_tmp = str;//这里只是将str字符串指向堆上数据的指针赋值给了str_tmp变量。
    println!("str_tmp: {}", str_tmp);
    let str1 = String::from("你好！");
    let str_mov = str1;
    //println!("str1: {}", str1);//编译时会出现一个错误，因为已经将str1变量绑定的指向堆上数据的指针指向了str_mov变量
    //                                  ，该str1变量已经没有绑定任何值和堆上数据的指针所以是一个无效变量。
    println!("str_mov: {}", str_mov);
    let str2 = String::from("吃饭了吗？");
    takes_ownership(str2);
   // println!("{}",str2); //编译时会出现一个错误，因为String类型并未实现copy trait当调用函数时会发生值移动。
    let v = 5;
    makes_copy(v);//值拷贝操作，因为类似i32存储在栈上这样的基本类型实现了copy trait
    let f = File::open("d:\\hello.txt");//打开文件
    let f = match f {   //使用match判断打开文件是否出现错误
        Ok(file) => file,   //打开成功返回一个文件句柄并赋值给f
        //调用error.kind（）方法获取错误类型，如果错误类型为ErrorKind枚举中
        //的文件不存在则创建文件;注意之所以使用ref是因为在此我们只需要将引用
        //使用到if中而不是获取所有权(& 匹配一个引用并返回它的值，而
        //ref 匹配一个值并返回一个引用)
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            //因为创建文件也可能出现错误所以应该使用match包裹判断
            match File::create("d:\\hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file but there was a problem: {:?}",e)
                },
            }
        },
        //其他错误情况程序panic，上一次if条件中并未获得error所有权所以还可以对其使用
        Err(error) => panic!("{:?}",error)
    }; 
    let path = "d:\\hello.txt";
    let s = read_username_from_file(&path);
    println!("{:?}",s);
    //使用Result类型的unwrap方法可以简化成功则返回，错误则panic操作。
    let f = File::open("d:\\hello.txt").unwrap();
    //也可以使用expect()方法简化而且panic后可以自定义错误消息。
    let f = File::open("d:\\hello.txt").expect("打开文件发生错误！");
    let vec = vec![1,2,3,4,5,6,7,8,9];
    let max_value = select_max_val(&vec);
    println!("{}",max_value);
    for i in &vec {
        print!("{} ",i);
    }

}
fn select_max_val(vec :&[i32]) -> i32{
    let mut largest :i32 = vec[0];
    for &item in vec.iter() {
        if item>largest{
            largest = item;
        }
    }
    largest
}

fn read_username_from_file(path :&str) -> Result<String, io::Error> {
    let f = File::open(path);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn takes_ownership(some_string: String) { // some_string comes into scope.
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.
fn makes_copy(some_integer: i32) { // some_integer comes into scope.
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.