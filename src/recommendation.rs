use std::collections::{HashMap, HashSet, VecDeque};

use crate::graph::Graph;
use crate::models::{Customer, Product, Vertex};

#[derive(Debug, Default)]
pub struct Store {
    products: HashMap<u32, Product>,
    customers: HashMap<u32, Customer>,
    purchases: HashSet<(u32, u32)>,
    graph: Graph,
}

impl Store {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_product(&mut self, product: Product) -> Result<(), String> {
        if self.products.contains_key(&product.id) {
            return Err(format!("Produto {} já cadastrado.", product.id));
        }

        self.graph.add_vertex(Vertex::Product(product.id));
        self.products.insert(product.id, product);
        Ok(())
    }

    pub fn add_customer(&mut self, customer: Customer) -> Result<(), String> {
        if self.customers.contains_key(&customer.id) {
            return Err(format!("Cliente {} já cadastrado.", customer.id));
        }

        self.graph.add_vertex(Vertex::Customer(customer.id));
        self.customers.insert(customer.id, customer);
        Ok(())
    }

    pub fn get_product(&self, id: u32) -> Option<&Product> {
        self.products.get(&id)
    }

    pub fn get_customer(&self, id: u32) -> Option<&Customer> {
        self.customers.get(&id)
    }

    pub fn record_purchase(&mut self, customer_id: u32, product_id: u32) -> Result<(), String> {
        if !self.customers.contains_key(&customer_id) {
            return Err(format!("Cliente {} não encontrado.", customer_id));
        }

        if !self.products.contains_key(&product_id) {
            return Err(format!("Produto {} não encontrado.", product_id));
        }

        if !self.purchases.insert((customer_id, product_id)) {
            return Err("Esta compra já foi registrada.".to_string());
        }

        self.graph.add_edge(
            Vertex::Customer(customer_id),
            Vertex::Product(product_id),
            1,
        );

        Ok(())
    }

    /// Busca produtos a três arestas de distância:
    /// cliente -> produto -> outro cliente -> outro produto.
    ///
    /// O BFS visita cada vértice no máximo uma vez.
    /// Produtos já comprados pelo cliente não são recomendados.
    pub fn recommend_products(&self, customer_id: u32) -> Result<Vec<Product>, String> {
        if !self.customers.contains_key(&customer_id) {
            return Err(format!("Cliente {} não encontrado.", customer_id));
        }

        let start = Vertex::Customer(customer_id);
        let mut queue = VecDeque::new();
        let mut distances: HashMap<Vertex, usize> = HashMap::new();
        let mut candidates: HashSet<u32> = HashSet::new();

        queue.push_back(start.clone());
        distances.insert(start, 0);

        while let Some(vertex) = queue.pop_front() {
            let distance = distances[&vertex];

            if distance >= 3 {
                continue;
            }

            for (neighbor, _weight) in self.graph.neighbors(&vertex) {
                if distances.contains_key(neighbor) {
                    continue;
                }

                let next_distance = distance + 1;

                distances.insert(neighbor.clone(), next_distance);
                queue.push_back(neighbor.clone());

                if next_distance == 3 {
                    if let Vertex::Product(product_id) = neighbor {
                        candidates.insert(*product_id);
                    }
                }
            }
        }

        let mut recommendations: Vec<Product> = candidates
            .iter()
            .filter_map(|id| self.products.get(id).cloned())
            .collect();

        // Desempate determinístico para facilitar testes e demonstrações.
        recommendations.sort_by_key(|product| product.id);

        Ok(recommendations)
    }

    pub fn product_count(&self) -> usize {
        self.products.len()
    }

    pub fn customer_count(&self) -> usize {
        self.customers.len()
    }

    pub fn purchase_count(&self) -> usize {
        self.purchases.len()
    }

    pub fn graph_vertex_count(&self) -> usize {
        self.graph.vertex_count()
    }

    pub fn graph_edge_count(&self) -> usize {
        self.graph.edge_count()
    }
}
