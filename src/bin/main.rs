use fuzzy_thread_pool::ThreadPool;
use std::thread;
use std::time::Duration;
use rand::{distributions::Uniform, Rng};

fn generate_and_multiply_small_random_vetor(){
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, 20);
    let vals: Vec<i32> = (0..10).map(|_| rng.sample(&range)).collect();
    let ret: Vec<i32> = vals.iter().map(|vals| vals * vals).collect();
    println!("Result is equal to {:?}", ret);
}

fn main() {
    //let result = generate_and_multiply_small_random_vetor();
    //println!("Result is equal to {:?}", result);
    let pool = ThreadPool::new(4);
    for i in 0..10 {
        println!("{}", i);
        pool.execute(|| {
            generate_and_multiply_small_random_vetor();
        });
    }

}
