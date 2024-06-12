use rand::Rng;

pub fn add_rand(x: u32) -> u32 {
    let random_number: u32 = rand::thread_rng().gen_range(1..=100);

    x + random_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = 6;

        assert!(input < add_rand(input));
    }
}
