#![no_std]
#![no_main]

use core::time::Duration;

use rand::RngCore;
use vexide::prelude::*;
use vexide_rand::SystemRng;

#[vexide::main]
async fn main(_: Peripherals) {
    let mut rng = SystemRng::new();
    loop {
        let random_number = rng.next_u64();
        println!("Random number: {random_number}");
        sleep(Duration::from_millis(60)).await;
    }
}
