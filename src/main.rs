use clap::{Arg, Command};
use person::Person;
use prost::encoding::encode_varint;
use protobuf::Message;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::io::{BufRead, BufWriter};
mod person;
//rustfmt clippy
fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("Person Proto Buffer")
        .arg(
            Arg::new("input_file")
                .short('i')
                .long("input-file")
                .required(true)
                .value_parser(clap::value_parser!(String))
                .help("Path to the input data file"),
        )
        .arg(
            Arg::new("output_file")
                .short('o')
                .long("output-file")
                .required(true)
                .value_parser(clap::value_parser!(String))
                .help("Path to the output data file"),
        )
        .get_matches();
    //Storing file paths
    let default_input_data_path = "default_input_file_path".to_string();
    let default_output_data_path = "default_output_file_path".to_string();
    let _input_data_file_path = matches
        .get_one::<String>("input_file")
        .unwrap_or(&default_input_data_path);
    let _output_data_file_path = matches
        .get_one::<String>("output_file")
        .unwrap_or(&default_output_data_path);
    // Generate Protobuf
    gen_protobuf(_input_data_file_path, _output_data_file_path).expect("Parsing error");
    Ok(())
}

fn gen_protobuf(
    file_path: &str,
    _output_data_file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(file_path).expect("error in opening file");
    let reader = BufReader::new(file);
    let lines = reader.lines();
    // let mut file = File::create(_output_data_file_path)?;
    let mut writer = BufWriter::new(
        File::create(_output_data_file_path).expect("Output file path not resolved"),
    );
    for line in lines {
        let line = line.expect("Line was not loaded");
        let mut person = Person::new();
        let fields: Vec<&str> = line.split(',').collect();
        person.first_name = fields[1].to_string();
        person.last_name = fields[0].to_string();
        person.date_of_birth = fields[2].to_string();

        // println!("Person: {:?}", person);

        let protobuf_payload = person.write_to_bytes()?;
        let payload_size = protobuf_payload.len();
        let mut varint_buffer = vec![];
        encode_varint(payload_size as u64, &mut varint_buffer);
        writer.write_all(&varint_buffer).expect("error");
        writer.write_all(&protobuf_payload).expect("error");
        // writer.w
    }
    Ok(())
}
