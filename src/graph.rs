use std::collections::HashMap;

use crate::models::Vertex;

/// Cada vizinho possui um vértice de destino e o peso da conexão.
pub type Edge = (Vertex, u32);

/// Grafo não direcionado representado por lista de adjacência.
#[derive(Debug, Default)]
pub struct Graph {
    adjacency: HashMap<Vertex, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adiciona um vértice sem duplicá-lo.
    pub fn add_vertex(&mut self, vertex: Vertex) {
        self.adjacency.entry(vertex).or_default();
    }

    /// Cria uma conexão nos dois sentidos.
    /// Se a conexão já existir, atualiza seu peso.
    pub fn add_edge(&mut self, from: Vertex, to: Vertex, weight: u32) {
        self.add_vertex(from.clone());
        self.add_vertex(to.clone());

        Self::set_edge(&mut self.adjacency, &from, to.clone(), weight);
        Self::set_edge(&mut self.adjacency, &to, from, weight);
    }

    fn set_edge(
        adjacency: &mut HashMap<Vertex, Vec<Edge>>,
        from: &Vertex,
        to: Vertex,
        weight: u32,
    ) {
        let neighbors = adjacency.entry(from.clone()).or_default();

        if let Some(edge) = neighbors.iter_mut().find(|edge| edge.0 == to) {
            edge.1 = weight;
        } else {
            neighbors.push((to, weight));
        }
    }

    /// Retorna os vizinhos e seus pesos.
    pub fn neighbors(&self, vertex: &Vertex) -> &[Edge] {
        self.adjacency.get(vertex).map(Vec::as_slice).unwrap_or(&[])
    }

    /// Retorna a quantidade de vértices cadastrados no grafo.
    pub fn vertex_count(&self) -> usize {
        self.adjacency.len()
    }

    /// Retorna a quantidade de conexões únicas.
    /// Cada aresta é armazenada nos dois sentidos.
    pub fn edge_count(&self) -> usize {
        self.adjacency.values().map(Vec::len).sum::<usize>() / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cria_conexao_nos_dois_sentidos() {
        let mut graph = Graph::new();
        let customer = Vertex::Customer(1);
        let product = Vertex::Product(10);

        graph.add_edge(customer.clone(), product.clone(), 1);

        assert_eq!(graph.vertex_count(), 2);
        assert_eq!(graph.edge_count(), 1);
        assert_eq!(graph.neighbors(&customer).len(), 1);
        assert_eq!(graph.neighbors(&product).len(), 1);
    }

    #[test]
    fn nao_duplica_conexao_existente() {
        let mut graph = Graph::new();
        let customer = Vertex::Customer(1);
        let product = Vertex::Product(10);

        graph.add_edge(customer.clone(), product.clone(), 1);
        graph.add_edge(customer.clone(), product.clone(), 3);

        assert_eq!(graph.edge_count(), 1);
        assert_eq!(graph.neighbors(&customer)[0].1, 3);
        assert_eq!(graph.neighbors(&product)[0].1, 3);
    }
}
