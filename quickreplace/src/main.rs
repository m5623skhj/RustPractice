use text_colorizer::*;
use std::env;

// [derive(Debug)] -> Arguments를 println!에서 {:?}로 형식화 할 수 있도록 컴파일러가 코드 추가 생성
#[derive(Debug)]
struct Arguments
{
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn PrintUsage()
{
    eprintln!("{} - change occurrences of one string into another", "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

fn main() 
{
    let args = ParseArgs();
    println!("{:?}", args);
}

fn ParseArgs() -> Arguments
{
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4
    {
        PrintUsage();
        eprintln!("{} wrong number of arguments: expected 4, got {}.", "Error:".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments
    {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}