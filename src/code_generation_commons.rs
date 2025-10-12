pub(crate) fn generate_blank_line(output_file: &mut impl std::io::Write) {
    writeln!(output_file).unwrap();
}
