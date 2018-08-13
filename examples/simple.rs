extern crate rand;
extern crate roulette;

use roulette::Roulette;

fn main() {
    let mut rng = rand::thread_rng();
    let roulette = Roulette::new(vec![('a', 1.0), ('b', 1.0), ('c', 0.5), ('d', 0.0)]);
    for _ in 0..10 {
        let rand = roulette.sample(&mut rng);
        println!("{}", rand);
    }
}
