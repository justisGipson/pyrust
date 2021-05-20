use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use primal::is_prime;

/// sum_primes(n: int) -> int
/// --
/// Calculates the sum of primes from 0 to n and returns it as an integer

#[pyfunction]
fn sum_primes(n: i64) -> u64 {
  let mut sum: u64 = 0;
  let n = n as u64;
  for i in 0..n {
    // check if number exists in hashmap
    let is_prime = is_prime(i);
    if is_prime {
      sum += i;
    }
  }
  sum
}

#[pymodule]
fn pyrust(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
  m.add_function(wrap_pyfunction!(sum_primes, m)?)?;
  Ok(())
}

#[cfg(test)]
mod test {
  use super::{sum_primes, is_prime};

  #[test]
  fn test_is_prime_only_returns_true_for_prime_numbers() {
    assert_eq!(is_prime(2), true);
    assert_eq!(is_prime(3), true);
    assert_eq!(is_prime(4), false);
    assert_eq!(is_prime(5), true);
    assert_eq!(is_prime(6), false);
    assert_eq!(is_prime(7), true);
    assert_eq!(is_prime(8), false);
    assert_eq!(is_prime(9), false);
    assert_eq!(is_prime(11), true);
    assert_eq!(is_prime(31267), true);
  }

  #[test]
  fn test_sum_prime() {
    let s = 2 + 3 + 5 + 7 + 11;
    assert_eq!(sum_primes(12), s);
  }
}
