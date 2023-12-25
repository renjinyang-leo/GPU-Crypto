extern crate numrs;

use super::aes_cpu::encrypt;

#[test]
fn test_enc_dec_1() {
    let key = "abcdef1234567890";
    let input = "Everything is awesome!";
    let expected_output =
"2a 06 f9 09 dd 48 5d 68 ad 11 9b 2f 25 4b 80 22 
97 53 f8 6a 54 10 e3 bc b0 f1 52 49 b2 0e 0e 3a 
";
    
    let ciphertext = encrypt(input.as_bytes(), key.as_bytes());
    assert_eq!(ciphertext, expected_output);
}

#[test]
fn test_enc_dec_2() {
    let key = "0123456789abcdef";
    let input = "This is the story all about how my life got turned flip upsidown.";
    let expected_output =
"ce ac 12 ce 84 ef 1c ef f9 81 ae e7 6f dc 3c 0d 
85 20 8e f3 8e f5 1a 86 64 9e 67 88 c2 1e aa b4 
77 2d d6 4a 61 e8 3b c3 33 53 75 0d ef 8a 32 ba 
c9 6c 94 f3 17 4a 36 22 94 3b 8f 05 a3 f9 bd fa 
5e 33 7c 06 a5 11 5d 82 79 02 08 8e 1b f6 3b e2 
";

    let ciphertext = encrypt(input.as_bytes(), key.as_bytes());
    assert_eq!(ciphertext, expected_output);
}
