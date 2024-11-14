#![no_std]

use core::hash::{BuildHasher, BuildHasherDefault, Hasher};

use ahash::AHasher;
use rand::{Error, RngCore};
use vex_sdk::{vexDeviceAdiValueGet, vexDeviceGetByIndex, vexSystemPowerupTimeGet};
use vexide_core::time::Instant;
use vexide_devices::{adi::AdiPort, battery};

/// A [`rand`](https://crates.io/crates/rand) RNG source that only uses system metrics for entropy.
/// This RNG source has a lower entropy than `AdiRng`, but does not require empty ADI ports.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SystemRng {
    time_of_creation: Instant,
}
impl SystemRng {
    /// Create a new `SystemRng`.
    ///
    /// # Examples
    ///
    /// ```
    /// use core::time::Duration;
    ///
    /// use rand::RngCore;
    /// use vexide::prelude::*;
    /// use vexide_rand::SystemRng;
    ///
    /// #[vexide::main]
    /// async fn main(_: Peripherals) {
    ///     let mut rng = SystemRng::new();
    ///     loop {
    ///         let random_number = rng.next_u64();
    ///         println!("Random number: {random_number}");
    ///         sleep(Duration::from_millis(60)).await;
    ///     }
    /// }
    /// ```
    pub fn new() -> SystemRng {
        SystemRng {
            time_of_creation: Instant::now(),
        }
    }

    fn hash_value(&self) -> u64 {
        let mut hasher = BuildHasherDefault::<AHasher>::default().build_hasher();
        hasher.write_u32((battery::voltage() * 1000.0) as _);
        hasher.write_u32((battery::current() * 1000.0) as _);
        hasher.write_u128(self.time_of_creation.elapsed().as_micros());
        hasher.write_u64(unsafe { vexSystemPowerupTimeGet() });
        hasher.finish()
    }
}
impl Default for SystemRng {
    fn default() -> Self {
        Self::new()
    }
}

impl RngCore for SystemRng {
    fn next_u32(&mut self) -> u32 {
        self.hash_value() as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.hash_value()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        dest.chunks_mut(4)
            .map(|chunk| {
                let len = chunk.len();
                let value = self.hash_value();
                chunk.copy_from_slice(&value.to_le_bytes()[..len]);
            })
            .count();
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

/// A [`rand`](https://crates.io/crates/rand) RNG source that includes empty ADI port(s) as a source of entropy.
/// It is incredibly important that the port is not connected to anything, as this will cause the RNG to be predictable.
#[derive(Debug, Eq, PartialEq)]
pub struct AdiRng<'a> {
    ports: &'a [AdiPort],
    time_of_creation: Instant,
}
impl AdiRng<'_> {
    /// Create a new `AdiRng` with the given ADI ports.
    /// Passing in multiple ports will increase the entropy of the RNG.
    ///
    /// # Examples
    ///
    /// ```
    /// use core::time::Duration;
    ///
    /// use rand::RngCore;
    /// use vexide::prelude::*;
    /// use vexide_rand::AdiRng;
    ///
    /// #[vexide::main]
    /// async fn main(p: Peripherals) {
    ///     let ports = &[p.adi_f, p.adi_g];
    ///     let mut rng = AdiRng::new(ports);
    ///     loop {
    ///         let random_number = rng.next_u64();
    ///         println!("Random number: {random_number}");
    ///         sleep(Duration::from_millis(60)).await;
    ///     }
    /// }
    /// ```
    pub fn new(ports: &[AdiPort]) -> AdiRng {
        AdiRng {
            ports,
            time_of_creation: Instant::now(),
        }
    }
    fn hash_value(&self) -> u64 {
        let values = self.ports.iter().map(|port| unsafe {
            vexDeviceAdiValueGet(
                vexDeviceGetByIndex(port.expander_number().unwrap_or(21) as _),
                port.number() as _,
            )
        });

        let mut hasher = BuildHasherDefault::<AHasher>::default().build_hasher();

        for value in values {
            hasher.write_i32(value);
        }
        hasher.write_u32((battery::voltage() * 1000.0) as _);
        hasher.write_u32((battery::current() * 1000.0) as _);
        hasher.write_u128(self.time_of_creation.elapsed().as_micros());
        hasher.write_u64(unsafe { vexSystemPowerupTimeGet() });

        hasher.finish()
    }
}

impl RngCore for AdiRng<'_> {
    fn next_u32(&mut self) -> u32 {
        self.hash_value() as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.hash_value()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        dest.chunks_mut(4)
            .map(|chunk| {
                let len = chunk.len();
                let value = self.hash_value();
                chunk.copy_from_slice(&value.to_le_bytes()[..len]);
            })
            .count();
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}
