extern crate rand;

use std::env;
use rand::Rng;

pub fn gen_rand (min: u32, max: u32) -> u32  {
    rand::thread_rng().gen_range(min, max+1)
}

fn fact2_extr (n: u32) -> (u32, u32) {
    let mut odd: u32 = n;
    let mut sup: u32 = 0;

    loop {
        if odd%2 != 0 { break; }
        
        sup = sup+1;
        odd = odd/2;
    }

    (sup, odd)
}

fn miller_witness (a: u32, n: u32) -> bool {
    let tup: (u32, u32) = fact2_extr(n-1);
    let mut rem: u32 = u32::pow(a, tup.1) % n;
    let mut sup: u32 = tup.0;
    let mut resp: bool = true;

    if rem == 1 || rem == n-1 {
        resp = false;
    }

    while sup > 1 {
        rem = u32::pow(rem, 2) % n;
        if rem == n-1 {
            resp = false;
        };
        
        sup = sup-1;
    }

    resp
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args[1].parse::<u32>().unwrap();
    let k = args[2].parse::<u32>().unwrap();

    let mut resp: bool = true;
    
    for _i in 1..k {
        let a: u32 = gen_rand(2, n-2);
        resp = resp && !miller_witness(a, n);
    }

    println!("{}", resp);
}