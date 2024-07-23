
use std::io;
/**
 猜数字: 用户控制台输入数字，与程序生成的伪随机数判断，根据提示的范围继续猜，直到猜中（二叉树搜索）
 知识点：io
*/
pub fn guess_number() {
     println!("Guess the number!");
     println!("Please input your guess.");
     let mut guess = String::new();
     io::stdin().read_line(&mut guess)
         .expect("Failed to read line");
     println!("You guessed: {}", &guess);

}

