use num_bigint::BigUint;

/// alpha^x mod p
/// output = n^exponent mod modulus
pub fn exponentiate(n: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    n.modpow(exponent, modulus)
}
