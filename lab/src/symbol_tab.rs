use std::collections::HashMap;
use crate::types::Ty;

/// symbol table
#[derive(Debug, Clone)]
pub struct SymbolTab<'a> {
    /// a stack of tables
    tabs: Vec<HashMap<&'a str, Ty>>
}

impl<'a> SymbolTab<'a> {
    pub fn new() -> Self {
        Self {
            tabs: vec![HashMap::new()]
        }
    }

    pub fn push_scope(&mut self) {
        self.tabs.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.tabs.pop();
    }

    pub fn lookup<'b>(&mut self, ident: &'b str) -> Option<Ty> {
        for tab in self.tabs.iter().rev() {
            if let Some(ty) = tab.get(ident) {
                return Some(*ty);
            }
        }

        None
    }

    pub fn insert(&mut self, ident: &'a str, ty: Ty) {
        let len = self.tabs.len();
        self.tabs[len - 1].insert(ident, ty);
    }
}
