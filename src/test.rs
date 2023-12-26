extern crate numrs;

use super::aes_cpu::AesEcb;

#[test]
fn test_enc_dec_1() {
    let key = "abcdef1234567890";
    let input = "Everything is awesome!";
    let expected_output ="2a06f909dd485d68ad119b2f254b80229753f86a5410e3bcb0f15249b20e0e3a";

    let aes_tool = AesEcb::new(key.as_bytes());
    let ciphertext = aes_tool.encrypt(input.as_bytes());
    
    assert_eq!(ciphertext, expected_output);
}
