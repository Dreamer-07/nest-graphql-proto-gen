use std::io;
use std::path::PathBuf;

use structopt::StructOpt;

pub mod core;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short = "i", parse(from_os_str))]
    input: PathBuf,

    #[structopt(short = "o", parse(from_os_str))]
    output: PathBuf,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    // 检查 input 路径是否存在
    if !opt.input.exists() {
        panic!("invalid input path {:?}", opt.input)
    }

    // 判断 input 是目录还是文件
    if opt.input.is_file() {
        // 文件 - 判断是不是 proto 结尾的
        if opt.input.extension().and_then(std::ffi::OsStr::to_str) == Some("proto") {
            let a = core::parse_protobuf::parse_protobuf::parse_proto(&opt.input)?;
        } else {
            panic!("the input file isn't recognized as a protobuf file.")
        }
    } else if !opt.input.read_dir()?.next().is_none() {} else {
        panic!("invalid or empty dir")
    }

    Ok(())
}