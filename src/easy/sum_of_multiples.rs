pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut result = 0;
    let mut vec: Vec<u32> = Vec::new();

    factors.iter().for_each(|x: &u32| {
        if *x == 0 {
            return;
        }
        let mut i = 1;
        while i * x < limit {
            vec.push(i * x);
            i += 1;
        }
    });
    vec.sort();
    vec.dedup();
    vec.iter().for_each(|x: &u32| {
        result += x;
    });
    result
}
