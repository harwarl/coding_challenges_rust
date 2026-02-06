// left - 0
// right - 1
use crate::huffman::Tree;
use std::{collections::HashMap, hash::Hash};

pub fn generate_huffman_codes<T: Clone + Eq + Hash>(huffman_tree: Tree<T>) -> HashMap<T, String> {
    // keep track of the bits and store when a char is met
    let mut codes_char: HashMap<T, String> = HashMap::new();
    let mut codes = String::new();
    // Get the codes
    get_code::<T>(huffman_tree, &mut codes, &mut codes_char);
    codes_char
}

pub fn get_code<T: Clone + Eq + Hash>(
    node: Tree<T>,
    current_code: &mut String,
    codes_char: &mut HashMap<T, String>,
) {
    // If node is a leaf, update char
    // Otherwise, recursively update the current code for the left and right values
    match node {
        Tree::Leaf { char, .. } => {
            codes_char.insert(char.clone(), current_code.clone());
        }
        Tree::Node { left, right, .. } => {
            current_code.push('0');
            get_code(*left, current_code, codes_char);
            current_code.pop();

            current_code.push('1');
            get_code(*right, current_code, codes_char);
            current_code.pop();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::huffman::huffman_tree;

    #[test]
    fn test_codes() {
        // Generate a Tree
        let mut char_map: HashMap<char, u64> = HashMap::new();
        char_map.insert('e', 120);
        char_map.insert('u', 37);
        char_map.insert('d', 42);
        char_map.insert('l', 42);
        char_map.insert('c', 32);
        char_map.insert('z', 2);
        char_map.insert('k', 7);
        char_map.insert('m', 24);

        // Build a huffman tree
        let tree = huffman_tree(char_map);
        let mut codes_char: HashMap<char, String> = HashMap::new();
        let mut code = String::new();

        // Get the codes
        get_code(tree, &mut code, &mut codes_char);

        assert_eq!(
            codes_char.get(&'e').unwrap().to_string(),
            "0".to_string(),
            "Mismatched Values"
        );
        assert_eq!(
            codes_char.get(&'u').unwrap().to_string(),
            "100".to_string(),
            "Mismatched Values"
        );
        assert_eq!(
            codes_char.get(&'d').unwrap().to_string(),
            "101".to_string(),
            "Mismatched Values"
        );
        assert_eq!(
            codes_char.get(&'l').unwrap().to_string(),
            "110".to_string(),
            "Mismatched Values"
        );
        assert_eq!(
            codes_char.get(&'c').unwrap().to_string(),
            "1110".to_string(),
            "Mismatched Values"
        );
        assert_eq!(
            codes_char.get(&'z').unwrap().to_string(),
            "111100".to_string(),
            "Mismatched Values"
        );
        assert_eq!(
            codes_char.get(&'k').unwrap().to_string(),
            "111101".to_string(),
            "Mismatched Values"
        );
        assert_eq!(
            codes_char.get(&'m').unwrap().to_string(),
            "11111".to_string(),
            "Mismatched Values"
        );
    }
}
