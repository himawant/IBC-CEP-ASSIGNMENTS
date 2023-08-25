use std::collections::HashMap;

// Define the SortByKey trait
trait SortByKey<K, V> {
    fn sort_by_key(&mut self);
}

// Implement the SortByKey trait for HashMap
impl<K, V> SortByKey<K, V> for HashMap<K, V>
where
    K: Ord + Clone + std::hash::Hash,
    V: Clone,
{
    fn sort_by_key(&mut self) {
        let mut sorted_keys: Vec<_> = self.keys().cloned().collect();
        sorted_keys.sort();

        let mut sorted_map = HashMap::new();

        for key in sorted_keys {
            let value = self.get(&key).unwrap().clone();
            sorted_map.insert(key, value);
        }

        *self = sorted_map;
    }
}

fn main() {
    // Create a new HashMap with some key-value pairs
    let mut my_map = HashMap::new();
    my_map.insert("apple", 3);
    my_map.insert("banana", 1);
    my_map.insert("cherry", 5);

    // Print the initial HashMap
    println!("Original HashMap: {:?}", my_map);

    // Sort the HashMap by key and print again
    my_map.sort_by_key();
    println!("Sorted HashMap: {:?}", my_map);
}
