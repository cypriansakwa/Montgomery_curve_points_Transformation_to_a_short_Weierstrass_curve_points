use num_bigint::BigInt;

// Montgomery point structure
#[derive(Debug)]
struct MontgomeryPoint {
    x: BigInt,
    y: BigInt,
}

#[allow(dead_code)] // Suppress the warning for unused fields
#[derive(Debug)]
struct WeierstrassPoint {
    x: BigInt,
    y: BigInt,
}

// Define a prime modulus for modular arithmetic
const MODULUS: &str = "17";  // Replace with the actual modulus value as needed

// Function to perform modular arithmetic
fn modp(value: BigInt) -> BigInt {
    let modulus = BigInt::parse_bytes(MODULUS.as_bytes(), 10).unwrap();
    ((value % &modulus) + &modulus) % modulus
}

// Function to compute modular inverse using Fermat's Little Theorem
fn mod_inverse(value: &BigInt, modulus: &BigInt) -> BigInt {
    value.modpow(&(modulus - BigInt::from(2)), modulus)
}

// Function to convert Montgomery curve point to short Weierstrass curve point
fn montgomery_to_weierstrass(
    point: MontgomeryPoint,
    a_mont: BigInt,
    b_mont: BigInt,
) -> WeierstrassPoint {
    let modulus = BigInt::parse_bytes(MODULUS.as_bytes(), 10).unwrap();

    // Precompute constants
    let three = BigInt::from(3);

    // Compute Weierstrass parameters
    let b_mont_squared = modp(&b_mont * &b_mont);
    let b_mont_cubed = modp(&b_mont * &b_mont_squared);
    let three_b_mont_squared = modp(three.clone() * &b_mont_squared);

    let a_weierstrass = modp(
        (three.clone() - &a_mont.pow(2u32)) * mod_inverse(&three_b_mont_squared, &modulus),
    );
    let _b_weierstrass = modp(
        (&a_mont * &a_weierstrass) - modp(BigInt::from(27) * &b_mont_cubed),
    );

    // Convert Montgomery point to Weierstrass point coordinates
    let x_weierstrass = modp(
        (three.clone() * &point.x + &a_mont) * mod_inverse(&modp(three * &b_mont), &modulus),
    );
    let y_weierstrass = modp(&point.y * mod_inverse(&b_mont, &modulus));

    // Return the Weierstrass point
    WeierstrassPoint {
        x: x_weierstrass,
        y: y_weierstrass,
    }
}

fn main() {
    // Define the Montgomery curve parameters
    let a_mont = BigInt::from(3);
    let b_mont = BigInt::from(15);

    // Define a sample point on the Montgomery curve
    let montgomery_point = MontgomeryPoint {
        x: BigInt::from(5),
        y: BigInt::from(5),
    };

    // Convert the point to the Weierstrass form
    let weierstrass_point = montgomery_to_weierstrass(montgomery_point, a_mont, b_mont);

    // Print the result
    println!("Weierstrass Point: {:?}", weierstrass_point);
}
