pub fn nth(n: u32) -> u32 {
    get_prime(n)
}

fn get_prime(n: u32) -> u32 {
    let mut prime_vec: Vec<u32> = Vec::new();
    let mut number = 2;
    prime_vec.push(2);

    while n >= prime_vec.len() as u32 {
        let mut prime = true;
        for prime_number in prime_vec.iter() {
            if number % prime_number == 0 {
                prime = false;

                break;
            }
        }

        if prime {
            prime_vec.push(number);
        }
        number += 1;
    }

    prime_vec[n as usize]
}
