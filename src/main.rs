use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 && args[1] == "contract-info" {
        contract_info(&args[2]);
    } else if args[1] == "header" {
        header(&args[2..]);
    }
}

fn contract_info(input: &str) {
    let arguments: Vec<&str> = input.split(',').collect();
    if arguments.len() != 4 {
        println!("Invalid number of arguments");
        return;
    }

    let title = arguments[0].trim();
    let author = arguments[1].trim();
    let notice = arguments[2].trim();
    let dev = arguments[3].trim();

    println!(
        "///////////////////////////\n/**\n* @title {}\n* @author {}\n* @notice {}\n* @dev {}\n*\n*/\n///////////////////////////",
        title, author, notice, dev
    );
}

fn header(args: &[String]) {
    if args.is_empty() {
        println!("Invalid number of arguments");
        return;
    }
    let head = args.join(" ");
    println!(
        "/*//////////////////////////////////////////////////////////////
                        {}
//////////////////////////////////////////////////////////////*/",
        head.to_uppercase()
    );
}