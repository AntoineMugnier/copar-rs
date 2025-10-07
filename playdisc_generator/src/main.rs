use std::{env, fs::File};

use clap::{Args, Parser, Subcommand};
use playdisc_generator::CGeneration;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: GenerationCommand,
}

#[derive(Args)]
struct CSharpArgs {
    input: String,
    output_file_path: String,
}

#[derive(Args)]
struct CArgs {
    input_file_path: String,
    output_c_file_path: String,
    output_h_file_path: String,
}

#[derive(Subcommand)]
enum GenerationCommand {
/// Adds files to myapp
    GenerateCSharp(CSharpArgs),
    GenerateC(CArgs),
    
}

// fn generate_csharp(input_log_file_path: &str, output_file_path: &str) {
//     let input_file = File::open(input_log_file_path).unwrap();
//     let mut output_file = File::create(output_file_path).unwrap();

//     let converter = playdisc_generator::Parser::new(input_file);
//     let model = converter.parse_file().unwrap();
//     model.compute_to_csharp(&mut output_file);
// }

fn generate_c(input_log_file_path: &str, output_c_file_path: &str, output_h_file_path: &str) {
    
    let input_file = File::open(input_log_file_path).unwrap();
    let mut output_c_file = File::create(output_c_file_path).unwrap();
    let mut output_h_file = File::create(output_h_file_path).unwrap();

    let converter = playdisc_generator::Parser::new(input_file);
    let mut model = converter.parse_file().unwrap();
    model.compute_to_c(&mut output_c_file, &mut output_h_file);
}

fn main() {
    let mut args = Cli::parse();

    match &args.command{
        GenerationCommand::GenerateCSharp(csharp_args) => {
            // generate_csharp(&csharp_args);
        }
        GenerationCommand::GenerateC(c_args) => {
            generate_c(c_args.input_file_path.as_str(), c_args.output_c_file_path.as_str(), c_args.output_h_file_path.as_str());
        }
    }
}
