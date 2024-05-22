use std::str::FromStr;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

fn main() {
    let number = dec!(1.999999999999) + dec!(0.000000000001);
    assert_eq!(number, dec!(2));
    // assert_eq!(number.to_string(), "2.0");

    // Using an integer followed by the decimal points
    let scaled = Decimal::new(202, 2);
    assert_eq!("2.02", scaled.to_string());

    // From a 128 bit integer
    let balance = Decimal::from_i128_with_scale(5_897_932_384_626_433_832, 2);
    assert_eq!("58979323846264338.32", balance.to_string());

    // From a string representation
    let from_string = Decimal::from_str("2.02").unwrap();
    assert_eq!("2.02", from_string.to_string());

    // From a string representation in a different base
    let from_string_base16 = Decimal::from_str_radix("ffff", 16).unwrap();
    assert_eq!("65535", from_string_base16.to_string());

    // From scientific notation
    let sci = Decimal::from_scientific("9.7e-7").unwrap();
    assert_eq!("0.00000097", sci.to_string());

    // Using the `Into` trait
    let my_int: Decimal = 3_i32.into();
    assert_eq!("3", my_int.to_string());

    // Using the raw decimal representation
    let pi = Decimal::from_parts(1_102_470_952, 185_874_565, 1_703_060_790, false, 28);
    assert_eq!("3.1415926535897932384626433832", pi.to_string());
}
