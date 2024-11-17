use std::mem::swap;

fn main() {
    //skip the file name
    let mut i = std::env::args().skip(1);
    let zh = i.next().unwrap();
    let en = i.next().unwrap();
    let mut zh = std::fs::read_to_string(zh).unwrap();
    let mut en = std::fs::read_to_string(en).unwrap();
    if !is_chinese(&zh){swap(&mut zh,&mut en);}
    let separator = "\t";
    let mut res = format!("zh{separator}en\n");
    zh.lines().zip(en.lines()).map(|(z,e)|{
        let temp = format!("{}{separator}{}\n",z.trim(),e.trim());
        res.extend(temp.chars());
    }).last();
    println!("{res}");
    let _ = std::fs::write("zhen.txt", res).unwrap();
}

fn is_chinese(text: &str) -> bool {
    let chinese_count = text.chars().filter(|&c| c >= '\u{4E00}' && c <= '\u{9FFF}').count();
    let english_count = text.chars().filter(|&c| c.is_ascii_alphabetic()).count();
    chinese_count > english_count
}

