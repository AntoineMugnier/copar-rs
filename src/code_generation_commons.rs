pub(crate) fn generate_blank_line(output_file: &mut impl std::io::Write) {
    writeln!(output_file).unwrap();
}

pub(crate) fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .filter(|sub| !sub.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

pub(crate) fn pascal_to_snake_case(input: &str) -> String {
    let mut result = String::new();

    for (i, c) in input.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }

    result
}
pub fn to_snake_case(s: &str) -> String {
    // Simple snake_case: replace '-' with '_', lowercase, and insert '_' before uppercase letters
    let mut out = String::new();
    let mut prev_lower = false;
    for ch in s.chars() {
        if ch == '-' {
            out.push('_');
            prev_lower = false;
            continue;
        }
        if ch.is_uppercase() {
            if prev_lower {
                out.push('_');
            }
            for lc in ch.to_lowercase() {
                out.push(lc);
            }
            prev_lower = false;
        } else {
            out.push(ch);
            prev_lower = true;
        }
    }
    out
}
pub(crate) fn pascal_to_macro_case(input: &str) -> String {
    let mut result = String::new();

    for (i, c) in input.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 {
                result.push('_');
            }
            result.push(c.to_ascii_uppercase());
        } else {
            result.push(c.to_ascii_uppercase());
        }
    }

    result
}
