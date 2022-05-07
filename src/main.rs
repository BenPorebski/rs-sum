use rand::Rng;
use std::time;

///Non-branch predicted for loop that sums numbers.
fn nonbranched_loop(a: &Vec<i32>) {
    let mut s: i32 = 0;
    let t_start = time::Instant::now();
    for &x in a {
            s += (x < 50) as i32 * x;
    }
    println!("Summed {} random numbers in {}us.", a.len(), t_start.elapsed().as_micros());
    println!("{}", s);
}

///Non-branch predicted for loop that sums numbers.
fn nonbranched_sum(a: &Vec<i32>) {
    let t_start = time::Instant::now();
    let s = a.iter().fold(0, |acc, &x| 
        acc + (x < 50) as i32 * x
    );

    println!("Summed {} random numbers in {}s.", a.len(), t_start.elapsed().as_secs_f32());
    println!("{}", s);

}

fn eval_random_sum() {
    const N_: usize = 100000000;
    let mut rng = rand::thread_rng();
    let mut a: Vec<i32> = vec![0; N_];
    let t_start = time::Instant::now();
    for i in 0..N_ {
        a[i] = rng.gen_range(0..1000);
    }
    println!("Generated {} random numbers in {}ms.", N_, t_start.elapsed().as_millis());

    branched_loop(&a);
    nonbranched_loop(&a);
    nonbranched_sum(&a);
}


fn main() {
    eval_random_sum();
}
