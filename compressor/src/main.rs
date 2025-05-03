use clap::Parser;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
mod compresion;
use compresion::rle;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    comprimir: bool,
    // #[arg(short, long)]
    // descomprimir: bool,
    #[arg(long, default_value = "rle")]
    algoritmo: String,
    #[arg(long)]
    entrada: String,
    #[arg(long)]
    salida: String,
}

fn rle_compress(input: String, output: String) -> std::io::Result<()> {
    let input_file = File::open(input)?;
    let mut reader = BufReader::new(input_file);
    let output_file = File::create(output)?;
    let mut writer = BufWriter::new(output_file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let encoded = rle::encode(&buffer);
    writer.write_all(&encoded)?;

    Ok(())
}

fn rle_decompress(input: String, output: String) -> std::io::Result<()> {
    let input_file = File::open(input)?;
    let mut reader = BufReader::new(input_file);
    let output_file = File::create(output)?;
    let mut writer = BufWriter::new(output_file);

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let decoded = rle::decode(&buffer);
    writer.write_all(&decoded)?;

    Ok(())
}

fn rle_compress_decompress(input: String, output: String, compress: bool) -> std::io::Result<()> {
    if compress {
        rle_compress(input, output)
    } else {
        rle_decompress(input, output)
    }
}


fn file_opener() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let _input: String = args.entrada;
    let _output: String = args.salida;
    println!("Hello, world!");
    if !PathBuf::from(_input.clone()).exists() {
        println!("No existe el archivo de entrada");
        return Ok(());
    }
    rle_compress_decompress(_input.clone(), _output.clone(), args.comprimir)?;

    Ok(())
}

fn main() {
    if let Err(e) = file_opener() {
        eprintln!("Error: {}", e);
    }
}
