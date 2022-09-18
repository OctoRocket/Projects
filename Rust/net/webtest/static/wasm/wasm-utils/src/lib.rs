use wasm_bindgen::prelude::*;
use integer_sqrt::IntegerSquareRoot;

fn gcd(mut n1: u64, mut n2: u64) -> u64 {
    loop {
        if n1 == 0 || n2 == 0 {
            if n1 > n2 {
                return n1;
            } else {
                return n2;
            }
        } else if n1 > n2 || n1 == n2 {
            n1 = n1 % n2;
        } else if n2 > n1 {
            n2 = n2 % n1;
        }
    };
}

fn p_squares(limit: i32) -> Vec<i32> {
    let mut i = 1;
    let mut vec: Vec<i32> = Vec::new();
    while i * i <= limit {
        vec.push(i * i);
        i += 1;
    }
    vec
}

#[wasm_bindgen]
pub fn factors(n: i32) -> String {
    let range = (1..n).filter(|x| n % x == 0);
    let mut vec = Vec::new();
    // format it correctly
    for i in range {
        if n % i == 0 {
            vec.push((n/i).to_string() + "*" + &i.to_string());
        }
    }
    // remove half of the list, as half of it will just be the flipped version of the other half (eg. 9*2 and 2*9)
    if vec.len() != 2 {
        for _i in vec.len() / 2 + 1..vec.len() {
            vec.pop();
        }
    }
    let mut packed_vec = "".to_string();
    for i in vec {
        packed_vec = packed_vec + i.as_str() + "|";
    }
    packed_vec.pop();
    packed_vec
}

#[wasm_bindgen]
pub fn primes(limit: i32) -> String {
    let mut primes = Vec::new();
    // input 2 as the first prime, otherwise there are no primes
    if limit >= 2 {
        primes.push(2);
    } else {
        return "".to_string();
    }
    // from 3 (first number above the first prime) to the limit
    for i in 3..=limit {
        let mut is_prime = true;
        // if the number is divisable a prime number before it, then it is not prime.
        for j in &primes {
            if i % j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime == true {
            primes.push(i);
        }
    }
    // make the vector into a packed string
    let mut packed_primes = "".to_string();
    for i in primes {
        packed_primes = packed_primes + i.to_string().as_str() + "|";
    }
    packed_primes.pop();
    packed_primes
}


#[wasm_bindgen]
pub fn dec_to_frac(input: String) -> String {
    if input.matches(".").count() != 1 {
        return "".to_string();
    }
    // get the number of places after the decimal point (eg. 10.48 is 2) 
    let places_after_point = input.split(".").nth(1).unwrap().len() as u32;
    // process the input 
    let input = {
        let split = input.split(".").collect::<Vec<&str>>();
        let split = split[0].to_string() + split[1];
        split.parse::<u64>().unwrap()
    };
    let denominator = 10u64.pow(places_after_point);
    let gcd = gcd(input, denominator);
    format!("{}/{}", input/gcd, denominator/gcd)
}

#[wasm_bindgen]
pub fn radical_simplifier(num: i32) -> String {

    let mut ps = p_squares(num);
    // sort perfect squares from biggest to smallest and remove the number one
    ps.reverse();
    ps.pop();
    // for every perfect square, if it devides with no remainder, then check if the number is a perfect square and format it accordingly
    for i in ps {
        if num % i == 0 {
            if num / i == 1 {
                return format!("{}", num.integer_sqrt());
            } else {
                return format!("{}√{}", &i.integer_sqrt(), num / i);
            }
        }
    }
    return format!("√{}", num);
}

#[cfg(test)]
mod tests {
    #[test]
    fn factor_test() {
        assert_eq!(crate::factors(8), "8*1|4*2");
        assert_eq!(crate::factors(16), "16*1|8*2|4*4");
        assert_eq!(crate::factors(182), "182*1|91*2|26*7|14*13");
    }

    #[test]
    fn prime_test() {
        assert_eq!(crate::primes(1), "");
        assert_eq!(crate::primes(2), "2");
        assert_eq!(crate::primes(10), "2|3|5|7");
        assert_eq!(crate::primes(25), "2|3|5|7|11|13|17|19|23");
        assert_eq!(crate::primes(97), "2|3|5|7|11|13|17|19|23|29|31|37|41|43|47|53|59|61|67|71|73|79|83|89|97")
    }

    #[test]
    fn dec_to_frac_test() {
        assert_eq!(crate::dec_to_frac("1".to_string()), "");
        assert_eq!(crate::dec_to_frac("0.125".to_string()), "1/8");
        assert_eq!(crate::dec_to_frac("1.625".to_string()), "13/8");
    }

    #[test]
    fn radical_simplifier_test() {
        assert_eq!(crate::radical_simplifier(9), "3");
        assert_eq!(crate::radical_simplifier(16), "4");
        assert_eq!(crate::radical_simplifier(12), "2√3");
        assert_eq!(crate::radical_simplifier(17), "√17");
    }
}
