/// Names are sequences of characters, which are scalar values as defined by Unicode (Section 2.4).
/// Due to the limitations of the binary format,
/// the length of a name is bounded by the length of its UTF-8 encoding.
/// See https://webassembly.github.io/spec/core/syntax/values.html#names
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Name {
    value: String,
}

impl Name {
    /// Returns a byte slice of this `Name`’s contents.
    pub fn as_bytes(&self) -> &[u8] {
        self.value.as_bytes()
    }

    /// Returns the length of this `Name`, in bytes, not chars or graphemes.
    /// In other words, it may not be what a human considers the length of the name.
    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Returns true if this `Name` has a length of zero, false otherwise.
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

impl From<&str> for Name {
    fn from(name: &str) -> Self {
        Name {
            value: name.to_string(),
        }
    }
}

impl From<String> for Name {
    fn from(name: String) -> Self {
        Name { value: name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_from_str() {
        let content = "Hello, World!";
        let name = Name::from(content);

        assert_eq!(name.len(), content.len());
        assert_eq!(name.is_empty(), content.is_empty());
        assert_eq!(name.as_bytes(), content.as_bytes());
    }

    #[test]
    fn name_from_string() {
        let content = "Hello, World!";
        let name = Name::from(content.to_string());

        assert_eq!(name.len(), content.len());
        assert_eq!(name.is_empty(), content.is_empty());
        assert_eq!(name.as_bytes(), content.as_bytes());
    }
}
