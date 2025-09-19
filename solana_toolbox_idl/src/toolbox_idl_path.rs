use anyhow::Result;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlPath {
    pub parts: Vec<ToolboxIdlPathPart>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlPathPart {
    Empty,
    Index(usize),
    Key(String),
}

impl ToolboxIdlPath {
    pub fn try_parse(value: &str) -> Result<ToolboxIdlPath> {
        let mut parts = vec![];
        for part in value.split(".") {
            if part.is_empty() {
                parts.push(ToolboxIdlPathPart::Empty)
            } else if part.chars().all(|c| c.is_ascii_digit()) {
                parts.push(ToolboxIdlPathPart::Index(part.parse()?))
            } else {
                parts.push(ToolboxIdlPathPart::Key(part.to_string()))
            }
        }
        if parts[0] == ToolboxIdlPathPart::Empty {
            parts.remove(0);
        }
        Ok(ToolboxIdlPath { parts })
    }

    pub fn concat(&self, other: &ToolboxIdlPath) -> ToolboxIdlPath {
        ToolboxIdlPath {
            parts: self
                .parts
                .iter()
                .chain(other.parts.iter())
                .cloned()
                .collect::<Vec<_>>(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.parts.is_empty()
    }

    pub fn split_first(&self) -> Option<(ToolboxIdlPathPart, ToolboxIdlPath)> {
        if let Some((first, rest)) = self.parts.split_first() {
            return Some((
                first.clone(),
                ToolboxIdlPath {
                    parts: rest.to_vec(),
                },
            ));
        }
        None
    }

    pub fn value(&self) -> String {
        let mut parts = vec![];
        for part in &self.parts {
            parts.push(part.value());
        }
        parts.join(".")
    }
}

impl ToolboxIdlPathPart {
    pub fn key(&self) -> Option<&str> {
        match self {
            ToolboxIdlPathPart::Empty => None,
            ToolboxIdlPathPart::Key(key) => Some(key),
            ToolboxIdlPathPart::Index(_) => None,
        }
    }

    pub fn index(&self) -> Option<usize> {
        match self {
            ToolboxIdlPathPart::Empty => None,
            ToolboxIdlPathPart::Key(_) => None,
            ToolboxIdlPathPart::Index(index) => Some(*index),
        }
    }

    pub fn value(&self) -> String {
        match self {
            ToolboxIdlPathPart::Empty => "".to_string(),
            ToolboxIdlPathPart::Key(key) => key.to_string(),
            ToolboxIdlPathPart::Index(index) => index.to_string(),
        }
    }
}
