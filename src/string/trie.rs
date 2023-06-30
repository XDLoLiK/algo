use std::collections::HashMap;

static ROOT: usize = 0;

#[derive(Debug, Default, Clone)]
struct State {
    to: HashMap<char, usize>,
    is_terminal: bool,
}

#[derive(Debug, Default, Clone)]
pub struct Trie {
    nodes: Vec<State>,
}

impl Trie {
    pub fn new() -> Self {
        let nodes = vec![State::default()];

        Self { nodes }
    }

    pub fn build(string: &str) -> Self {
        let mut ret = Trie::new();
        ret.add_string(string);

        ret
    }

    fn new_node(&mut self) -> usize {
        self.nodes.push(State::default());
        self.nodes.len() - 1
    }

    /// O(n) time
    pub fn add_string(&mut self, string: &str) {
        let mut vertex = ROOT;

        for c in string.chars() {
            match self.nodes[vertex].to.get(&c) {
                None => {
                    let new_node = self.new_node();
                    self.nodes[vertex].to.insert(c, new_node);
                }
                Some(_) => (),
            }

            vertex = self.nodes[vertex].to[&c];
        }

        self.nodes[vertex].is_terminal = true;
    }

    /// O(n) time
    pub fn contains(&self, string: &str) -> bool {
        let mut vertex = ROOT;

        for c in string.chars() {
            if !self.nodes[vertex].to.contains_key(&c) {
                return false;
            }

            vertex = self.nodes[vertex].to[&c];
        }

        self.nodes[vertex].is_terminal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trie_unit_1() {
        let mut trie = Trie::new();
        let strings = ["the", "a", "there", "answer", "any", "by", "bye", "their"];

        for string in strings {
            trie.add_string(string);
        }

        assert_eq!(trie.contains("the"), true);
        assert_eq!(trie.contains("these"), false);
        assert_eq!(trie.contains("their"), true);
        assert_eq!(trie.contains("thaw"), false);
    }
}
