// Find all our documentation at https://docs.near.org
use near_sdk::{log, near, near_bindgen, collections::Vector};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    greeting: String,
    names: Vector<String>,
    artist_names: Vector<String>,
    artist_descriptions: Vector<String>,
    dates_created: Vector<u64>,
    locations: Vector<String>,
    descriptions: Vector<String>,
    image_urls: Vector<String>,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            greeting: "Hello".to_string(),
            names: Vector::new(b"a"),
            artist_names: Vector::new(b"b"),
            artist_descriptions: Vector::new(b"c"),
            dates_created: Vector::new(b"d"),
            locations: Vector::new(b"e"),
            descriptions: Vector::new(b"f"),
            image_urls: Vector::new(b"g"),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // Public method - returns the greeting saved, defaulting to DEFAULT_GREETING
    pub fn get_greeting(&self) -> String {
        self.greeting.clone()
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn set_greeting(&mut self, greeting: String) {
        log!("Saving greeting: {}", greeting);
        self.greeting = greeting;
    }

    // Public method - adds a new artwork
    pub fn add_artwork(
        &mut self,
        name: String,
        artist_name: String,
        artist_description: String,
        date_created: u64,
        location: String,
        description: String,
        image_url: String,
    ) {
        self.names.push(&name);
        self.artist_names.push(&artist_name);
        self.artist_descriptions.push(&artist_description);
        self.dates_created.push(&date_created);
        self.locations.push(&location);
        self.descriptions.push(&description);
        self.image_urls.push(&image_url);
        log!("Artwork added: {}", name);
    }

    // Public method - returns all artworks
    pub fn get_artworks(&self) -> Vec<(String, String, String, u64, String, String, String)> {
        let mut artworks = Vec::new();
        for i in 0..self.names.len() {
            artworks.push((
                self.names.get(i).unwrap(),
                self.artist_names.get(i).unwrap(),
                self.artist_descriptions.get(i).unwrap(),
                self.dates_created.get(i).unwrap(),
                self.locations.get(i).unwrap(),
                self.descriptions.get(i).unwrap(),
                self.image_urls.get(i).unwrap(),
            ));
        }
        artworks
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(contract.get_greeting(), "Hello");
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(contract.get_greeting(), "howdy");
    }

    #[test]
    fn add_and_get_artworks() {
        let mut contract = Contract::default();
        contract.add_artwork(
            "Mona Lisa".to_string(),
            "Leonardo da Vinci".to_string(),
            "Italian Renaissance artist".to_string(),
            1503,
            "Louvre, Paris".to_string(),
            "A portrait of a woman with an enigmatic expression".to_string(),
            "https://example.com/mona_lisa.jpg".to_string(),
        );

        let artworks = contract.get_artworks();
        assert_eq!(artworks.len(), 1);
        assert_eq!(artworks[0].0, "Mona Lisa");
        assert_eq!(artworks[0].1, "Leonardo da Vinci");
        assert_eq!(artworks[0].2, "Italian Renaissance artist");
        assert_eq!(artworks[0].3, 1503);
        assert_eq!(artworks[0].4, "Louvre, Paris");
        assert_eq!(artworks[0].5, "A portrait of a woman with an enigmatic expression");
        assert_eq!(artworks[0].6, "https://example.com/mona_lisa.jpg");
    }
}
