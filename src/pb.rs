use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::Result;
use std::io::{ BufRead, BufReader };
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

#[cfg(windows)]
const PBJS: &'static str = "pbjs.cmd";

#[cfg(not(windows))]
const PBJS: &'static str = "pbjs";

pub fn pbjs(
    output: &str,
    input: &str,
) -> Result<()> {
    fs::create_dir_all(output)?;
    let mut vec = Vec::new();
    for entry in fs::read_dir(input)? {
        let entry = entry?;
        let path = &entry.path();
        if let Some(stem) = &path.file_stem() {
            println!("{:?}", stem);
        }
        vec.push(path.clone());
    }
    Command::new(PBJS)
        .arg("-t")
        .arg("static-module")
        .arg("-w")
        .arg("commonjs")
        .arg("--es6")
        .arg("-o")
        .arg(format!("{}proto.js", output))
        .args(vec)
        .output()?;
    Ok(())
}

pub fn protoc(
    lang: &str,
    output: &str,
    input: &str,
) -> Result<()> {
    fs::create_dir_all(output)?;
    for entry in fs::read_dir(input)? {
        let mut vec = Vec::new();
        let entry = entry?;
        let path = entry.path();
        if let Some(stem) = &path.file_stem() {
            println!("{:?}", stem);
        }
        vec.push(path.clone());
        for path in get_import_filepaths(input, &path, r"[A-Z][A-Za-z]+\.proto") {
            vec.push(path.clone());
        }
        Command::new("protoc")
            .arg(format!("--{}_out={}", lang, output))
            .arg(format!("--proto_path={}", input))
            .args(vec)
            .output()?;
    }
    Ok(())
}

fn get_import_filepaths(
    input: &str,
    path: &PathBuf,
    regex: &str,
) -> Vec<PathBuf> {
    let mut vec = Vec::new();
    for filename in captures(&path, regex) {
        let path = format!("{}{}", input, filename);
        let path = Path::new(&path);
        let path = path.to_path_buf();
        vec.push(path.clone());
        let filepaths = get_import_filepaths(input, &path.clone(), regex);
        if filepaths.len() > 0 {
            for path in filepaths {
                vec.push(path);
            }
        }
    }
    vec
}

fn captures(
    path: &PathBuf,
    regex: &str,
) -> Vec<String> {
    let mut vec = Vec::new();
    let regex = Regex::new(regex).unwrap();
    let file = File::open(path).expect("failed to open file.");
    for line in BufReader::new(file).lines() {
        if let Ok(l) = line {
            if let Some(caps) = regex.captures(&l[..]) {
                let cap = &caps[0];
                let target = cap.to_string();
                vec.push(target.clone());
            }
        }
    }
    vec
}