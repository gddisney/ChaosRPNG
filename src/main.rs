mod num_complex;
mod pmpt;
mod prime_shamir;
mod zeta_wells;
use zeta_wells::*;
use pmpt::*;
use num_complex::*;
use clap::Parser;
use num_bigint::{BigUint, RandBigInt};
use num_traits::{One, Zero};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

/// Arguments for ChaosPNG
#[derive(Parser, Debug)]
struct Args {
    /// Number of bits for prime generation
    #[arg(short, long, default_value_t = 512)]
    prime_bits: usize,

    /// Base test rounds for primality testing
    #[arg(short, long, default_value_t = 20)]
    base_test_rounds: usize,

    /// Maximum number of attempts
    #[arg(short, long, default_value_t = 1000)]
    max_attempts: usize,

    /// Henon Map parameters
    #[arg(long, default_value_t = 1.4)]
    henon_a: f64,
    #[arg(long, default_value_t = 0.3)]
    henon_b: f64,
    #[arg(long, default_value_t = 0.1)]
    henon_x0: f64,
    #[arg(long, default_value_t = 0.3)]
    henon_y0: f64,

    /// Lorenz Attractor parameters
    #[arg(long, default_value_t = 10.0)]
    lorenz_sigma: f64,
    #[arg(long, default_value_t = 28.0)]
    lorenz_rho: f64,
    #[arg(long, default_value_t = 2.6666666666666665)]
    lorenz_beta: f64,
    #[arg(long, default_value_t = 0.1)]
    lorenz_x0: f64,
    #[arg(long, default_value_t = 0.0)]
    lorenz_y0: f64,
    #[arg(long, default_value_t = 0.0)]
    lorenz_z0: f64,
    #[arg(long, default_value_t = 0.01)]
    lorenz_dt: f64,
}

/// Henon Map Struct
struct HenonMap {
    a: f64,
    b: f64,
    x: f64,
    y: f64,
}

impl HenonMap {
    fn new(a: f64, b: f64, x0: f64, y0: f64) -> Self {
        HenonMap { a, b, x: x0, y: y0 }
    }

    fn next(&mut self, modifier: f64) -> (f64, f64) {
        let adjusted_a = self.a + modifier % 0.1;
        let next_x = 1.0 - adjusted_a * self.x * self.x + self.y;
        let next_y = self.b * self.x;

        self.x = next_x;
        self.y = next_y;

        (self.x, self.y)
    }
}

/// Lorenz Attractor Struct
struct LorenzAttractor {
    sigma: f64,
    rho: f64,
    beta: f64,
    x: f64,
    y: f64,
    z: f64,
    dt: f64,
}

impl LorenzAttractor {
    fn new(sigma: f64, rho: f64, beta: f64, x0: f64, y0: f64, z0: f64, dt: f64) -> Self {
        LorenzAttractor {
            sigma,
            rho,
            beta,
            x: x0,
            y: y0,
            z: z0,
            dt,
        }
    }

    fn next(&mut self) -> (f64, f64, f64) {
        let dx = self.sigma * (self.y - self.x) * self.dt;
        let dy = (self.x * (self.rho - self.z) - self.y) * self.dt;
        let dz = (self.x * self.y - self.beta * self.z) * self.dt;

        self.x += dx;
        self.y += dy;
        self.z += dz;

        (self.x, self.y, self.z)
    }
}


/// Ring Metadata for ChaosCoin
#[derive(Debug, Clone)]
struct RingMetadata {
    ring_value: BigUint,
}

impl RingMetadata {
    /// Generate the quadratic ring metadata
    fn generate(
        public: &SpherePoint,
        substituted: &SpherePoint,
        modulus: &BigUint,
    ) -> Self {
        let ring_value = (public.x.clone() * substituted.x.clone()
            + public.y.clone() * substituted.y.clone()
            + public.z.clone() * substituted.z.clone())
            % modulus;

        RingMetadata { ring_value }
    }

    /// Validate the quadratic ring metadata
    fn validate(
        &self,
        public: &SpherePoint,
        substituted: &SpherePoint,
        modulus: &BigUint,
    ) -> bool {
        let computed_ring = (public.x.clone() * substituted.x.clone()
            + public.y.clone() * substituted.y.clone()
            + public.z.clone() * substituted.z.clone())
            % modulus;

        &computed_ring == &self.ring_value
    }
}

/// Generate a random large prime with specified bit size.
pub fn generate_large_prime(bits: usize, test_rounds: usize, rng: &mut ChaCha20Rng) -> BigUint {
   loop {
        let candidate = rng.gen_biguint(bits as u64) | BigUint::one();
        if is_probably_prime(&candidate, test_rounds) {
                return candidate;
        }
    }
}
/// Check if a number is probably prime using the Miller-Rabin test.
pub fn is_probably_prime(n: &BigUint, k: usize) -> bool {
    if *n <= BigUint::from(1u64) {
        return false;
    }
    if *n == BigUint::from(2u64) {
        return true;
    }
    if n % 2u64 == BigUint::zero() {
        return false;
    }

    let mut rng = ChaCha20Rng::from_entropy();
    let one = BigUint::one();
    let two = &one + &one;
    let n_minus_one = n - &one;
    let mut d = n_minus_one.clone();
    let mut s = 0;
    while &d % &two == BigUint::zero() {
        d /= &two;
        s += 1;
    }

    'outer: for _ in 0..k {
        let a = rng.gen_biguint_range(&two, n);
        let mut x = a.modpow(&d, n);
        if x == one || x == n_minus_one {
            continue;
        }
        for _ in 0..(s - 1) {
            x = x.modpow(&two, n);
            if x == n_minus_one {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

fn generate_chaos_primes(
    prime_bits: usize,
    base_test_rounds: usize,
    max_attempts: usize,
    henon_map: &mut HenonMap,
    lorenz_attractor: &mut LorenzAttractor,
    rng_prime: &mut ChaCha20Rng,
) -> Option<(BigUint, BigUint, BigUint, BigUint, RingMetadata)> {
    for attempt in 1..=max_attempts {
        let (lorenz_x, lorenz_y, lorenz_z) = lorenz_attractor.next();
        let (henon_x, henon_y) = henon_map.next(lorenz_z);

        let chaotic_bits = prime_bits + ((henon_x.abs() * 10.0).floor() as usize % 32);
        let dynamic_test_rounds = base_test_rounds
            + ((henon_y.abs() * 5.0).floor() as usize % 5);

        let x = generate_large_prime(chaotic_bits, dynamic_test_rounds, rng_prime);
        let y = generate_large_prime(chaotic_bits, dynamic_test_rounds, rng_prime);
        let z = generate_large_prime(chaotic_bits, dynamic_test_rounds, rng_prime);

        // Polynomial structure
        let a = BigUint::from(3u32);
        let b = BigUint::from(5u32);
        let c = BigUint::from(7u32);
        let d = BigUint::from(11u32);
        let e = BigUint::from(13u32);
        let f = BigUint::from(17u32);
        let g = BigUint::from(19u32);

        let n = &a * &x * &x
            + &b * &x * &y
            + &c * &y * &y
            + &d * &x * &z
            + &e * &y * &z
            + &f * &z * &z
            + &g;

        if is_probably_prime(&n, dynamic_test_rounds) {
            let public_point = SpherePoint::new(x.clone(), y.clone(), z.clone());
            let substituted_point = SpherePoint::new(
                x.clone() + BigUint::one(),
                y.clone() + BigUint::one(),
                z.clone() + BigUint::one(),
            );
            let ring_metadata = RingMetadata::generate(&public_point, &substituted_point, &n);

            println!(
                "Attempt {}: Successfully generated Universal Prime with N = {}",
                attempt, n
            );
            return Some((x, y, z, n, ring_metadata));
        } else {
            println!(
                "Attempt {}: N = {} is not prime, regenerating...",
                attempt, n
            );
        }
    }

    println!("Failed to generate Quantum Primes after {} attempts.", max_attempts);
    None
}

fn main() {
    let args = Args::parse();

    let prime_bits = args.prime_bits;
    let base_test_rounds = args.base_test_rounds;
    let max_attempts = args.max_attempts;

    let mut lorenz_attractor = LorenzAttractor::new(
        args.lorenz_sigma,
        args.lorenz_rho,
        args.lorenz_beta,
        args.lorenz_x0,
        args.lorenz_y0,
        args.lorenz_z0,
        args.lorenz_dt,
    );

    let mut henon_map = HenonMap::new(args.henon_a, args.henon_b, args.henon_x0, args.henon_y0);

    let mut rng_prime = ChaCha20Rng::from_entropy();

    println!("ChaosPNG: Prime Generator with Henon and Lorenz Chaos Integration");

    match generate_chaos_primes(
        prime_bits,
        base_test_rounds,
        max_attempts,
        &mut henon_map,
        &mut lorenz_attractor,
        &mut rng_prime,
    ) {
        Some((x, y, z, n, metadata)) => {
            println!("Chaos Primes found:");
            println!("x = {}", x);
            println!("y = {}", y);
            println!("z = {}", z);
            println!("Universal Prime found:");
            println!("N = {}", n);
            println!("Ring Metadata = {:?}", metadata);
            let primes = vec![x.clone(), y.clone(), z.clone(), n.clone()]; // Include the generated universal prime
            let chaotic_points = vec![
                SpherePoint::new(x.clone(), y.clone(), z.clone()), // Use the chaos primes
            ];

            // Detect anomalous primes
            let anomalous_primes = detect_anomalous_primes(primes, chaotic_points);

            // Print anomalous primes, if any
            if anomalous_primes.is_empty() {
                println!("No anomalous primes detected.");
            } else {
                println!("Anomalous primes detected: {:?}", anomalous_primes);
            }
            // Test against the Riemann Zeta function
            let result = test_universal_prime_against_zeta(&n, 3, 1e-3);
            if result {
                println!("Universal Prime N = {} aligns with a zeta zero!", n);
            } else {
                println!("Universal Prime N = {} does not align with a zeta zero.", n);
            }
        }
        None => {
            println!("No valid Universal Prime generated after {} attempts.", max_attempts);
        }
    }   
}

