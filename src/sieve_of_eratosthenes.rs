pub fn sieve_of_eratosthenes(n: usize) -> Vec<Option<usize>> {
    let mut table: Vec<_> = (2..(n + 1)).map(|m| Some(m)).collect();
    let mut primes = vec![];

    for i in 0..table.len() {
        if let Some(a) = table[i] {
            primes.push(Some(a));
            for j in i..table.len() {
                if let Some(b) = table[j] {
                    if i != j && b % a == 0 {
                        table[j] = None;
                    }
                }
            }
            table[i] = None;
        }
    }
    primes
}

#[cfg(test)]
mod test {
    #[test]
    fn test_sieve_of_eratosthenes() {
        assert_eq!(
            super::sieve_of_eratosthenes(13),
            vec![Some(2), Some(3), Some(5), Some(7), Some(11), Some(13)]
        )
    }
}
