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
    // Constructor to initialize the contract with predefined artworks
    #[init]
    pub fn new() -> Self {
        let mut contract = Self::default();

        contract.add_artwork(
            "Mona Lisa".to_string(),
            "Leonardo da Vinci".to_string(),
            "Leonardo da Vinci, an Italian polymath of the Renaissance, whose areas of interest included invention, painting, sculpting, architecture, science, music, mathematics, engineering, literature, anatomy, geology, astronomy, botany, writing, history, and cartography. He is widely considered one of the greatest painters of all time.".to_string(),
            1503,
            "Louvre Museum, Paris".to_string(),
            "The Mona Lisa is a half-length portrait painting by Italian artist Leonardo da Vinci. Considered an archetypal masterpiece of the Italian Renaissance, it has been described as 'the best known, the most visited, the most written about, the most sung about, the most parodied work of art in the world.'".to_string(),
            "https://upload.wikimedia.org/wikipedia/commons/thumb/e/ec/Mona_Lisa%2C_by_Leonardo_da_Vinci%2C_from_C2RMF_retouched.jpg/800px-Mona_Lisa%2C_by_Leonardo_da_Vinci%2C_from_C2RMF_retouched.jpg".to_string(),
        );
        contract.add_artwork(
            "The Scream".to_string(),
            "Edvard Munch".to_string(),
            "Edvard Munch was a Norwegian painter, whose best known work, The Scream, has become one of the most iconic images of world art. His childhood was overshadowed by illness, bereavement and the dread of inheriting a mental condition that ran in the family.".to_string(),
            1893,
            "National Gallery, Oslo".to_string(),
            "The Scream is the popular name given to a composition created by Norwegian artist Edvard Munch in 1893. The agonized face in the painting has become one of the most iconic images of art, symbolizing the anxiety of modern man.".to_string(),
            "https://upload.wikimedia.org/wikipedia/commons/thumb/c/c5/Edvard_Munch%2C_1893%2C_The_Scream%2C_oil%2C_tempera_and_pastel_on_cardboard%2C_91_x_73_cm%2C_National_Gallery_of_Norway.jpg/1024px-Edvard_Munch%2C_1893%2C_The_Scream%2C_oil%2C_tempera_and_pastel_on_cardboard%2C_91_x_73_cm%2C_National_Gallery_of_Norway.jpg".to_string(),
        );
        contract.add_artwork(
            "Starry Night".to_string(),
            "Vincent van Gogh".to_string(),
            "Vincent van Gogh was a Dutch post-impressionist painter who is among the most famous and influential figures in Western art. In just over a decade, he created about 2,100 artworks, including around 860 oil paintings, most of them in the last two years of his life.".to_string(),
            1889,
            "Museum of Modern Art, New York".to_string(),
            "The Starry Night is an oil on canvas painting by Dutch Post-Impressionist painter Vincent van Gogh. Painted in June 1889, it depicts the view from the east-facing window of his asylum room at Saint-Remy-de-Provence, just before sunrise, with the addition of an ideal village.".to_string(),
            "https://upload.wikimedia.org/wikipedia/commons/thumb/e/ea/Van_Gogh_-_Starry_Night_-_Google_Art_Project.jpg/1920px-Van_Gogh_-_Starry_Night_-_Google_Art_Project.jpg".to_string(),
        );
        contract.add_artwork(
            "The Last Supper".to_string(),
            "Leonardo da Vinci".to_string(),
            "Leonardo da Vinci, an Italian polymath of the Renaissance, whose areas of interest included invention, painting, sculpting, architecture, science, music, mathematics, engineering, literature, anatomy, geology, astronomy, botany, writing, history, and cartography. He is widely considered one of the greatest painters of all time.".to_string(),
            1498,
            "Convent of Santa Maria delle Grazie, Milan".to_string(),
            "The Last Supper is a late 15th-century mural painting by Italian artist Leonardo da Vinci housed by the refectory of the Convent of Santa Maria delle Grazie in Milan, Italy. It is one of the Western world's most recognizable paintings.".to_string(),
            "https://upload.wikimedia.org/wikipedia/commons/thumb/4/4b/%C3%9Altima_Cena_-_Da_Vinci_5.jpg/1920px-%C3%9Altima_Cena_-_Da_Vinci_5.jpg".to_string(),
        );
        contract.add_artwork(
            "Creation of Adam".to_string(),
            "Michelangelo".to_string(),
            "Michelangelo di Lodovico Buonarroti Simoni was an Italian sculptor, painter, architect, and poet of the High Renaissance. He exerted an unparalleled influence on the development of Western art.".to_string(),
            1512,
            "Sistine Chapel, Vatican City".to_string(),
            "The Creation of Adam is a fresco painting by Italian artist Michelangelo, which forms part of the Sistine Chapel's ceiling, painted c. 1508-1512. It illustrates the Biblical creation narrative from the Book of Genesis in which God gives life to Adam, the first man.".to_string(),
            "https://upload.wikimedia.org/wikipedia/commons/5/5b/Michelangelo_-_Creation_of_Adam_%28cropped%29.jpg".to_string(),
        );
        contract.add_artwork(
            "American Gothic".to_string(),
            "Grant Wood".to_string(),
            "Grant DeVolson Wood was an American painter best known for his paintings depicting the rural American Midwest, particularly American Gothic, which has become an iconic image of the 20th century.".to_string(),
            1930,
            "Art Institute of Chicago, Chicago".to_string(),
            "American Gothic is a painting by Grant Wood in the collection of the Art Institute of Chicago. Wood was inspired to paint what is now known as the American Gothic House in Eldon, Iowa, along with 'the kind of people I fancied should live in that house.'".to_string(),
            "https://upload.wikimedia.org/wikipedia/commons/thumb/c/cc/Grant_Wood_-_American_Gothic_-_Google_Art_Project.jpg/1024px-Grant_Wood_-_American_Gothic_-_Google_Art_Project.jpg".to_string(),
        );
        contract.add_artwork(
            "Water Lilies".to_string(),
            "Claude Monet".to_string(),
            "Claude Monet was a French painter, a founder of French Impressionist painting and the most consistent and prolific practitioner of the movement's philosophy of expressing one's perceptions before nature.".to_string(),
            1916,
            "Musee de l'Orangerie, Paris".to_string(),
            "Water Lilies is a series of approximately 250 oil paintings by French Impressionist Claude Monet. The paintings depict Monet's flower garden at his home in Giverny and were the main focus of his artistic production during the last thirty years of his life.".to_string(),
            "https://www.artic.edu/iiif/2/3c27b499-af56-f0d5-93b5-a7f2f1ad5813/full/1686,/0/default.jpg".to_string(),
        );
        contract.add_artwork(
            "Girl with a Pearl Earring".to_string(),
            "Johannes Vermeer".to_string(),
            "Johannes Vermeer was a Dutch Baroque Period painter who specialized in domestic interior scenes of middle-class life. He was relatively successful during his lifetime but his modest celebrity gave way to obscurity after his death.".to_string(),
            1665,
            "Mauritshuis, The Hague".to_string(),
            "Girl with a Pearl Earring is an oil painting by Dutch Golden Age painter Johannes Vermeer. It is a tronie of a girl wearing a headscarf and a pearl earring.".to_string(),
            "https://upload.wikimedia.org/wikipedia/commons/thumb/0/0f/1665_Girl_with_a_Pearl_Earring.jpg/1024px-1665_Girl_with_a_Pearl_Earring.jpg".to_string(),
        );
        contract.add_artwork(
            "Wanderer above the Sea of Fog".to_string(),
            "Caspar David Friedrich".to_string(),
            "Caspar David Friedrich was a 19th-century German Romantic landscape painter, generally considered the most important German artist of his generation. He is best known for his mid-period allegorical landscapes.".to_string(),
            1818,
            "Kunsthalle Hamburg, Hamburg".to_string(),
            "Wanderer above the Sea of Fog is a painting by German Romantic artist Caspar David Friedrich. It depicts a man standing on a rocky precipice with his back to the viewer, gazing out over a landscape covered in a thick sea of fog.".to_string(),
            "https://upload.wikimedia.org/wikipedia/commons/thumb/a/af/Caspar_David_Friedrich_-_Wanderer_above_the_Sea_of_Fog.jpeg/1024px-Caspar_David_Friedrich_-_Wanderer_above_the_Sea_of_Fog.jpeg".to_string(),
        );
        contract.add_artwork(
            "The Night Watch".to_string(),
            "Rembrandt van Rijn".to_string(),
            "Rembrandt Harmenszoon van Rijn was a Dutch Golden Age painter, printmaker, and draughtsman. An innovative and prolific master in three media, he is generally considered one of the greatest visual artists in the history of art and the most important in Dutch art history.".to_string(),
            1642,
            "Rijksmuseum, Amsterdam".to_string(),
            "The Night Watch is a 1642 painting by Rembrandt van Rijn. It is one of the most famous Dutch Golden Age paintings. The painting is notable for its colossal size, the dramatic use of light and shadow, and the perception of motion.".to_string(),
            "https://upload.wikimedia.org/wikipedia/commons/thumb/5/5a/The_Night_Watch_-_HD.jpg/1280px-The_Night_Watch_-_HD.jpg".to_string(),
        );
        contract.add_artwork(
            "The Great Wave off Kanagawa".to_string(),
            "Hokusai".to_string(),
            "Katsushika Hokusai was a Japanese ukiyo-e painter and printmaker of the Edo period. He is best known as the author of the woodblock print series Thirty-Six Views of Mount Fuji, which includes the internationally iconic print The Great Wave off Kanagawa.".to_string(),
            1831,
            "Various collections".to_string(),
            "The Great Wave off Kanagawa is a woodblock print by the Japanese ukiyo-e artist Hokusai. It was published sometime between 1829 and 1833 and is Hokusai's most famous work. The image depicts an enormous wave threatening boats off the coast of the prefecture of Kanagawa.".to_string(),
            "https://upload.wikimedia.org/wikipedia/commons/thumb/a/a5/Tsunami_by_hokusai_19th_century.jpg/1920px-Tsunami_by_hokusai_19th_century.jpg".to_string(),
        );
        contract.add_artwork(
            "Napoleon Crossing the Alps".to_string(),
            "Jacques-Louis David".to_string(),
            "Jacques-Louis David was a French painter in the Neoclassical style, considered to be the preeminent painter of the era. His cerebral brand of history painting marked a change in taste away from Rococo frivolity toward a classical austerity.".to_string(),
            1801,
            "Chateau de Malmaison, Rueil-Malmaison".to_string(),
            "Napoleon Crossing the Alps is the title given to five versions of an oil on canvas equestrian portrait of Napoleon Bonaparte painted by the French artist Jacques-Louis David between 1801 and 1805. It shows a strongly idealized view of the real crossing that Napoleon and his army made across the Alps through the Great St Bernard Pass in May 1800.".to_string(),
            "https://upload.wikimedia.org/wikipedia/commons/thumb/f/fd/David_-_Napoleon_crossing_the_Alps_-_Malmaison2.jpg/1024px-David_-_Napoleon_crossing_the_Alps_-_Malmaison2.jpg".to_string(),
        );
        contract.add_artwork(
            "The Garden of Earthly Delights".to_string(),
            "Hieronymus Bosch".to_string(),
            "Hieronymus Bosch was a Dutch/Netherlandish painter from Brabant. He is one of the most notable representatives of the Early Netherlandish painting school. His work is known for its fantastic illustrations of religious concepts and narratives.".to_string(),
            1505,
            "Museo del Prado, Madrid".to_string(),
            "The Garden of Earthly Delights is the modern title given to a triptych painted by the Early Netherlandish master Hieronymus Bosch. It dates from between 1490 and 1510, when Bosch was about 40 or 50 years old. It has been housed in the Museo del Prado in Madrid since 1939.".to_string(),
            "https://upload.wikimedia.org/wikipedia/commons/6/62/The_Garden_of_Earthly_Delights_by_Bosch_High_Resolution_2.jpg".to_string(),
        );
        contract.add_artwork(
            "Liberty Leading the People".to_string(),
            "Eugene Delacroix".to_string(),
            "Ferdinand Victor Eugene Delacroix was a French Romantic artist regarded from the outset of his career as the leader of the French Romantic school. Delacroix's use of expressive brushstrokes and his study of the optical effects of color profoundly shaped the work of the Impressionists.".to_string(),
            1830,
            "Louvre Museum, Paris".to_string(),
            "Liberty Leading the People is a painting by Eugene Delacroix commemorating the July Revolution of 1830 in France. A woman personifying Liberty leads the people forward over a barricade and the bodies of the fallen, holding the flag of the French Revolution in one hand and brandishing a bayonetted musket with the other.".to_string(),
            "https://upload.wikimedia.org/wikipedia/commons/0/02/La_Libert%C3%A9_guidant_le_peuple_-_Eug%C3%A8ne_Delacroix_-_Mus%C3%A9e_du_Louvre_Peintures_RF_129_-_apr%C3%A8s_restauration_2024.jpg".to_string(),
        );
        contract.add_artwork(
            "A Sunday Afternoon on the Island of La Grande Jatte".to_string(),
            "Georges Seurat".to_string(),
            "Georges-Pierre Seurat was a French post-Impressionist artist. He is best known for devising the painting techniques known as chromoluminarism and pointillism.".to_string(),
            1886,
            "Art Institute of Chicago, Chicago".to_string(),
            "A Sunday Afternoon on the Island of La Grande Jatte painted in 1884, is one of Georges Seurat's most famous works. Seurat's composition includes a number of Parisians at a park on the banks of the River Seine.".to_string(),
            "https://upload.wikimedia.org/wikipedia/commons/thumb/7/7d/A_Sunday_on_La_Grande_Jatte%2C_Georges_Seurat%2C_1884.jpg/1920px-A_Sunday_on_La_Grande_Jatte%2C_Georges_Seurat%2C_1884.jpg".to_string(),
        );
        contract.add_artwork(
            "Washington Crossing the Delaware".to_string(),
            "Emanuel Leutze".to_string(),
            "Emanuel Gottlieb Leutze was a German American history painter best known for his 1851 painting Washington Crossing the Delaware. His works are characterized by their dramatic, detailed, and historically accurate depictions of scenes from American history.".to_string(),
            1851,
            "Metropolitan Museum of Art, New York".to_string(),
            "Washington Crossing the Delaware is an 1851 oil-on-canvas painting by the German-American artist Emanuel Leutze. It commemorates General George Washington's crossing of the Delaware River with the Continental Army during the American Revolutionary War.".to_string(),
            "https://upload.wikimedia.org/wikipedia/commons/thumb/9/95/Washington_Crossing_the_Delaware_by_Emanuel_Leutze%2C_MMA-NYC%2C_1851.jpg/1920px-Washington_Crossing_the_Delaware_by_Emanuel_Leutze%2C_MMA-NYC%2C_1851.jpg".to_string(),
        );

        contract
    }

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

    #[test]
    fn test_new() {
        let contract = Contract::new();

        let artworks = contract.get_artworks();
        assert_eq!(artworks.len(), 14);

        // Check first artwork
        assert_eq!(artworks[0].0, "Mona Lisa");
        assert_eq!(artworks[0].1, "Leonardo da Vinci");
        // ... more checks for the first artwork

        // Check second artwork
        assert_eq!(artworks[1].0, "The Scream");
        assert_eq!(artworks[1].1, "Edvard Munch");
        // ... more checks for the second artwork
    }
}
