use std::collections::HashMap;

use crate::util::errors::TrieError;

#[derive(Default, Debug)]
pub struct TrieNode {
    children: HashMap<char, Option<Box<TrieNode>>>,
    word_ends: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self::default()
    }
}

#[derive(Default)]
pub struct Trie {
    pub root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    pub fn insert(word: String, node: &mut TrieNode, position: usize) -> Result<(), TrieError> {
        if position == word.len() {
            node.word_ends = true;
            return Ok(());
        }

        let c = word.as_bytes()[position] as char;

        if !c.is_ascii_alphabetic() {
            return Err(TrieError::InvalidCharacter);
        }

        let child = node
            .children
            .entry(c)
            .or_insert_with(|| Some(Box::new(TrieNode::new())));

        if let Some(child) = child {
            Self::insert(word, child.as_mut(), position + 1)?;
        }

        Ok(())
    }

    pub fn search(word: String, node: &TrieNode, position: usize) -> Result<bool, TrieError> {
        if position == word.len() {
            return Ok(node.word_ends);
        }

        let c = word.as_bytes()[position] as char;

        if !c.is_ascii_alphabetic() {
            return Err(TrieError::InvalidCharacter);
        }

        match node.children.get(&c) {
            Some(child) => Self::search(word, child.as_deref().unwrap(), position + 1),
            None => Ok(false),
        }
    }

    pub fn suggest(self, prefix: &str) -> Result<Vec<String>, TrieError> {
        let mut node = &self.root;
        let mut current_letters = String::new();

        for i in 0..prefix.len() {
            let letter = prefix.as_bytes()[i] as char;

            if let Some(child) = node.children.get(&letter) {
                node = child.as_deref().unwrap();
                current_letters.push(letter);
            } else {
                return Ok(Vec::new());
            }
        }

        let mut suggestion_list: Vec<String> = Vec::new();

        Self::consume_words(node, &mut suggestion_list, &mut current_letters);

        Ok(suggestion_list)
    }

    fn consume_words(node: &TrieNode, word_list: &mut Vec<String>, current_letters: &mut String) {
        if node.word_ends {
            word_list.push(current_letters.clone());
        }

        for (child_char, child_node) in &node.children {
            current_letters.push(*child_char);
            Self::consume_words(child_node.as_deref().unwrap(), word_list, current_letters);
            current_letters.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

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
    fn it_inserts_word_to_trie() -> Result<(), Box<dyn Error>> {
        let mut trie = Trie::new();
        let word = "test".to_string();

        Trie::insert(word.clone(), &mut trie.root, 0)?;

        let has_word = Trie::search(word.clone(), &trie.root, 0)?;

        assert!(has_word);

        Ok(())
    }

    #[test]
    fn it_suggests_words() -> Result<(), Box<dyn Error>> {
        let mut trie = Trie::new();
        let words = vec!["hello", "helicopter", "helium", "hall", "hundred"];
        let expected = vec![
            "hello".to_string(),
            "helicopter".to_string(),
            "helium".to_string(),
        ];

        for word in words {
            Trie::insert(word.to_string(), &mut trie.root, 0)?;
        }

        let result = trie.suggest("hel")?;

        for word in &result {
            assert!(expected.contains(word));
        }

        assert_eq!(result.len(), 3);

        Ok(())
    }

    #[test]
    fn it_inserts_word_with_non_ascii_character() -> Result<(), Box<dyn Error>> {
        let mut trie = Trie::new();
        let word = "~~~".to_string();

        assert_eq!(
            Trie::insert(word.clone(), &mut trie.root, 0).unwrap_err(),
            TrieError::InvalidCharacter
        );

        Ok(())
    }
}
