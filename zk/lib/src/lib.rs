#[derive(Debug)]
struct MRZData {
    document_type: char,
    country_code: String,
    surname: String,
    given_names: Vec<String>,
    passport_number: String,
    passport_checksum: u32,
    issuing_country: String,
    date_of_birth: String,
    dob_checksum: u32,
    gender: char,
    expiry_date: String,
    expiry_checksum: u32,
    final_checksum: u32,
}

fn parse_mrz(mrz: &str) -> MRZData {
    // First, split the MRZ into its main parts
    let parts: Vec<&str> = mrz.split('<').collect();

    // Parse the first part (document type and country code)
    let doc_type = parts[0].chars().next().unwrap_or_else(|| {
        panic!("Invalid document type");
    });

    // Parse surname and given names
    let country_code = parts[1][..3].to_string();
    let surname = parts[1][3..].to_string();
    let given_names: Vec<String> = parts[3..]
        .iter()
        .take_while(|&&part| !part.chars().all(|c| c.is_ascii_digit()))
        .map(|&s| s.to_string())
        .collect();

    // Find the start of the fixed-length fields by looking for the passport number pattern
    let mut fixed_fields_start = 0;
    for (i, part) in parts.iter().enumerate() {
        if part.len() >= 9 && part[..9].chars().all(|c| c.is_ascii_digit()) {
            fixed_fields_start = i;
            break;
        }
    }

    // Get the fixed-length fields by concatenating parts until we have enough characters
    let mut fixed_fields = String::new();
    let mut current_part = fixed_fields_start;
    while fixed_fields.len() < 44 && current_part < parts.len() {
        fixed_fields.push_str(parts[current_part]);
        current_part += 1;
    }

    // Parse the fixed-length fields
    let passport_number = fixed_fields[..9].to_string();
    let passport_checksum = fixed_fields[9..10]
        .chars()
        .next()
        .and_then(|c| c.to_digit(10))
        .unwrap_or_else(|| {
            panic!("Invalid passport number checksum");
        });
    let issuing_country = fixed_fields[10..13].to_string();
    let date_of_birth = fixed_fields[13..19].to_string();
    let dob_checksum = fixed_fields[19..20]
        .chars()
        .next()
        .and_then(|c| c.to_digit(10))
        .unwrap_or_else(|| {
            panic!("Invalid date of birth checksum");
        });
    let gender = fixed_fields[20..21].chars().next().unwrap_or(' ');
    let expiry_date = fixed_fields[21..27].to_string();
    let expiry_checksum = fixed_fields[27..28]
        .chars()
        .next()
        .and_then(|c| c.to_digit(10))
        .unwrap_or_else(|| {
            panic!("Invalid expiry date checksum");
        });

    // Find the final checksum from the last part
    let final_checksum = parts
        .last()
        .and_then(|s| s.chars().rev().find(|c| c.is_ascii_digit()))
        .and_then(|c| c.to_digit(10))
        .unwrap_or_else(|| {
            panic!("Invalid final checksum");
        });

    MRZData {
        document_type: doc_type,
        country_code,
        surname,
        given_names,
        passport_number,
        passport_checksum,
        issuing_country,
        date_of_birth,
        dob_checksum,
        gender,
        expiry_date,
        expiry_checksum,
        final_checksum,
    }
}

fn compute_checksum(data: &str) -> u32 {
    let mut position = 1;
    let sum: u32 = data
        .chars()
        .map(|c| {
            let multiplier = match position % 3 {
                1 => 7,
                2 => 3,
                0 => 1,
                _ => unreachable!(),
            };

            let char_value = if c.is_ascii_digit() {
                c.to_digit(10).unwrap()
            } else if c.is_ascii_alphabetic() {
                (c.to_ascii_uppercase() as u32) - ('A' as u32) + 10
            } else {
                0
            };
            position += 1;
            char_value * multiplier
        })
        .sum();
    sum % 10
}

fn validate_checksums(mrz_data: &MRZData) -> bool {
    let passport_checksum = compute_checksum(&mrz_data.passport_number);
    let dob_checksum = compute_checksum(&mrz_data.date_of_birth);
    let expiry_checksum = compute_checksum(&mrz_data.expiry_date);

    let final_checksum_data = format!(
        "{}{}{}{}{}{}",
        mrz_data.passport_number,
        mrz_data.passport_checksum,
        mrz_data.date_of_birth,
        mrz_data.dob_checksum,
        mrz_data.expiry_date,
        mrz_data.expiry_checksum
    );
    let final_checksum = compute_checksum(&final_checksum_data);

    passport_checksum == mrz_data.passport_checksum
        && dob_checksum == mrz_data.dob_checksum
        && expiry_checksum == mrz_data.expiry_checksum
        && final_checksum == mrz_data.final_checksum
}

// Returns whether the passport is valid and the full name of the person
pub fn validate_passport(mrz: String) -> (bool, String) {
    let mrz_data = parse_mrz(&mrz);
    let is_valid = validate_checksums(&mrz_data);

    let name = format!("{} {}", mrz_data.given_names.join(" "), mrz_data.surname);
    (is_valid, name)
}
