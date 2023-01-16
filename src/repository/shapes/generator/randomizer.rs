use rand::{rngs, Rng, SeedableRng};

pub struct RandomGenerator {
    generator: rngs::StdRng,
    amplitude: f64,
}

impl RandomGenerator {
    // xorshift
    pub fn generate(self: &mut Self) -> f64 {
        let rand: f64 = self.generator.gen();
        rand * self.amplitude
    }

    pub fn new(seed: u64, amplitude: f64) -> Self {
        RandomGenerator {
            generator: SeedableRng::seed_from_u64(seed),
            amplitude,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_random_generation() {
        let mut rng = RandomGenerator::new(100, 1.0);
        let actual = vec![
            rng.generate(),
            rng.generate(),
            rng.generate(),
            rng.generate(),
            rng.generate(),
        ];
        assert_eq!(
            vec![
                0.6066489624222408,
                0.6278977165779525,
                0.8861789426811084,
                0.8370359510382557,
                0.2982992708014165
            ],
            actual
        );
    }

    #[test]
    fn test_large_random_generation() {
        let mut rng = RandomGenerator::new(100, 100.0);
        let actual = vec![
            rng.generate(),
            rng.generate(),
            rng.generate(),
            rng.generate(),
            rng.generate(),
        ];
        assert_eq!(
            vec![
                60.664896242224074,
                62.78977165779524,
                88.61789426811085,
                83.70359510382556,
                29.82992708014165
            ],
            actual
        );
    }
}
