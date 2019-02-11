struct TrieNode {
    data: Option<char>,
    children: Vec<usize>,
}

pub struct Trie {
    nodes: Vec<TrieNode>,
}

impl Trie {
    fn new() -> Trie {
        Trie {
            nodes: vec![TrieNode {
                data: None,
                children: Vec::new(),
            }],
        }
    }

    fn add(&mut self, st: &str) {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut trie = Trie::new();
        let st = String::from("hello");
        trie.add(&st);

        assert_eq!(trie.nodes.len(), 6);

        let mut char = "h".chars();
        assert_eq!(trie.nodes[1].data, char.next());
        assert_eq!(trie.nodes[1].children.len(), 1);
        assert_eq!(trie.nodes[1].children[0], 2);

        trie.add("help");
        assert_eq!(trie.nodes.len(), 7);
        assert_eq!(trie.nodes[3].children.len(), 2);
        assert_eq!(trie.nodes[3].children[1], 6);

        let mut char2 = "p".chars();
        assert_eq!(trie.nodes[6].data, char2.next());
    }
}
