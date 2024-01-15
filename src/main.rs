use std::{fs, io};
use std::fs::create_dir_all;
use std::path::PathBuf;

use structopt::StructOpt;

use crate::core::format_typescript::format_typescript;
use crate::core::parse_protobuf::parse_protobuf;

pub mod core;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short = "i", parse(from_os_str))]
    input: PathBuf,

    #[structopt(short = "o", parse(from_os_str))]
    output: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    // 检查 input 路径是否存在
    if !opt.input.exists() {
        panic!("invalid input path {:?}", opt.input)
    }

    // 判断 input 是目录还是文件
    if opt.input.is_file() {
        // 文件 - 判断是不是 proto 结尾的
        if opt.input.extension().map_or(false, |ext| ext == "proto") {
            read_proto(&opt.input, &opt.output);
        } else {
            panic!("the input file isn't recognized as a protobuf file.")
        }
    } else if !opt.input.read_dir().unwrap().next().is_none() {
        read_dir(&opt.input, &opt.output);
    } else {
        panic!("invalid or empty dir")
    }
}

fn read_proto(input: &PathBuf, output: &PathBuf) {
    let proto_buf_data = parse_protobuf::parse_proto(input).unwrap();
    let ts_str = format_typescript::fmt_ts_str(proto_buf_data).unwrap();

    let file_name = PathBuf::from(input.file_name().unwrap().to_os_string()).with_extension("ts");

    export_ts_file(output, file_name, ts_str).expect(&format!("read proto {:?} is error", input))
}

fn read_dir(input: &PathBuf, output: &PathBuf) {
    for entry in fs::read_dir(input).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            read_dir(&path, output);
        } else if path.extension().map_or(false, |ext| ext == "proto") {
            read_proto(&path, output);
        }
    }
}

fn export_ts_file(output: &PathBuf, file_name: PathBuf, content: String) -> io::Result<()> {
    let mut file_path = output.clone();
    file_path.push(file_name);
    if let Some(parent) = file_path.parent() {
        create_dir_all(parent)?;
    }
    fs::write(file_path, content)
}