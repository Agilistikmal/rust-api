//! Value Objects for Flower domain

use serde::{Deserialize, Serialize};

use crate::domain::errors::DomainResult;
use crate::domain::flower::FlowerError;

/// Value object for flower name with domain-specific rules
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FlowerName(String);

impl FlowerName {
    /// Create a new FlowerName with domain-specific validation
    pub fn new(name: impl Into<String>) -> DomainResult<Self> {
        let name = name.into().trim().to_string();

        // Guard clauses for domain-specific validation
        Self::validate_name(&name)?;

        Ok(Self(name))
    }

    /// Domain-specific name validation
    fn validate_name(name: &str) -> DomainResult<()> {
        // Check for empty or whitespace-only names
        if name.is_empty() {
            return Err(FlowerError::invalid_name("Name cannot be empty"));
        }

        // Check name length
        if name.len() > 100 {
            return Err(FlowerError::invalid_name("Name cannot exceed 100 characters"));
        }

        // Ensure name contains at least one alphabetic character
        if !name.chars().any(|c| c.is_alphabetic()) {
            return Err(FlowerError::invalid_name("Name must contain at least one alphabetic character"));
        }

        Ok(())
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

/// Value object for flower color with domain-specific validation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FlowerColor(String);

impl FlowerColor {
    /// Create a new FlowerColor with domain-specific validation
    pub fn new(color: impl Into<String>) -> DomainResult<Self> {
        let color = color.into().trim().to_lowercase();

        // Guard clauses for domain-specific validation
        Self::validate_color(&color)?;

        Ok(Self(color))
    }

    /// Domain-specific color validation
    fn validate_color(color: &str) -> DomainResult<()> {
        // Check for empty or whitespace-only colors
        if color.is_empty() {
            return Err(FlowerError::invalid_color("Color cannot be empty"));
        }

        // Check color length
        if color.len() > 50 {
            return Err(FlowerError::invalid_color("Color cannot exceed 50 characters"));
        }

        // Ensure color contains at least one alphabetic character
        if !color.chars().any(|c| c.is_alphabetic()) {
            return Err(FlowerError::invalid_color("Color must contain at least one alphabetic character"));
        }

        Ok(())
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
    fn test_flower_name_special_chars() {
        let result = FlowerName::new("!!!!");
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

    #[test]
    fn test_flower_color_special_char() {
        let result = FlowerColor::new("!!!!");
        assert!(result.is_err());
    }
}
