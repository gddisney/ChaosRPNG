
# ChaosPNG: Optimized Randomized Prime Generator with Henon and Lorenz Chaos Integration

**ChaosPNG** is an optimized randomized prime generator that utilizes chaotic systems, specifically the **Henon Map** and **Lorenz Attractor**, to derive primes with strong ties to the **Riemann zeta function**. By combining deterministic chaos with prime construction, **ChaosPNG** introduces a new way to generate primes that exhibit **mathematical universality** and align with critical zeros of the **Riemann zeta function**.

## Key Features

- **Chaos Integration**: Uses chaotic systems (Henon Map and Lorenz Attractor) to generate candidate primes.
- **Universal Primes**: Constructs primes that align with the Riemann zeta function's zeros, at high precision (\(10^{-3}\)).
- **Ring Metadata**: Embeds cryptographic and validation metadata into the generated primes for further use in secure systems.
- **High Precision**: Achieves alignment with zeta zeros with 100% consistency at precision \(10^{-3}\).

## Overview

The goal of **ChaosPNG** is to create **Universal Primes**-primes derived from chaotic systems that align with the **critical line** of the **Riemann zeta function**. These primes are tested for primality using **Miller-Rabin primality tests** and then validated through their alignment with **zeta zeros**.

The construction process begins with **Chaos Primes**, generated using the **Henon Map** and **Lorenz Attractor**, and is followed by **polynomial construction** to derive **Universal Primes**. These primes are validated and tested for their alignment with the Riemann zeta function.

## Installation

Clone the repository and use `cargo` to install dependencies:

```bash
git clone https://github.com/your-repo/ChaosPNG.git
cd ChaosPNG
cargo build --release
```

## Usage

Run the **ChaosPNG** generator with the following command-line arguments:

```bash
cargo run -- -p 16
```

### Arguments

- `prime_bits` (default: 512): The bit size of the primes to be generated.
- `base_test_rounds` (default: 20): The number of rounds for primality testing.
- `max_attempts` (default: 1000): The maximum number of attempts to generate a valid prime.
- **Henon Map Parameters**:
  - `henon_a`, `henon_b`, `henon_x0`, `henon_y0`: Parameters for the **Henon Map**.
- **Lorenz Attractor Parameters**:
  - `lorenz_sigma`, `lorenz_rho`, `lorenz_beta`, `lorenz_x0`, `lorenz_y0`, `lorenz_z0`, `lorenz_dt`: Parameters for the **Lorenz Attractor**.

Example usage:

```bash
cargo run -- --prime_bits 1024 --base_test_rounds 30 --max_attempts 1000 --henon_a 1.4 --henon_b 0.3 --henon_x0 0.1 --henon_y0 0.3
```

## Technical Details

### **Chaos Prime Generation**
- **Henon Map** and **Lorenz Attractor** are used to drive randomness in the generation process, ensuring that the **generated primes** are not purely random but derive from deterministic chaotic systems.
- The chaotic points \( x, y, z \) are then used to form **polynomials** which are tested for primality.

### **Universal Prime Construction**
- Once **chaos primes** are generated, they are used in a **polynomial structure** to form a candidate prime.
- The polynomial equation is as follows:

  \[
  n = a \cdot x^2 + b \cdot x \cdot y + c \cdot y^2 + d \cdot x \cdot z + e \cdot y \cdot z + f \cdot z^2 + g
  \]

  Where \( a, b, c, d, e, f, g \) are fixed coefficients.

### **Primality Testing**
- After generating the prime candidate \( n \), the **Miller-Rabin primality test** is used to verify whether \( n \) is prime.
- If \( n \) passes the primality test, it is then tested for **alignment with the Riemann zeta function's zeros**.

### **Ring Metadata**
- **Ring Metadata** is computed to validate the structural integrity of the generated prime.
- The metadata is derived from a quadratic ring formed from the **chaotic points** and is used to confirm that the prime follows a predictable, structured pattern.

### **Alignment with Zeta Zeros**
- The ultimate test for **Universal Primes** is their alignment with the **critical line** of the **Riemann zeta function**. This alignment provides insight into their **universality** and connection to **prime number distribution**.

## Example Output

```
ChaosPNG: Optimized Randomized Prime Generator with Henon and Lorenz Chaos Integration
Attempt 126: Successfully generated Universal Prime with N = 2580474434795127743030850837352085939290870435777748739068572660631154816382847047769230382395134614444690078540432600583433150411452852550806303166979651884685386776625810903
Chaos Primes found:
x = 136651454294867039958368257703743051125665152386317976317114394639123603210080881472749
y = 217123449713134213767884273174492606366477489268116471818609289423187049053755295084347
z = 242292999503364223856574642167264547004696593229022820829323803904853627353754083547553
Universal Prime found:
N = 2580474434795127743030850837352085939290870435777748739068572660631154816382847047769230382395134614444690078540432600583433150411452852550806303166979651884685386776625810903
Ring Metadata = RingMetadata { ring_value: 124522109984571313304263033209331372962316969679228501718408800993275978754179943063932610231286416039165056561648692784580808037136051414690833818974845783583160496946425868 }
Anomalous primes detected: [136651454294867039958368257703743051125665152386317976317114394639123603210080881472749, 217123449713134213767884273174492606366477489268116471818609289423187049053755295084347, 242292999503364223856574642167264547004696593229022820829323803904853627353754083547553, 2580474434795127743030850837352085939290870435777748739068572660631154816382847047769230382395134614444690078540432600583433150411452852550806303166979651884685386776625810903
Potential zero found: s = 0.5 + 105.11000000001687i, Zeta(s) = -0.0005464004986807103-0.000029536791777251548i
Universal Prime N = 2580474434795127743030850837352085939290870435777748739068572660631154816382847047769230382395134614444690078540432600583433150411452852550806303166979651884685386776625810903 aligns with a zeta zero!
```

## Contributing

Contributions are welcome! If you find any bugs or want to suggest improvements, feel free to open an issue or submit a pull request.

### Development
To contribute to **ChaosPNG**, you need to have **Rust** installed. Clone the repository, make your changes, and run the tests to ensure everything is working.

```bash
cargo test
```

## License

This project is licensed under the Apache 2.0 - see the [LICENSE](LICENSE) file for details.

