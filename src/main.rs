use minecraft_nether_gen_rs::{NetherBiomes, NetherGen};
use std::{any::Any, collections::HashSet, env::{self, args}, fmt::{DebugTuple, Display}, iter::Enumerate, ops::{Index, Neg}, process::exit};
use rand::prelude::*;
use std::time::{Duration, Instant};
use std::io;

fn main() {

    let now = Instant::now();

    let mut default_dist = "".to_string();
    let mut default_interval = "".to_string();

    // take arguments for searching and generation
    let args: Vec<String> = env::args().collect();

    
    if args.len() == 1 {
        println!("Enter search distance:");
        io::stdin()
            .read_line(&mut default_dist)
            .expect("Failed to read input");
        default_dist = default_dist.parse().unwrap();

        println!("Enter search interval:");
        io::stdin()
            .read_line(&mut default_interval)
            .expect("Failed to read input");
        default_interval = default_interval.parse().unwrap();
    }

    let distance: i32 = match args
        .get(1)
        .or(Some(&default_dist))
        .unwrap()
        .parse() {
            Ok(x) => x,
            Err(e) => {
                println!("failed to parse distance");
                std::process::exit(1);

            }
        };

    let interval: usize = match args
        .get(2)
        .or(Some(&default_interval))
        .unwrap()
        .parse() {
            Ok(x) => x,
            Err(e) => {
                println!("failed to parse interval");
                std::process::exit(1);
            }
        };

    let mut found_seed = false;
    let mut attempts = 0;
    while found_seed == false {

        println!("{} seeds searched in {} seconds", attempts, now.elapsed().as_secs());

        let mut rng = thread_rng();
        let seed: u32 = rng.gen();
        //println!("{}", seed);

        let mut nether = NetherGen::new(seed.into());

        let mut found_biomes= HashSet::new();

        let mut last_check = (0,0);

        // check along x axis
        for i in (distance.neg()..distance + 1).step_by(interval) {

            // check along y axis
            for n in (distance.neg()..distance + 1).step_by(interval) {

                let biome = nether.get_final_biome(i, 0, n).to_string();
                last_check = (i,n);

                //println!("{}, {} {}", seed, i, n);

                if found_biomes.contains(&biome) {
                        
                } else {
                    found_biomes.insert(biome);
                    //println!("{:?}", found_biomes);
                }
            }
        }

        attempts += 1;

        if found_biomes.len() == 5 {
            println!();
            println!("Found seed with all nether biomes within {} blocks", distance);
            //println!("{:?}", found_biomes);
            println!("{} seeds searched in {} seconds", attempts, now.elapsed().as_secs());
            println!("Seed: {}", seed);
            println!();
            found_seed = true;

            let mut number = String::new();
            io::stdin()
                .read_line(&mut number)
                .expect("Failed to read input");
            exit(0)
        }

    }
}
