// rns.rs

/// RNS Encoding: Breaks a value into residues based on a list of moduli.
pub fn rns_encode(value: u64, moduli: &[u64]) -> Vec<u64> {
    moduli.iter().map(|&m| value % m).collect()
}

/// RNS Addition: Adds two RNS-encoded numbers element-wise.
pub fn rns_add(a: &[u64], b: &[u64]) -> Vec<u64> {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x + y) % 255) // Example modulus; can be adjusted
        .collect()
}

/// RNS Multiplication: Multiplies two RNS-encoded numbers element-wise.
#[cfg(test)]  // Only compile for tests
pub fn rns_multiply(a: &[u64], b: &[u64], moduli: &[u64]) -> Vec<u64> {
    a.iter()
        .zip(b.iter())
        .zip(moduli.iter())
        .map(|((ai, bi), mod_val)| (ai * bi) % mod_val)
        .collect()
}
//  test cases

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rns_encode() {
        let moduli = vec![255, 256];
        let result = rns_encode(1234, &moduli);
        assert_eq!(result, vec![1234 % 255, 1234 % 256]);
    }

    #[test]
    fn test_rns_add() {
        let a = vec![100, 200];
        let b = vec![150, 50];
        let result = rns_add(&a, &b);
        assert_eq!(result, vec![(100 + 150) % 255, (200 + 50) % 255]);
    }

    #[test]
    fn test_rns_multiply() {
        let a = vec![2, 3];
        let b = vec![5, 7];
        let moduli = vec![7, 11];
        let result = rns_multiply(&a, &b, &moduli);
        // This checks that the multiplication is done correctly under modular arithmetic
        assert_eq!(result, vec![(2 * 5) % 7, (3 * 7) % 11]);
    }
}
