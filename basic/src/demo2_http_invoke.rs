
#[cfg(test)]
mod tests{
    use std::fs;

    #[test]
    fn  test() {
        let url = "https://www.rust-lang.org/";
        let output = "rust.md";
        println!("Fetching url: {}", url);
        // step1 请求网址内容
        let body = reqwest::blocking::get(url).unwrap().text().unwrap();

        println!("Converting html to markdown....");
        // step2 将网页内容转为markdown格式
        let md = html2md::parse_html(&body);
        // step3 输出到文件
        fs::write(output,md.as_bytes()).unwrap();
        println!("Converting markdown has been saved in {}",output);
    }


}