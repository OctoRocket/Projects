#![no_std]
#![no_main]
#![feature(lang_items)]

use atmega_hal::{
    pins,
    Peripherals,
    delay::Delay,
    I2c,
    prelude::{
        _embedded_hal_blocking_i2c_Write, 
        _embedded_hal_blocking_delay_DelayMs,
    },
};

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub type SystemClock = atmega_hal::clock::MHz16;

fn primes_under(limit: u16) -> Option<[u16; 1229]> {
    let mut primes = [0u16; 1229];
    let mut index = 0;
    // input 2 as the first prime, otherwise there are no primes
    if limit < 2 {
        return None;
    }
    // from 3 (first number above the first prime) to the limit
    for i in 2..=limit {
        let mut is_prime = true;
        // if the number is divisable a prime number before it, then it is not prime.
        for j in primes {
            if i % j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes[index] = i;
            index += 1
        }
    }
    Some(primes)
}

fn number_to_chars(mut num: u16) -> [u8; 4] {
    let mut char_array = [48u8; 4];
    if !(1..=9999).contains(&num) {
        return [0; 4];
    }
    let num_len = num.checked_ilog10().unwrap_or(0) + 1;
    for i in (1..=num_len).rev() {
        (num, char_array[4-i as usize]) = (num - (num / 10u16.pow(i - 1)*10u16.pow(i - 1)),
        (num / 10u16.pow(i-1)) as u8 + 48);
    }
    char_array
}

fn num_to_display(num: u16) -> [u8; 4] {
    let chars = number_to_chars(num);
    chars.map(|f|
        match f as char {
            '0' => 0b00111111u8,
            '1' => 0b00000110u8,
            '2' => 0b01011011u8,
            '3' => 0b01001111u8,
            '4' => 0b01100110u8,
            '5' => 0b01101101u8,
            '6' => 0b01111101u8,
            '7' => 0b00000111u8,
            '8' => 0b01111111u8,
            '9' => 0b01100111u8,
            _ => 0b00000000u8
        }
    )
}

fn number_to_position(num: u8) -> u8 {
    match num {
        0 => 0b1110_1111u8,
        1 => 0b1101_1111u8,
        2 => 0b1011_1111u8,
        3 => 0b0111_1111u8,
        _ => 0b1111_1111u8
    }
}

#[no_mangle]
pub extern fn main() {
    let mut delay: Delay<SystemClock> = Delay::new();
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);
    let addr = 0b0100_0001u8 >> 1;
    /*               |--| |-||
        Fixed bits --|--|  | |
        Programable bits --| |
        RW bit --------------|
    */
    
    let mut i2c: I2c<SystemClock> = I2c::with_external_pullup(
        dp.TWI,
        pins.pc4,
        pins.pc5,
        400000,
    );
    let mut index = 0;
    let primes = primes_under(9999).unwrap();
    let mut conv_num = num_to_display(primes[index]);
    let mut count = 0;

    i2c.write(addr, &[6, 0b00000001, 0b00000000]).unwrap();

    loop {
        for i in 0..=3 {
            i2c.write(addr, &[2, number_to_position(i), conv_num[i as usize]]).unwrap();
            delay.delay_ms(1u16);
            i2c.write(addr, &[2, 0b00001111u8, 0b00000000u8]).unwrap();
        }
        count += 1;
        if count == 1000 {
            count = 0;
            index += 1;
            conv_num = num_to_display(primes[index]);
        }
    }
}
