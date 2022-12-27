use std::collections::HashMap;

struct NFT {
    id: u64,
    name: String,
    data: HashMap<String, String>,
}

impl NFT {
    fn new(id: u64, name: String, data: HashMap<String, String>) -> Self {
        Self { id, name, data }
    }

    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_data(&self) -> &HashMap<String, String> {
        &self.data
    }
}

fn main() {
    let mut data = HashMap::new();
    data.insert("manufacturer".to_string(), "Acme Co.".to_string());
    data.insert("model".to_string(), "Widget 3000".to_string());
    data.insert("serial_number".to_string(), "1234ABCD".to_string());

    let nft = NFT::new(1, "Widget NFT".to_string(), data);

    println!("ID: {}", nft.get_id());
    println!("Name: {}", nft.get_name());
    println!("Data:");
    for (key, value) in nft.get_data() {
        println!("  {}: {}", key, value);
    }
}
