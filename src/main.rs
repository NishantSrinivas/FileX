mod file_rdr;
use clap::Parser;

#[derive(Parser)]
struct Args {
    operation: String,
    operand1: String,
    operand2: String
}

fn perform_operation(op: filex::Operation) {
    match op.ops {
        filex::Ops::Copy(src, dst) => {
            println!("Copy");
        },
        filex::Ops::Count(word, file_path ) => {
            println!("Count");
            let res = file_rdr::count_occurrence(word, file_path);
            match res {
                Some(count) => println!("Occurrences = {}",count),
                None => println!("Error while counting")
            };
        },
        filex::Ops::Search(word, file_path) => {
            println!("Search");
        },
        filex::Ops::NoOps => {
            println!("Invalid operation");
        }
    }
}

fn main() {
    let mut args = Args::parse();
    args.operation = args.operation.trim().to_string().to_lowercase();
    args.operand1 = args.operand1.trim().to_string();
    args.operand2 = args.operand2.trim().to_string();

    // copy arm matches to any operation regardless of
    // actual string value
    // let ops = match args.operation {
    //     copy => Operation{ops: Ops::Copy(args.operand1, args.operand2)},
    //     count => Operation{ops: Ops::Count(args.operand1, args.operand2)},
    //     search => Operation{ops: Ops::Search(args.operand1, args.operand2)},
    //     _ => Operation{ops: Ops::NoOps}
    // };

    let ops = if args.operation == "copy".to_string() {
        filex::Operation{ops: filex::Ops::Copy(args.operand1, args.operand2)}
    } else if args.operation == "count".to_string() {
        filex::Operation{ops: filex::Ops::Count(args.operand1, args.operand2)}
    } else if  args.operation == "search".to_string() {
        filex::Operation{ops: filex::Ops::Search(args.operand1, args.operand2)}
    } else {
        filex::Operation{ops: filex::Ops::NoOps}
    };

    perform_operation(ops);
}
