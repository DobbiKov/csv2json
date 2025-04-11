use clap::Parser;
use csv::Reader;
use serde_json::{Map, Value};
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Duration;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "Input CSV file")]
    input: String,

    #[arg(help = "Output JSONL file")]
    output: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let csv_filename = args.input;
    let jsonl_filename = args.output;

    convert_csv(&csv_filename, &jsonl_filename)?;

    println!("Conversion complete!");

    Ok(())
}

fn convert_csv(input_filename: &str, output_filename: &str) -> Result<(), Box<dyn Error>> {
    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_message("Reading CSV file...");
    spinner.enable_steady_tick(Duration::from_millis(100));

    let mut rdr = Reader::from_path(input_filename)?;
    let headers = rdr
        .headers()?
        .iter()
        .map(|h| h.to_string())
        .collect::<Vec<String>>();

    spinner.set_message("Reading CSV file... Done!");

    let output_file = File::create(output_filename)?;
    let mut writer = BufWriter::new(output_file);

    for (num_read, result) in rdr.records().enumerate() {
        let record = result?;
        let mut map = Map::new();

        for (header, field) in headers.iter().zip(record.iter()) {
            map.insert(header.to_string(), Value::String(field.to_string()));
        }

        spinner.set_message(format!("Writing record {}...", num_read));
        let json = Value::Object(map).to_string();
        writeln!(writer, "{}", json)?;
    }

    spinner.finish();

    Ok(())
}
