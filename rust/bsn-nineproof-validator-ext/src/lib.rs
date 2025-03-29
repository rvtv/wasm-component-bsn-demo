#[allow(warnings)]
mod bindings;

use bindings::exports::summit25::validator::string_validation::Guest;
use bindings::summit25::validator::string_length_validation;

struct Component;

impl Guest for Component {
    fn validate(value: String) -> bool {
        // check length
        if !string_length_validation::validate(&value, 9) {
            return false;
        }
        
        // for BSN representation ABCDEFGHI, do:
        // (9 × A) + (8 × B) + (7 × C) + (6 × D) + (5 × E) + (4 × F) + (3 × G) + (2 × H) + (-1 × I)
        // sum should be dividable by 11
        const RADIX: u32 = 10;
        let sum = value.chars()
            .enumerate()
            .map(|(i, c)| -> i32 {
                let digit = c.to_digit(RADIX).unwrap() as i32;
                if i == 8 {
                    return digit * -1;
                } else {
                    return digit * (9 - i as i32);
                }
            })
            .sum::<i32>();
        
        sum % 11 == 0
    }
}

bindings::export!(Component with_types_in bindings);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bsn_valid() {
        const BSN: &str = "111222333";

        assert_eq!(Component::validate(BSN.to_string()), true);
    }

    #[test]
    fn bsn_invalid() {
        const BSN: &str = "111222334";

        assert_eq!(Component::validate(BSN.to_string()), false);
    }
}