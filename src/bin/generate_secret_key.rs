use argonautica::utils::generate_random_base64_encoded_string;
fn main() {
    println!("{}", generate_random_base64_encoded_string(33).unwrap());
}
