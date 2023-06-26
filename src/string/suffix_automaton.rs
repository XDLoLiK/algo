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
        Self { last: ROOT, states }
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

    /// O(string.len()) time
    pub fn contains(&self, string: &str) -> bool {
        let mut cur = ROOT;

        for sym in string.chars() {
            if self.states[cur].next.contains_key(&sym) {
                return false;
            }

            cur = *self.states[cur].next.get(&sym).unwrap();
        }

        true
    }

    /// O(string.length) time
    ///
    /// O(string.length) memory
    pub fn add_string(&mut self, string: &str) {
        for sym in string.chars() {
            self.add_char(sym);
        }

        self.mark_terminal();
    }

    fn mark_terminal(&mut self) {
        let mut prev = self.last;

        while prev != NO_LINK {
            self.states[prev].is_terminal = true;
            prev = self.states[prev].link;
        }
    }

    /// O(1) time (amortized)
    fn add_char(&mut self, sym: char) {
        // Sc - new state
        let cur_state = self.new_state();
        // |Sc| = |S| + 1
        self.states[cur_state].length = self.states[self.last].length + 1;

        let mut prev = self.last;

        while prev != NO_LINK && !self.states[prev].next.contains_key(&sym) {
            // S -->(next, c) Sc
            // U_1 = link(S) -->(next, c) Sc ...
            // untill U_k -->(next, c) S_0
            *self.states[prev].next.get_mut(&sym).unwrap() = cur_state;
            prev = self.states[prev].link;
        }

        if prev == NO_LINK {
            // if there's no U_k that has edge towards S_0
            self.states[cur_state].link = ROOT;
        } else {
            // S_0 ~ next
            let next = self.states[prev].next[&sym];

            if self.states[prev].length + 1 == self.states[next].length {
                // if S_0 has already been longest(next) (class representative)
                self.states[cur_state].link = next;
            } else {
                // If there has alredy been a transition by symbol
                // but the existing state is not the longest in its class
                let cloned = self.new_state();
                self.states[cloned].length = self.states[prev].length + 1;
                self.states[cloned].next = self.states[next].next.clone();
                self.states[cloned].link = self.states[next].link;

                while prev != NO_LINK && self.states[prev].next[&sym] == next {
                    *self.states[prev].next.get_mut(&sym).unwrap() = cloned;
                    prev = self.states[prev].link;
                }

                self.states[next].link = cloned;
                self.states[cur_state].link = cloned;
            }
        }

        self.last = cur_state;
    }
}
