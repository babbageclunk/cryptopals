mod common;

#[cfg(test)]
mod tests {
    use crate::common;

    #[test]
    fn c1() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(common::hex_to_base64(input), expected);
    }

    #[test]
    fn c2() {
        let a = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
        let b = hex::decode("686974207468652062756c6c277320657965").unwrap();
        let expected = "746865206b696420646f6e277420706c6179";
        assert_eq!(hex::encode(common::xor(&a, &b)), expected);
    }
}
