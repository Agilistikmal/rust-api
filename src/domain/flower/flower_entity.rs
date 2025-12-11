//! Flower Entity

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::errors::DomainResult;
use crate::domain::shared::Entity;

use super::FlowerError;
use super::flower_vo::{FlowerColor, FlowerName};

/// Flower entity representing a flower in the domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flower {
    id: Uuid,
    name: FlowerName,
    color: FlowerColor,
    description: Option<String>,
    price: f64,
    stock: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Flower {
    /// Create a new Flower entity
    pub fn new(
        name: impl Into<String>,
        color: impl Into<String>,
        description: Option<String>,
        price: f64,
        stock: i32,
    ) -> DomainResult<Self> {
        let now = Utc::now();
        Ok(Self {
            id: Uuid::new_v4(),
            name: FlowerName::new(name)?,
            color: FlowerColor::new(color)?,
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
            name: FlowerName::new(name)?,
            color: FlowerColor::new(color)?,
            description,
            price,
            stock,
            created_at,
            updated_at,
        })
    }

    // Getters
    pub fn name(&self) -> &str {
        self.name.value()
    }

    pub fn color(&self) -> &str {
        self.color.value()
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

    // Setters with validation
    pub fn update_name(&mut self, name: impl Into<String>) -> DomainResult<()> {
        self.name = FlowerName::new(name)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn update_color(&mut self, color: impl Into<String>) -> DomainResult<()> {
        self.color = FlowerColor::new(color)?;
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
