use megastore::models::{Customer, Product};
use megastore::recommendation::Store;

fn main() -> Result<(), String> {
    let mut store = Store::new();

    // Cadastro dos clientes.
    store.add_customer(Customer::new(1, "Ana"))?;
    store.add_customer(Customer::new(2, "Bruno"))?;
    store.add_customer(Customer::new(3, "Carla"))?;

    // Cadastro do catálogo.
    store.add_product(Product::new(101, "Teclado", "Eletrônicos"))?;
    store.add_product(Product::new(102, "Mouse", "Eletrônicos"))?;
    store.add_product(Product::new(103, "Monitor", "Eletrônicos"))?;
    store.add_product(Product::new(104, "Webcam", "Eletrônicos"))?;

    // Histórico de compras.
    store.record_purchase(1, 101)?;
    store.record_purchase(1, 102)?;

    store.record_purchase(2, 101)?;
    store.record_purchase(2, 103)?;

    store.record_purchase(3, 102)?;
    store.record_purchase(3, 104)?;

    println!("=== ConectaStore ===");
    println!("Clientes: {}", store.customer_count());
    println!("Produtos: {}", store.product_count());
    println!("Compras registradas: {}", store.purchase_count());
    println!("Vértices do grafo: {}", store.graph_vertex_count());
    println!("Arestas do grafo: {}", store.graph_edge_count());

    let customer_id = 1;
    let customer = store
        .get_customer(customer_id)
        .ok_or("Cliente não encontrado.")?;

    println!("\nRecomendações para {}:", customer.name);

    let recommendations = store.recommend_products(customer_id)?;

    if recommendations.is_empty() {
        println!("Nenhum produto recomendado neste momento.");
    } else {
        for (index, product) in recommendations.iter().enumerate() {
            println!(
                "{}. {} - Categoria: {}",
                index + 1,
                product.name,
                product.category
            );
        }
    }

    Ok(())
}
