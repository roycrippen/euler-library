// #![feature(plugin)]
//
// #![plugin(clippy)]

// return prime factors of a number
pub fn prime_factors(mut n: usize) -> Vec<usize> {
    let mut xs: Vec<usize> = Vec::new();
    let mut i = 2;
    while n > 1 {
        while n % i == 0 {
            xs.push(i);
            n /= i;
        }
        i += 1;
    }
    xs
}

pub fn prime_factors_unique(n: usize) -> Vec<usize> {
    let mut xs = prime_factors(n);
    xs.dedup();
    xs
}


// sum of unique prime factors
pub fn sopf(n: usize) -> usize { prime_factors_unique(n).iter().fold(0, |acc, x| acc + x) }


// return vector with count of the unique prime factors of i in 0..n - fast
pub fn prime_factor_cnt(n: usize) -> Vec<usize> {
    let mut s = vec![0 as usize; n];
    for (i, _) in s.clone().iter().enumerate().skip(2) {
        if s[i] == 0 {
            let mut j = i;
            while j < n {
                s[j] += 1;
                j += i
            }
        }
    }
    s
}
