use clap::Parser;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
mod compresion;
use compresion::rle;

/// Command line arguments structure
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Activate compression mode (default is decompression)
    #[arg(short, long)]
    comprimir: bool,

    /// Specify the compression algorithm
    #[arg(long, default_value = "rle")]
    algoritmo: String,

    /// Input file path
    #[arg(long)]
    entrada: PathBuf, // Use PathBuf for better path handling

    /// Output file path
    #[arg(long)]
    salida: PathBuf, // Use PathBuf for better path handling
}

/// Reads all bytes from a file specified by the path.
///
/// # Arguments
/// * `path` - A reference to the path of the file to read.
///
/// # Returns
/// * `io::Result<Vec<u8>>` - A Result containing the file content as bytes or an IO error.
fn read_file_bytes(path: &Path) -> io::Result<Vec<u8>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}

/// Writes a slice of bytes to a file specified by the path.
/// Creates the file if it doesn't exist, truncates it if it does.
///
/// # Arguments
/// * `path` - A reference to the path of the file to write.
/// * `data` - The byte slice to write to the file.
///
/// # Returns
/// * `io::Result<()>` - A Result indicating success or an IO error.
fn write_file_bytes(path: &Path, data: &[u8]) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(data)?;
    Ok(())
}

/// Compresses data using the RLE algorithm.
///
/// # Arguments
/// * `data` - The byte slice to compress.
///
/// # Returns
/// * `Vec<u8>` - The compressed data.
fn rle_compress(data: &[u8]) -> Vec<u8> {
    rle::encode(data)
}

/// Decompresses data using the RLE algorithm.
///
/// # Arguments
/// * `data` - The byte slice to decompress.
///
/// # Returns
/// * `Vec<u8>` - The decompressed data.
fn rle_decompress(data: &[u8]) -> Vec<u8> {
    rle::decode(data)
}

/// Runs the compression/decompression tool based on command-line arguments.
/// Orchestrates reading, processing, and writing.
///
/// # Returns
/// * `io::Result<()>` - A Result indicating overall success or an IO error.
fn run() -> io::Result<()> {
    let args = Args::parse();

    // Input validation (already implicitly checked by File::open in read_file_bytes)
    // We could add an explicit check earlier if desired:
    if !args.entrada.exists() {
        // Use eprintln! for errors, print to stderr
        eprintln!("Error: El archivo de entrada no existe: {:?}", args.entrada);
        // Return an error of kind NotFound
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Input file not found",
        ));
    }

    // Read input file content
    println!("Leyendo archivo: {:?}", args.entrada);
    let input_data = read_file_bytes(&args.entrada)?;
    println!("Leídos {} bytes.", input_data.len());

    let output_data = match args.algoritmo.as_str() {
        "rle" => {
            println!("Usando algoritmo RLE.");
            if args.comprimir {
                println!("Comprimiendo...");
                rle_compress(&input_data)
            } else {
                println!("Descomprimiendo...");
                rle_decompress(&input_data)
            }
        }
        _ => {
            eprintln!("Error: Algoritmo '{}' no soportado.", args.algoritmo);
            // Return an error for unsupported algorithm
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unsupported algorithm",
            ));
        }
    };

    // Write the processed data to the output file
    println!("Escribiendo archivo: {:?}", args.salida);
    write_file_bytes(&args.salida, &output_data)?;
    println!("Escritos {} bytes.", output_data.len());

    println!("Proceso completado exitosamente.");
    Ok(())
}

/// Main entry point of the application.
fn main() {
    // Execute the main logic and handle potential errors
    if let Err(e) = run() {
        eprintln!("Error en la ejecución: {}", e);
        // Exit with a non-zero status code to indicate failure
        std::process::exit(1);
    }
}
