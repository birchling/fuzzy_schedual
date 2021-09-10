use std::thread;
use std::time::Duration;
use rand::{distributions::Uniform, Rng};

fn generate_and_multiply_small_random_vetor() -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, 20);
    
    let vals: Vec<i32> = (0..10).map(|_| rng.sample(&range)).collect();
    vals.iter().map(|vals| vals*vals).collect()
}

fn main() {
    //let result = generate_and_multiply_small_random_vetor();
    //println!("Result is equal to {:?}", result);
    let pool = ThreadPool::new(4);
    for _ in (1..10) {

    }
}
