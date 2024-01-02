use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    value: String,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}

fn convert_to_roman(n: u32) -> String {
    let mut ret: String = "".to_string();
    if n > 4000 {
        panic!("invalid value")
    }

    match n / 1000 {
        1 => ret.push('M'),
        2 => ret.push_str("MM"),
        3 => ret.push_str("MMM"),
        _ => (),
    }

    match (n % 1000) / 100 {
        1 => ret.push('C'),
        2 => ret.push_str("CC"),
        3 => ret.push_str("CCC"),
        4 => ret.push_str("CD"),
        5 => ret.push('D'),
        6 => ret.push_str("DC"),
        7 => ret.push_str("DCC"),
        8 => ret.push_str("DCCC"),
        9 => ret.push_str("CM"),
        _ => (),
    }

    match (n % 100) / 10 {
        1 => ret.push('X'),
        2 => ret.push_str("XX"),
        3 => ret.push_str("XXX"),
        4 => ret.push_str("XL"),
        5 => ret.push('L'),
        6 => ret.push_str("LX"),
        7 => ret.push_str("LXX"),
        8 => ret.push_str("LXXX"),
        9 => ret.push_str("XC"),
        _ => (),
    }

    match n % 10 {
        1 => ret.push('I'),
        2 => ret.push_str("II"),
        3 => ret.push_str("III"),
        4 => ret.push_str("IV"),
        5 => ret.push('V'),
        6 => ret.push_str("VI"),
        7 => ret.push_str("VII"),
        8 => ret.push_str("VIII"),
        9 => ret.push_str("IX"),
        _ => (),
    }

    ret
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let value: String = convert_to_roman(num);
        Self { value }
    }
}

fn main() {
    println!("{}", Roman::from(3994));
    println!("{}", Roman::from(3004));
    println!("{}", Roman::from(304));
    println!("{}", Roman::from(1));
    println!("{}", Roman::from(17));
    println!("{}", Roman::from(101));
    println!("{}", Roman::from(121));
    println!("{}", Roman::from(49));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number_1_is_i() {
        let input = 1;
        let output = Roman::from(input).to_string();
        let expected = "I";
        assert_eq!(output, expected);
    }

    #[test]
    fn number_2_is_ii() {
        let input = 2;
        let output = Roman::from(input).to_string();
        let expected = "II";
        assert_eq!(output, expected);
    }

    #[test]
    fn number_3_is_iii() {
        let input = 3;
        let output = Roman::from(input).to_string();
        let expected = "III";
        assert_eq!(output, expected);
    }

    #[test]
    fn number_4_is_iv() {
        let input = 4;
        let output = Roman::from(input).to_string();
        let expected = "IV";
        assert_eq!(output, expected);
    }

    #[test]
    fn number_5_is_v() {
        let input = 5;
        let output = Roman::from(input).to_string();
        let expected = "V";
        assert_eq!(output, expected);
    }

    #[test]
    fn number_6_is_vi() {
        let input = 6;
        let output = Roman::from(input).to_string();
        let expected = "VI";
        assert_eq!(output, expected);
    }

    #[test]
    fn number_9_is_ix() {
        let input = 9;
        let output = Roman::from(input).to_string();
        let expected = "IX";
        assert_eq!(output, expected);
    }

    #[test]
    fn number_16_is_xvi() {
        let input = 16;
        let output = Roman::from(input).to_string();
        let expected = "XVI";
        assert_eq!(output, expected);
    }

    #[test]
    fn number_27_is_xxvii() {
        let input = 27;
        let output = Roman::from(input).to_string();
        let expected = "XXVII";
        assert_eq!(output, expected);
    }

    #[test]
    fn number_48_is_xlviii() {
        let input = 48;
        let output = Roman::from(input).to_string();
        let expected = "XLVIII";
        assert_eq!(output, expected);
    }

    #[test]
    fn number_49_is_xlix() {
        let input = 49;
        let output = Roman::from(input).to_string();
        let expected = "XLIX";
        assert_eq!(output, expected);
    }

    #[test]
    fn number_59_is_lix() {
        let input = 59;
        let output = Roman::from(input).to_string();
        let expected = "LIX";
        assert_eq!(output, expected);
    }

    #[test]
    fn number_66_is_lxvi() {
        let input = 66;
        let output = Roman::from(input).to_string();
        let expected = "LXVI";
        assert_eq!(output, expected);
    }

    #[test]
    fn number_93_is_xciii() {
        let input = 93;
        let output = Roman::from(input).to_string();
        let expected = "XCIII";
        assert_eq!(output, expected);
    }

    #[test]
    fn number_141_is_cxli() {
        let input = 141;
        let output = Roman::from(input).to_string();
        let expected = "CXLI";
        assert_eq!(output, expected);
    }

    #[test]
    fn number_163_is_clxiii() {
        let input = 163;
        let output = Roman::from(input).to_string();
        let expected = "CLXIII";
        assert_eq!(output, expected);
    }

    #[test]
    fn number_166_is_clxvi() {
        let input = 166;
        let output = Roman::from(input).to_string();
        let expected = "CLXVI";
        assert_eq!(output, expected);
    }

    #[test]
    fn number_402_is_cdii() {
        let input = 402;
        let output = Roman::from(input).to_string();
        let expected = "CDII";
        assert_eq!(output, expected);
    }

    #[test]
    fn number_575_is_dlxxv() {
        let input = 575;
        let output = Roman::from(input).to_string();
        let expected = "DLXXV";
        assert_eq!(output, expected);
    }

    #[test]
    fn number_666_is_dclxvi() {
        let input = 666;
        let output = Roman::from(input).to_string();
        let expected = "DCLXVI";
        assert_eq!(output, expected);
    }

    #[test]
    fn number_911_is_cmxi() {
        let input = 911;
        let output = Roman::from(input).to_string();
        let expected = "CMXI";
        assert_eq!(output, expected);
    }

    #[test]
    fn number_1024_is_mxxiv() {
        let input = 1024;
        let output = Roman::from(input).to_string();
        let expected = "MXXIV";
        assert_eq!(output, expected);
    }

    #[test]
    fn number_1666_is_mdclxvi() {
        let input = 1666;
        let output = Roman::from(input).to_string();
        let expected = "MDCLXVI";
        assert_eq!(output, expected);
    }

    #[test]
    fn number_3000_is_mmm() {
        let input = 3000;
        let output = Roman::from(input).to_string();
        let expected = "MMM";
        assert_eq!(output, expected);
    }

    #[test]
    fn number_3001_is_mmmi() {
        let input = 3001;
        let output = Roman::from(input).to_string();
        let expected = "MMMI";
        assert_eq!(output, expected);
    }

    #[test]
    fn number_3999_is_mmmcmxcix() {
        let input = 3999;
        let output = Roman::from(input).to_string();
        let expected = "MMMCMXCIX";
        assert_eq!(output, expected);
    }
}
