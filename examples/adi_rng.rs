#![no_std]
#![no_main]

use core::time::Duration;

use rand::RngCore;
use vexide::prelude::*;
use veranda::AdiRng;

#[vexide::main]
async fn main(p: Peripherals) {
    let ports = &[p.adi_f, p.adi_g];
    let mut rng = AdiRng::new(ports);
    loop {
        let random_number = rng.next_u64();
        println!("Random number: {random_number}");
        sleep(Duration::from_millis(60)).await;
    }
}
