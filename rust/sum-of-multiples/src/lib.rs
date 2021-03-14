pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum: u32 = 0;

    for number in 1..limit {
        let mut multiples: u32 = 0;
        for factor in factors {
            if *factor != 0 && number % factor == 0 {
                multiples += 1;
            }
        }
        if multiples > 0 && multiples <= factors.len() as u32 {
            sum += number;
        }
    }
    sum
}
