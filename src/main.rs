use std::fs::File;

use clap::{Args, Parser, Subcommand};
use copar::{CGeneration, CSharpGeneration, Model, RustGeneration};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: GenerationCommand,
}

#[derive(Args)]
struct CSharpArgs {
    input_file_path: String,
    output_file_path: String,
}

#[derive(Args)]
struct RustArgs {
    input_file_path: String,
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
    GenerateCSharp(CSharpArgs),
    GenerateC(CArgs),
    GenerateRust(RustArgs),
}

fn generate_rust(input_log_file_path: &str, output_file_path: &str) {
    let input_file = File::open(input_log_file_path).unwrap();
    let mut output_file = File::create(output_file_path).unwrap();

    let model = Model::parse(input_file).unwrap();
    model.compute_to_rust(&mut output_file);
}
fn generate_csharp(input_log_file_path: &str, output_file_path: &str) {
    let input_file = File::open(input_log_file_path).unwrap();
    let mut output_file = File::create(output_file_path).unwrap();

    let model = Model::parse(input_file).unwrap();
    model.compute_to_cs(&mut output_file);
}

fn generate_c(input_log_file_path: &str, output_c_file_path: &str, output_h_file_path: &str) {
    let input_file = File::open(input_log_file_path).unwrap();
    let mut output_c_file = File::create(output_c_file_path).unwrap();
    let mut output_h_file = File::create(output_h_file_path).unwrap();

    let model = Model::parse(input_file).unwrap();
    model.compute_to_c(&mut output_c_file, &mut output_h_file);
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        GenerationCommand::GenerateCSharp(cs_args) => {
            generate_csharp(
                cs_args.input_file_path.as_str(),
                cs_args.output_file_path.as_str(),
            );
        }
        GenerationCommand::GenerateC(c_args) => {
            generate_c(
                c_args.input_file_path.as_str(),
                c_args.output_c_file_path.as_str(),
                c_args.output_h_file_path.as_str(),
            );
        }
        GenerationCommand::GenerateRust(rust_args) => {
            generate_rust(
                rust_args.input_file_path.as_str(),
                rust_args.output_file_path.as_str(),
            );
        }
    }
}
