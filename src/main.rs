use std::env;

mod pb;

fn main() {
    let args: Vec<String> = env::args().collect();

    let output_java = "./output/java";
    let output_js = "./output/js";

    let input = "./input/";

    for a in args {
        match &a[..] {
            "java" => {
                println!("proto -> java");
                match pb::protoc("java", output_java, input) {
                    Ok(_) => println!("success.\n"),
                    Err(err) => panic!("{:?}", err),
                }
            },
            "js" => {
                println!("proto -> js");
                match pb::pbjs(output_js, input) {
                    Ok(_) => println!("success.\n"),
                    Err(err) => panic!("{:?}", err),
                }
            }
            _ => {
                // Do nothing.
            },
        }
    }
}