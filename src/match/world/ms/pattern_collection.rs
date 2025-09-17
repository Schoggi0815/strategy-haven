use crate::r#match::world::ms::pattern::Pattern;

pub struct PatternCollection {
    patterns: Vec<Pattern>,
}

impl PatternCollection {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    pub fn get(&self, pattern_id: usize) -> &Pattern {
        &self.patterns[pattern_id]
    }

    pub fn add_or_get(&mut self, pattern: Pattern) -> usize {
        if let Some((id, _)) = self
            .patterns
            .iter()
            .enumerate()
            .find(|(_, p)| **p == pattern)
        {
            id
        } else {
            let id = self.patterns.len();
            self.patterns.push(pattern);
            id
        }
    }

    pub fn get_all_ids(&self) -> impl Iterator<Item = usize> {
        (0..self.patterns.len()).into_iter()
    }
}
