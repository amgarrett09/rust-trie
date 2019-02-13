pub mod trie {
    struct TrieNode {
        data: Option<char>,
        children: Vec<usize>,
        valid_word: bool,
    }

    pub struct Trie {
        nodes: Vec<TrieNode>,
    }

    impl Default for Trie {
        fn default() -> Self {
            Trie::new()
        }
    }

    impl Trie {
        pub fn new() -> Self {
            Trie {
                nodes: vec![TrieNode {
                    data: None,
                    children: Vec::new(),
                    valid_word: false,
                }],
            }
        }

        pub fn add(&mut self, st: &str) {
            // The current node we're looking at
            let mut current = 0;

            for ch in st.chars() {
                let mut found = false;

                // Look at each child of the current node
                for child in self.nodes[current].children.iter() {
                    // If we find the char we're looking for
                    if self.nodes[*child].data == Some(ch) {
                        // Change current node to that child
                        current = *child;
                        found = true;
                        break;
                    }
                }

                // If none of the children contain our char, create a new node
                if !found {
                    let new_index = self.nodes.len();
                    self.nodes[current].children.push(new_index);
                    self.nodes.push(TrieNode {
                        data: Some(ch),
                        children: Vec::new(),
                        valid_word: false,
                    });

                    // On next iteration, start at new node we just created
                    current = new_index;
                }
            }

            // Set last node to be a valid endpoint for a keyword
            self.nodes[current].valid_word = true;
        }

        pub fn contains(&self, st: &str) -> bool {
            let mut current = 0;

            for ch in st.chars() {
                if self.nodes[current].children.is_empty() {
                    return false;
                }

                let mut found = false;
                for child in self.nodes[current].children.iter() {
                    if self.nodes[*child].data == Some(ch) {
                        current = *child;
                        found = true;
                        break;
                    }
                }

                if !found {
                    return false;
                }
            }

            // Return true if the last char we found is a valid word endpoint
            self.nodes[current].valid_word
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::trie::Trie;

        #[test]
        fn test_add() {
            let mut trie = Trie::new();
            trie.add("hello");

            assert_eq!(trie.nodes.len(), 6);
            assert!(trie.nodes[5].valid_word);
            assert!(!trie.nodes[4].valid_word);

            let mut char = "h".chars();
            assert_eq!(trie.nodes[1].data, char.next());
            assert_eq!(trie.nodes[1].children.len(), 1);
            assert_eq!(trie.nodes[1].children[0], 2);

            trie.add("hell");
            assert_eq!(trie.nodes.len(), 6);
            assert!(trie.nodes[4].valid_word);

            trie.add("help");
            assert_eq!(trie.nodes.len(), 7);
            assert_eq!(trie.nodes[3].children.len(), 2);
            assert_eq!(trie.nodes[3].children[1], 6);

            let mut char2 = "p".chars();
            assert_eq!(trie.nodes[6].data, char2.next());

            trie.add("cat");

            assert_eq!(trie.nodes.len(), 10);

            let mut char = "c".chars();
            assert_eq!(trie.nodes[7].data, char.next());
            assert_eq!(trie.nodes[7].children.len(), 1);
            assert_eq!(trie.nodes[7].children[0], 8);
            assert_eq!(trie.nodes[9].children.len(), 0);

            trie.add("cap");
            assert_eq!(trie.nodes.len(), 11);
            assert_eq!(trie.nodes[8].children.len(), 2);
            assert_eq!(trie.nodes[8].children[1], 10);
        }

        #[test]
        fn test_contains() {
            let mut trie = Trie::new();
            assert!(!trie.contains("hello"));

            trie.add("hello");
            assert!(trie.contains("hello"));
            assert!(!trie.contains("hell"));
            assert!(!trie.contains("help"));

            trie.add("help");
            assert_eq!(trie.contains("help"), true);
        }

    }

}
