use sha1::{Sha1, Digest};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <hash_to_check>", args[0]);
        std::process::exit(1);
    }
    
    let hash_to_check = &args[1];
    println!("Hash to check: {}", hash_to_check);

    let found_ip = Arc::new(Mutex::new(None));
    let hash_count = Arc::new(Mutex::new(0usize));
    
    let start_time = Instant::now();

    (0..256).into_par_iter().for_each(|i| {
        for j in 0..256 {
            for k in 0..256 {
                for l in 0..256 {
                    let ip = format!("{}.{}.{}.{}", i, j, k, l);
                    let mut hasher = Sha1::new();
                    hasher.update(ip.as_bytes());
                    let hash = hasher.finalize();
                    let hash_str = format!("{:x}", hash);

                    let mut found_ip_guard = found_ip.lock().unwrap();
                    if found_ip_guard.is_some() {
                        return; // Exit early if another thread has found the IP
                    }

                    if hash_str == *hash_to_check {
                        *found_ip_guard = Some(ip.clone());
                        println!("Found IP: {}", ip);
                        return; // Exit the thread
                    }
                    
                    let mut count = hash_count.lock().unwrap();
                    *count += 1;
                }
            }
        }
    });

    let elapsed_time = start_time.elapsed();
    let total_hashes = *hash_count.lock().unwrap();

    let result = found_ip.lock().unwrap().clone();
    if let Some(ip) = result {
        println!("Matching IP found: {}", ip);
    } else {
        println!("No matching IP found.");
    }

    println!("Total hashes checked: {}", total_hashes);
    println!("Time elapsed: {:?}", elapsed_time);
}
