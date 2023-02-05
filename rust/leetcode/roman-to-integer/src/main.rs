fn roman_to_int(s: String) -> i32 {
    let mut previous_value = 0;
    let mut total = 0;
    for i in s.chars().rev() {
        let current_value = match i {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!("Incorrect value"),
        };
        if previous_value > current_value {
            total -= current_value;
        } else {
            total += current_value;
        }
        previous_value = current_value
    }
    total
}

fn main() {
    assert_eq!(roman_to_int("III".to_string()), 3);
    assert_eq!(roman_to_int("LVIII".to_string()), 58);
    assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
}
