pub struct Allergies {
    allergies: Vec<Allergen>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut allergies = Vec::new();
        let score_bits = format!("{:b}", score % 256);

        for (i, bit) in score_bits.chars().rev().enumerate() {
            if bit == '1' {
                match i {
                    0 => allergies.push(Allergen::Eggs),
                    1 => allergies.push(Allergen::Peanuts),
                    2 => allergies.push(Allergen::Shellfish),
                    3 => allergies.push(Allergen::Strawberries),
                    4 => allergies.push(Allergen::Tomatoes),
                    5 => allergies.push(Allergen::Chocolate),
                    6 => allergies.push(Allergen::Pollen),
                    7 => allergies.push(Allergen::Cats),
                    _ => panic!("What the fuck are you doing here???"),
                }
            }
        }

        Self {
            allergies
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies.iter().any(|allergy| allergy == allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies.clone()
    }
}
