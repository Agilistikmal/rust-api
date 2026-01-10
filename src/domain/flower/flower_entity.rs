//! Flower Entity

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::errors::DomainResult;
use crate::domain::shared::Entity;

use crate::domain::flower::errors::FlowerError;

/// Flower entity representing a flower in the domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flower {
    id: Uuid,
    name: String,
    color: String,
    description: Option<String>,
    price: f64,
    stock: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Flower {
    /// Create a new Flower entity
    pub fn new(
        name: String,
        color: String,
        description: Option<String>,
        price: f64,
        stock: i32,
    ) -> DomainResult<Self> {
        let now = Utc::now();
        Ok(Self {
            id: Uuid::new_v4(),
            name,
            color,
            description,
            price,
            stock,
            created_at: now,
            updated_at: now,
        })
    }

    /// Reconstruct a Flower from persistence layer
    pub fn from_persistence(
        id: Uuid,
        name: String,
        color: String,
        description: Option<String>,
        price: f64,
        stock: i32,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> DomainResult<Self> {
        Ok(Self {
            id,
            name,
            color,
            description,
            price,
            stock,
            created_at,
            updated_at,
        })
    }

    // Getters
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn color(&self) -> &str {
        &self.color
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn stock(&self) -> i32 {
        self.stock
    }

    // Setters with basic validation
    pub fn update_name(&mut self, name: String) -> DomainResult<()> {
        if name.trim().is_empty() {
            return Err(FlowerError::invalid_name("Name cannot be empty"));
        }
        if name.len() > 100 {
            return Err(FlowerError::invalid_name("Name too long"));
        }
        self.name = name.trim().to_string();
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn update_color(&mut self, color: String) -> DomainResult<()> {
        if color.trim().is_empty() {
            return Err(FlowerError::invalid_color("Color cannot be empty"));
        }
        if color.len() > 50 {
            return Err(FlowerError::invalid_color("Color too long"));
        }
        self.color = color.trim().to_lowercase();
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn update_description(&mut self, description: Option<String>) {
        self.description = description;
        self.updated_at = Utc::now();
    }

    pub fn update_price(&mut self, price: f64) {
        self.price = price;
        self.updated_at = Utc::now();
    }

    pub fn update_stock(&mut self, stock: i32) {
        self.stock = stock;
        self.updated_at = Utc::now();
    }

    pub fn add_stock(&mut self, quantity: i32) {
        self.stock += quantity;
        self.updated_at = Utc::now();
    }

    pub fn reduce_stock(&mut self, quantity: i32) -> DomainResult<()> {
        if self.stock < quantity {
            return Err(FlowerError::insufficient_stock());
        }
        self.stock -= quantity;
        self.updated_at = Utc::now();
        Ok(())
    }
}

impl Entity for Flower {
    fn id(&self) -> Uuid {
        self.id
    }

    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
