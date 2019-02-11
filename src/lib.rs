pub mod trie {
    struct TrieNode {
        data: Option<char>,
        children: Vec<usize>,
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
                }],
            }
        }

        pub fn add(&mut self, st: &str) {
            // The current node we're looking at
            let mut index = 0;

            for ch in st.chars() {
                let mut found = false;

                if !self.nodes[index].children.is_empty() {
                    // Look at each child of the current node
                    for i in self.nodes[index].children.iter() {
                        // If we find the char we're looking for
                        if self.nodes[*i].data == Some(ch) {
                            // Change current node to that child
                            index = *i;
                            found = true;
                            break;
                        }
                    }
                }

                // If none of the children contain our char, create a new node
                if !found {
                    let new_index = self.nodes.len();
                    self.nodes[index].children.push(new_index);
                    self.nodes.push(TrieNode {
                        data: Some(ch),
                        children: Vec::new(),
                    });

                    // On next iteration, start at new node we just created
                    index = new_index;
                }
            }
        }

        pub fn contains(&self, st: &str) -> bool {
            let mut index = 0;

            for ch in st.chars() {
                if self.nodes[index].children.is_empty() {
                    return false;
                }

                let mut found = false;
                for i in self.nodes[index].children.iter() {
                    if self.nodes[*i].data == Some(ch) {
                        index = *i;
                        found = true;
                        break;
                    }
                }

                if !found {
                    return false;
                }
            }

            true
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

            let mut char = "h".chars();
            assert_eq!(trie.nodes[1].data, char.next());
            assert_eq!(trie.nodes[1].children.len(), 1);
            assert_eq!(trie.nodes[1].children[0], 2);

            trie.add("hell");
            assert_eq!(trie.nodes.len(), 6);

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
            assert_eq!(trie.contains("hello"), false);

            trie.add("hello");
            assert_eq!(trie.contains("hello"), true);
            assert_eq!(trie.contains("hell"), true);
            assert_eq!(trie.contains("help"), false);

            trie.add("help");
            assert_eq!(trie.contains("help"), true);
        }

    }

}
