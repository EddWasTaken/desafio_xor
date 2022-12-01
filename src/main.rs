use std::{
    fs::{
        File, 
        OpenOptions
    },
    io::{
        BufRead,
        BufReader, 
        Write
    }, 
};
use hex::{
    decode,
    encode
};
use text_io::read;
use chrono::{
    DateTime,
    Utc,
};

fn main() {

    test_xor();

    let lines = get_lines("ciphers.txt");

    // reads a string until a newline appears (not including it)
    println!("Insert your guess for crib dragging:");
    let guess: String = read!("{}\n");
    let hex_guess = encode(&guess);

    //let file = File::create("guessed.txt").expect("File couldn't be created");
    let file = OpenOptions::new()
                                            .append(true)
                                            .create(true)
                                            .open("guesses.txt")
                                            .expect("File couldn't be created or opened, exiting.");
    let now: DateTime<Utc> = Utc::now();
    writeln!(&file, "\nTimestamp: {}\nGuess-word given: {}\n", now.format("%a %b %e %T %Y"), guess).expect("Couldn't write timestamp to file.");
    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            let xored_lines = hex_xor(lines[i].to_string(), lines[j].to_string());
            let final_xor = hex_xor(hex_guess.to_string(), xored_lines);
            //println!("(Cipher {} ^ Cipher {}) ^ Guess: {} = {}", i, j, guess, hex_to_utf8(final_xor));
            writeln!(&file, "(Cipher {} ^ Cipher {}) ^ Guess = {}", i, j, hex_to_utf8(final_xor)).expect("Couldn't write final XOR to file.");
        }
    }



    

}
/// Opens a given file and reads each line into a Vector of Strings
fn get_lines(filename: &str) -> Vec<String> {

    let file = File::open(filename).expect("No such file");
    let buff = BufReader::new(file);
    buff.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

/**
 * Checking if the program is working as intended before moving on to real crib dragging
 * The key used was "engenhariainformatica"
 * The messages are encoded in hex prior to the One Time Pad
*/
fn test_xor(){

    let m1 = "7465737465786f72";                                            // "testexor"
    let m2 = "786f727465737465";                                            // "xorteste"
    let key = "656e67656e6861726961696e666f726d6174696361";                 // "engenhariainformatica"
    let m1_otp = hex_xor(String::from(m1), String::from(key)); // m1 ^ key
    let m2_otp = hex_xor(String::from(m2), String::from(key)); // m2 ^ key
    let xor_otp = hex_xor(m1_otp, m2_otp);                     // m1_otp ^ m2_otp
    assert_eq!(xor_otp, "0c0a0100000b1b17");                                      // checks if xor_otp is equal to the predicted value, else panics                                                     
    let guess = encode("testexor");                                 // guess in hex form
    let xor_guess = hex_xor(guess, xor_otp);                  // guess ^ xor_otp
    assert_eq!("xorteste", String::from_utf8(decode(xor_guess).unwrap()).unwrap());

}

/// Executes the following bitwise operation: hex1 ^ hex2, returning the result encoded in a String
fn hex_xor(hex1: String, hex2: String) -> String {

    // binary vectors
    let bin_vec1 = decode(hex1);
    let bin_vec2 = decode(hex2);
    // XORs both messages
    let bin_vec3: Vec<u8> = bin_vec1.unwrap()
        .iter()
        .zip(bin_vec2.unwrap().iter())
        .map(|(&x1, &x2)| x1 ^ x2)
        .collect();
    return encode(bin_vec3);
}

fn hex_to_utf8(hex: String) -> String {
    return String::from_utf8_lossy(&decode(hex).unwrap()).as_ref().to_string();
}