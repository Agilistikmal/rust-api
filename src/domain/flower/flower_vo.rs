//! Value Objects for Flower domain

use serde::{Deserialize, Serialize};

use crate::domain::errors::DomainResult;
use crate::domain::flower::FlowerError;

/// Value object for flower name
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FlowerName(String);

impl FlowerName {
    pub fn new(name: impl Into<String>) -> DomainResult<Self> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(FlowerError::invalid_name("name cannot be empty"));
        }
        if name.len() > 100 {
            return Err(FlowerError::invalid_name(
                "name cannot exceed 100 characters",
            ));
        }
        Ok(Self(name.trim().to_string()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<FlowerName> for String {
    fn from(name: FlowerName) -> Self {
        name.0
    }
}

/// Value object for flower color
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FlowerColor(String);

impl FlowerColor {
    pub fn new(color: impl Into<String>) -> DomainResult<Self> {
        let color = color.into();
        if color.trim().is_empty() {
            return Err(FlowerError::invalid_color("color cannot be empty"));
        }
        if color.len() > 50 {
            return Err(FlowerError::invalid_color(
                "color cannot exceed 50 characters",
            ));
        }
        Ok(Self(color.trim().to_lowercase()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<FlowerColor> for String {
    fn from(color: FlowerColor) -> Self {
        color.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flower_name_valid() {
        let name = FlowerName::new("Rose").unwrap();
        assert_eq!(name.value(), "Rose");
    }

    #[test]
    fn test_flower_name_empty() {
        let result = FlowerName::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_flower_color_valid() {
        let color = FlowerColor::new("Red").unwrap();
        assert_eq!(color.value(), "red");
    }

    #[test]
    fn test_flower_color_empty() {
        let result = FlowerColor::new("");
        assert!(result.is_err());
    }
}
