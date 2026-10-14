use megastore::models::{Customer, Product};
use megastore::recommendation::Store;

#[test]
fn recomenda_produtos_de_clientes_com_compras_em_comum() {
    let mut store = Store::new();

    store.add_customer(Customer::new(1, "Ana")).unwrap();
    store.add_customer(Customer::new(2, "Bruno")).unwrap();

    store
        .add_product(Product::new(101, "Teclado", "Eletrônicos"))
        .unwrap();
    store
        .add_product(Product::new(102, "Monitor", "Eletrônicos"))
        .unwrap();

    store.record_purchase(1, 101).unwrap();
    store.record_purchase(2, 101).unwrap();
    store.record_purchase(2, 102).unwrap();

    let recommendations = store.recommend_products(1).unwrap();

    assert_eq!(recommendations.len(), 1);
    assert_eq!(recommendations[0].id, 102);
    assert_eq!(recommendations[0].name, "Monitor");
}

#[test]
fn nao_recomenda_produto_ja_comprado() {
    let mut store = Store::new();

    store.add_customer(Customer::new(1, "Ana")).unwrap();
    store.add_customer(Customer::new(2, "Bruno")).unwrap();

    store
        .add_product(Product::new(101, "Teclado", "Eletrônicos"))
        .unwrap();

    store.record_purchase(1, 101).unwrap();
    store.record_purchase(2, 101).unwrap();

    let recommendations = store.recommend_products(1).unwrap();

    assert!(recommendations.is_empty());
}

#[test]
fn impede_registro_duplicado_da_mesma_compra() {
    let mut store = Store::new();

    store.add_customer(Customer::new(1, "Ana")).unwrap();
    store
        .add_product(Product::new(101, "Teclado", "Eletrônicos"))
        .unwrap();

    store.record_purchase(1, 101).unwrap();

    assert!(store.record_purchase(1, 101).is_err());
    assert_eq!(store.purchase_count(), 1);
    assert_eq!(store.graph_edge_count(), 1);
}
