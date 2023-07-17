use rand::random;
use rand_utf8::rand_utf8;

fn main() {
    let mut rng = rand::thread_rng();
    println!("{}", rand_utf8(&mut rng, random::<u8>() as usize));
}
