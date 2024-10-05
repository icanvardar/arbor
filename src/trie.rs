use std::collections::HashMap;

#[derive(Debug)]
pub struct TrieNode {
    children: HashMap<char, Option<Box<TrieNode>>>,
    word_ends: bool,
}

impl Default for TrieNode {
    fn default() -> Self {
        Self {
            children: HashMap::new(),
            word_ends: false,
        }
    }
}

impl TrieNode {
    fn new() -> Self {
        Self::default()
    }
}

pub struct Trie {
    pub root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    pub fn insert(word: String, node: &mut TrieNode, position: usize) {
        if position == word.len() {
            node.word_ends = true;
            return;
        }

        let c = word.as_bytes()[position] as char;

        if !c.is_ascii_alphabetic() {
            panic!("Invalid ASCII character!");
        }

        let child = node
            .children
            .entry(c)
            .or_insert_with(|| Some(Box::new(TrieNode::new())));

        if let Some(child) = child {
            Self::insert(word, child.as_mut(), position + 1);
        }
    }

    pub fn search(word: String, node: &TrieNode, position: usize) -> bool {
        if position == word.len() {
            return node.word_ends;
        }

        let c = word.as_bytes()[position] as char;

        if !c.is_ascii_alphabetic() {
            panic!("Invalid ASCII character!");
        }

        match node.children.get(&c) {
            Some(child) => Self::search(word, child.as_deref().unwrap(), position + 1),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_initializes_trie_node() {
        let trie_node = TrieNode::new();

        assert_eq!(trie_node.children.len(), 0);
        assert_eq!(trie_node.word_ends, false);
    }

    #[test]
    fn it_initializes_trie() {
        let trie = Trie::new();

        assert_eq!(trie.root.children.len(), 0);
        assert_eq!(trie.root.word_ends, false);
    }

    #[test]
    fn it_inserts_word_to_trie() {
        let mut trie = Trie::new();
        let word = "test".to_string();

        Trie::insert(word.clone(), &mut trie.root, 0);

        let has_word = Trie::search(word.clone(), &trie.root, 0);

        assert!(has_word);
    }
}
