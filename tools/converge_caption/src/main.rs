use std::mem::swap;
use clap::Parser;

/// Converge line by line and output a csv file
#[derive(Parser,Debug)]
#[command(version, about, long_about=None)]
struct Args {
    /// File of the input
    #[arg(short,long)]
    first: String,

    /// File of the second input
    #[arg(short,long)]
    second: String
}

fn main() {
    // Parse is needed for --help
    let args = Args::parse();
    let zh = args.first;
    let en = args.second;
    let err_zh = format!("No file named {zh}");
    let err_en = format!("No file named {en}");
    let mut zh = std::fs::read_to_string(zh).expect(&err_zh);
    let mut en = std::fs::read_to_string(en).expect(&err_en);
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

