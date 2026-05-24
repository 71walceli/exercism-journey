use indexmap::IndexSet;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct Allergies {
    allergy_set: IndexSet<Allergen>,
}

#[derive(Debug, PartialEq, Eq, EnumIter, Hash, Clone, Copy)]
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
        Allergies { 
            allergy_set: Allergen::iter().enumerate()
                .filter(|(i, _)| score >> i & 1 == 1)
                .map(|(_, e)| e)
                .collect()
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergy_set.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergy_set.iter().copied().collect()
    }
}
