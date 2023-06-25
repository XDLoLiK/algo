use std::collections::HashMap;

static ROOT: usize = 0;
static NO_LINK: usize = usize::MAX;

#[derive(Debug, Default, Clone)]
struct State {
    length: usize,
    link: usize,
    next: HashMap<char, usize>,
    is_terminal: bool,
}

#[derive(Debug, Clone)]
pub struct SuffixAutomaton {
    last: usize,
    states: Vec<State>,
}

impl SuffixAutomaton {
    fn new_state(&mut self) -> usize {
        self.states.push(State::default());
        self.states.len() - 1
    }

    pub fn new() -> Self {
        let mut states = vec![State::default()];
        states[ROOT].link = NO_LINK;
        Self {
            last: ROOT,
            states,
        }
    }

    /// O(string.length) time
    pub fn build(string: &str) -> Self {
        let mut ret = SuffixAutomaton::new();
        ret.add_string(string);
        ret
    }

    /// O(1) time
    pub fn clear(&mut self) {
        self.states.clear();
        self.states.shrink_to_fit();
        self.last = self.new_state();
        self.states[ROOT].link = NO_LINK;
    }

    /// O(string.length) time
    ///
    /// O(string.length) memory
    pub fn add_string(&mut self, string: &str) {
        for sym in string.chars() {
            self.add_char(sym);
        }
    }

    /// O(1) time (amortized)
    pub fn add_char(&mut self, sym: char) {
        let cur_state = self.new_state();
        self.states[cur_state].length = self.states[self.last].length + 1;

        let mut prev = self.last;

        while prev != NO_LINK && !self.states[prev].next.contains_key(&sym) {
            self.states[prev].next[&sym] = cur_state;
            prev = self.states[prev].link;
        }

        if prev == NO_LINK {
            self.states[cur_state].link = ROOT;
        } else {
            let next = self.states[prev].next[&sym];

            if self.states[prev].length + 1 == self.states[next].length {
                self.states[cur_state].link = next;
            } else {
                let cloned = self.new_state();
                self.states[cloned].length = self.states[prev].length + 1;
                self.states[cloned].next = self.states[next].next;
                self.states[cloned].link = self.states[next].link;

                while prev != NO_LINK && self.states[prev].next[&sym] == next {
                    self.states[prev].next[&sym] = cloned;
                    prev = self.states[prev].link;
                }

                self.states[next].link = cloned;
                self.states[cur_state].link = cloned;
            }
        }

        self.last = cur_state;
    }
}
