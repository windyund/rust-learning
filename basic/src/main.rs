
/**
  本次每个demo文件都写了单元测试，但需要将对应的mod引入到main 入口函数中才能执行测试方法
*/

mod demo1_guess_number;
mod demo2_http_invoke;
mod demo3_feibo;

fn main() {
    demo1_guess_number::guess_number();
    println!("hello world!")
}
