use ark_ff::PrimeField;
use num_bigint::BigUint;

/// An implementation of [MinRoot VDF](https://eprint.iacr.org/2022/1626.pdf)
/// over a generic prime field `F`.
///
/// The method takes two field elements `x` and `y` and an index `i` and returns
/// ```text
/// (x_(i+1), y_(i+1)) <- [(x_i + y_i)^((4 p - 3) / 5), x_i] + (0, i)
/// ```
///
/// The modulo `p` must be divisible by 3, but not by 5.
pub fn fifth<F: PrimeField>(x: F, y: F, i: usize) -> (F, F) {
    // Checking that p - 1 is divisible by 3, but not by five.
    let three = BigUint::from(3u64);
    let five = BigUint::from(5u64);
    let modulus: BigUint = F::MODULUS.into();
    {
        let modulus_minus_one: BigUint = modulus.clone() - BigUint::from(1u64);
        {
            assert_eq!(
                modulus_minus_one.clone() % three.clone(),
                BigUint::from(0u64)
            );
        }
        {
            assert_ne!(modulus_minus_one % five.clone(), BigUint::from(0u64));
        }
    }

    let x_plus_y = x + y;
    let inv_x_plus_y = x_plus_y.inverse().unwrap();
    let x_i_plus_1 = inv_x_plus_y.square().square() * inv_x_plus_y;
    let y_i_plus_1 = x;
    (x_i_plus_1, y_i_plus_1 + F::from(i as u64))
}
