

fn main() {
    // Read the first command line argument as a string
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Check if it is a valid IP address
    if is_valid_ip(&input) {
        // Split the IP address into octets
        let octets = input.split(".").collect::<Vec<&str>>();
        // Convert each octet to an integer
        let octets = octets.iter().map(|x| x.parse::<u8>().unwrap()).collect::<Vec<u8>>();
        // Convert each octet to a binary representation with to_binary()
        let octets = octets.iter().map(|x| to_binary(x)).collect::<Vec<Vec<bool>>>();
        // Concatenate the binary representations
        let mut binary = Vec::new();
        for octet in octets {
            binary.extend(octet);
        }
        // Convert the binary representation to a floating point number
        let decimal = to_float(&binary);

        // Print the decimal representation
        println!("{}", decimal);
        return
    } else {
        println!("{} is not a valid IP address!", input);
        return;
    }
}

fn is_valid_ip(input: &str) -> bool {
    // Attempt to split the string into four parts
    let parts: Vec<&str> = input.split('.').collect();

    // Check if the number of parts is not equal to 4
    if parts.len() != 4 {
        return false;
    }

    // Check if each part is a valid number
    for part in parts {
        if !is_valid_number(part) {
            return false;
        }
    }

    return true;
}

fn is_valid_number(input: &str) -> bool {
    // Check if the string is empty
    if input.is_empty() {
        return false;
    }
    // Check if the string is a number
    if input.parse::<u8>().is_ok() {
        return true;
    }
    return false;
}

// Cast the number to its binary representation
fn to_binary(input: &u8) -> Vec<bool> {
    let mut out: Vec<bool> = Vec::new();

    // Convert the number to binary
    let mut num = input.to_owned();
    while num > 0 {
        out.push(num % 2 == 1);
        num /= 2;
    }

    // Reverse the order of the binary representation
    out.reverse();

    return out;
}

fn concat_vectors(vec1: &Vec<bool>, vec2: &Vec<bool>) -> Vec<bool> {
    let mut out: Vec<bool> = Vec::new();
    out.extend(vec1);
    out.extend(vec2);
    return out;
}

// Convert a binary representation to a floating point number according to IEEE 754
fn to_float(input: &Vec<bool>) -> f32 {
    // As we'll be damaging the input, we'll make an owned copy
    let mut input = input.to_owned();

    // The first bit is the sign bit
    let sign = input.remove(0);
    // The next 8 bits are the exponent
    let exponent_bits = input.drain(0..8).collect::<Vec<bool>>();
    // Turn them into an integer for math later
    let exponent = to_integer(&exponent_bits);
    // Free the exponent, we don't need it anymore
    drop(exponent_bits);

    // Our real exponent is the exponent minus 127
    let real_exponent = exponent - 127;

    // The next 23 bits are the mantissa
    let mantissa_bits = input.drain(0..23).collect::<Vec<bool>>();
    // The mantissa has an implicit 1 bit at the start
    let mantissa = to_integer(&concat_vectors(&vec![true], &mantissa_bits));
    // Free the mantissa, we don't need it anymore
    drop(mantissa_bits);

    // The real mantissa is the mantissa divided by 2^23
    let real_mantissa = mantissa as f32 / (2.0_f32).powi(23);

    // The real number is the real mantissa times 2^real_exponent
    let real_number = real_mantissa * (1 << real_exponent) as f32;

    // If the sign bit is 1, the number is negative
    if sign {
        return 0.0 - real_number;
    } else {
        return real_number;
    }
}

fn to_integer(input: &Vec<bool>) -> u8 {
    let mut out: u8 = 0;
    for (i, bit) in input.iter().enumerate() {
        if *bit {
            out += 1 << i;
        }
    }
    return out;
}