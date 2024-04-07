use std::{thread::spawn, time::Instant};

type Num = usize;
const THREADS: Num = 8;

fn phi(mut x: Num) -> Num {
    let mut y = 1;
    while x != 0 {
        match x % 10 {
            0 => {},
            n => y *= n
        }
        x /= 10;
    }
    y
}

fn main() {
    let start = Instant::now();

    let mut threads = Vec::new();
    for n in 0..THREADS {
        threads.push(spawn(move || {
            let mut repartition = [0_u32; 10];
            for i in 0..1_000_000_000/THREADS {
                let mut y = (i*THREADS) + n;
                while y >= 10 {
                    y = phi(y)
                }
                repartition[y] += 1;
            }
            repartition
        }));
    }

    let mut repartition = [0_u32; 10];
    for thread in threads {
        let r = thread.join().unwrap();
        for i in 0..10 {
            repartition[i] += r[i];
        }
    }
    
    println!("{:#?}", repartition);
    println!("Done in {:?}", start.elapsed());
}
