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
    fn new_node(&mut self) -> usize {
        self.nodes.push(State::default());
        self.nodes.len() - 1
    }

    pub fn new() -> Self {
        let mut nodes = vec![State::default()];
        Self { nodes }
    }

    pub fn build(string: &str) -> Self {
        let mut ret = Trie::new();
        ret.add_string(string);
        ret
    }

    /// O(string.len()) time.
    pub fn add_string(&mut self, string: &str) {
        let mut vertex = ROOT;

        for c in string.chars() {
            if !self.nodes[vertex].to.contains_key(&c) {
                let new_node = self.new_node();
                self.nodes[vertex].to.insert(c, new_node);
            }

            vertex = *self.nodes[vertex].to.get(&c).unwrap();
        }

        self.nodes[vertex].is_terminal = true;
    }

    /// O(string.len()) time.
    pub fn contains(&self, string: &str) -> bool {
        let mut vertex = ROOT;

        for c in string.chars() {
            if !self.nodes[vertex].to.contains_key(&c) {
                return false;
            }

            vertex = *self.nodes[vertex].to.get(&c).unwrap();
        }

        self.nodes[vertex].is_terminal
    }
}
