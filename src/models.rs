use std::hash::Hash;

/// Identifica um vértice do grafo.
///
/// O mesmo número pode existir para um cliente e para um produto,
/// pois cada tipo possui seu próprio espaço de identificadores.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Vertex {
    Customer(u32),
    Product(u32),
}

/// Representa um produto do catálogo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub category: String,
}

impl Product {
    pub fn new(id: u32, name: &str, category: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            category: category.to_string(),
        }
    }
}

/// Representa um cliente da MegaStore.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Customer {
    pub id: u32,
    pub name: String,
}

impl Customer {
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }
}
