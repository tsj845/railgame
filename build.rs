use std::path::Path;

extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=./c");
    let v = Path::new("./c").read_dir().unwrap().map(|n|(String::from("./c/")+&(n.unwrap().file_name().into_string().unwrap()))).filter(|n|n.ends_with(".c")).collect::<Vec<String>>();
    println!("cargo:warning={:?}", &v);
    cc::Build::new().files(v).compile("test");
    println!("cargo:warning=COMPILED");
    // cc::Build::new().file("c/test.c").compile("test");
}
