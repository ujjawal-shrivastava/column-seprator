use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process;

fn write_to_file(name: String, data: &[u8]) {
    let mut ofile = File::create(name).expect("Unable to create file!");
    ofile.write_all(data).expect("Unable to write!");
}

fn run(
    file_path: &str,
    column: usize,
    batch_size: usize,
    prefix: String,
) -> Result<(), Box<dyn Error>> {
    // Print Metadata
    println!("\nConversion Starting ...\n");
    println!("File Path : {}", file_path);
    println!("Column Number : {}", column);
    println!("Batch Size : {}", batch_size);

    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    println!("Total Rows : {}", rdr.records().count());

    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let batches = rdr.records().collect::<Vec<_>>();

    for (pos, rows) in batches.chunks(batch_size).enumerate() {
        let mut values = vec![];
        for row in rows {
            if let Ok(row) = row.as_ref() {
                values.push(format!(
                    "{}{}",
                    prefix,
                    &row.get(column - 1).unwrap_or_default()
                ));
            }
        }
        let output = values.join(",");
        let file_name = format!("Batch {}.txt", pos + 1);
        write_to_file(file_name, output.as_bytes());
    }
    println!("Completed conversion! Machayenge...");
    println!("Press enter to exit . . .");
    std::io::stdin().bytes().next();
    Ok(())
}

fn main() {
    let mut inp = String::new();

    let file_path = loop {
        print!("Enter File Path: ");
        inp.clear();
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut inp).unwrap();
        let file_path = inp.trim().clone().to_string();
        if Path::new(&file_path).is_file() {
            break file_path;
        } else {
            println!("Error: entered file path does not exist. Try again!");
        }
    };

    let column: usize = loop {
        print!("Enter Column Number: ");
        inp.clear();
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut inp).unwrap();
        let v = inp.trim().parse::<usize>();
        if let Ok(v) = v {
            break v;
        } else {
            println!("Error: entered value is not a valid integer. Try again!");
        }
    };

    let batch_size: usize = loop {
        print!("Enter Batch Size: ");
        inp.clear();
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut inp).unwrap();
        let v = inp.trim().parse::<usize>();
        if let Ok(v) = v {
            break v;
        } else {
            println!("Error: entered value is not a valid integer. Try again!");
        }
    };

    let prefix: String = loop {
        print!("Enter Prefix: ");
        inp.clear();
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut inp).unwrap();
        let v = inp.trim().to_string();
        break v;
    };

    if let Err(err) = run(&file_path, column, batch_size, prefix) {
        println!("{}", err);
        process::exit(1);
    }
}
