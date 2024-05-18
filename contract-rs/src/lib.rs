// Find all our documentation at https://docs.near.org
use near_sdk::{log, near, near_bindgen, collections::Vector};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    greeting: String,
    image_urls: Vector<String>,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            greeting: "Hello".to_string(),
            image_urls: Vector::new(b"i"),
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

    // Public method - adds a new image URL
    pub fn add_image_url(&mut self, image_url: String) {
        self.image_urls.push(&image_url);
        log!("Image URL added: {}", image_url);
    }

    // Public method - returns all image URLs
    pub fn get_image_urls(&self) -> Vec<String> {
        self.image_urls.to_vec()
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
    fn add_and_get_image_urls() {
        let mut contract = Contract::default();
        contract.add_image_url("https://example.com/image1.jpg".to_string());
        contract.add_image_url("https://example.com/image2.jpg".to_string());

        let image_urls = contract.get_image_urls();
        assert_eq!(image_urls.len(), 2);
        assert_eq!(image_urls[0], "https://example.com/image1.jpg");
        assert_eq!(image_urls[1], "https://example.com/image2.jpg");
    }
}
