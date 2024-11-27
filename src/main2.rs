use clap::{Arg, Command};
use prost::bytes::{Buf, Bytes};
use prost::encoding::decode_varint;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
mod person; // Import the `person` module
use person::Person;
use protobuf::Message;

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

    // File paths
    let default_input_data_path = "default_input_file_path".to_string();
    let default_output_data_path = "default_output_file_path".to_string();
    let input_data_file_path = matches
        .get_one::<String>("input_file")
        .unwrap_or(&default_input_data_path);
    let output_data_file_path = matches
        .get_one::<String>("output_file")
        .unwrap_or(&default_output_data_path);

    // Generate text
    gen_text(input_data_file_path, output_data_file_path)?;
    Ok(())
}

fn gen_text(input_file_path: &str, output_file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(input_file_path)?;
    let mut reader = BufReader::new(file);

    let output_file = File::create(output_file_path)?;
    let mut writer = BufWriter::new(output_file);

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let mut buffer = Bytes::from(buffer); // Convert to Bytes (which implements `Buf`)

    // Process each entry
    while buffer.remaining() > 0 {
        // Decode the VarInt for the size
        let size = decode_varint(&mut buffer)? as usize;

        // Read the corresponding payload
        let protobuf_payload = buffer.split_to(size);

        // Deserialize the protobuf payload into a `Person` object
        let person = Person::parse_from_bytes(&protobuf_payload)?;

        // Print the deserialized Person (for debugging)
        println!("{:?}", person);

        // Write the deserialized data to the output file
        writeln!(
            writer,
            "{},{},{}",
            person.last_name, person.first_name, person.date_of_birth
        )?;
    }

    println!("Deserialized data written to '{}'.", output_file_path);

    Ok(())
}
