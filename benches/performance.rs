use megastore::models::{Customer, Product};
use megastore::recommendation::Store;
use std::hint::black_box;
use std::time::{Duration, Instant};

const REPETICOES: usize = 5;

struct Medicoes {
    cadastro: Vec<Duration>,
    compras: Vec<Duration>,
    recomendacao: Vec<Duration>,
    quantidade_sugestoes: usize,
}

impl Medicoes {
    fn new() -> Self {
        Self {
            cadastro: Vec::new(),
            compras: Vec::new(),
            recomendacao: Vec::new(),
            quantidade_sugestoes: 0,
        }
    }
}

fn executar_uma_vez(
    total_clientes: u32,
    total_produtos: u32,
) -> (Duration, Duration, Duration, usize) {
    let mut store = Store::new();

    // 1. Mede o cadastro de clientes e produtos.
    let inicio = Instant::now();

    for id in 1..=total_clientes {
        store
            .add_customer(Customer::new(id, &format!("Cliente {}", id)))
            .expect("Falha ao cadastrar cliente");
    }

    for id in 1..=total_produtos {
        store
            .add_product(Product::new(
                id,
                &format!("Produto {}", id),
                "Categoria geral",
            ))
            .expect("Falha ao cadastrar produto");
    }

    let tempo_cadastro = inicio.elapsed();

    // 2. Mede o registro de compras.
    // Todos compram o produto 1; cada cliente a partir do
    // segundo também compra seu próprio produto.
    let inicio = Instant::now();

    for cliente_id in 1..=total_clientes {
        store
            .record_purchase(cliente_id, 1)
            .expect("Falha ao registrar compra");

        if cliente_id > 1 && cliente_id <= total_produtos {
            store
                .record_purchase(cliente_id, cliente_id)
                .expect("Falha ao registrar compra");
        }
    }

    let tempo_compras = inicio.elapsed();

    // 3. Mede a recomendação para o primeiro cliente.
    let inicio = Instant::now();

    let recomendacoes = store
        .recommend_products(1)
        .expect("Falha ao gerar recomendações");

    let quantidade_sugestoes = black_box(recomendacoes.len());
    black_box(&recomendacoes);

    let tempo_recomendacao = inicio.elapsed();

    (
        tempo_cadastro,
        tempo_compras,
        tempo_recomendacao,
        quantidade_sugestoes,
    )
}

fn media_ms(valores: &[Duration]) -> f64 {
    valores
        .iter()
        .map(|d| d.as_secs_f64() * 1000.0)
        .sum::<f64>()
        / valores.len() as f64
}

fn mediana_ms(valores: &[Duration]) -> f64 {
    let mut tempos: Vec<f64> = valores.iter().map(|d| d.as_secs_f64() * 1000.0).collect();

    tempos.sort_by(|a, b| a.total_cmp(b));

    let meio = tempos.len() / 2;

    if tempos.len() % 2 == 0 {
        (tempos[meio - 1] + tempos[meio]) / 2.0
    } else {
        tempos[meio]
    }
}

fn executar_benchmark(total_clientes: u32, total_produtos: u32) {
    let mut medicoes = Medicoes::new();

    for _ in 0..REPETICOES {
        let (cadastro, compras, recomendacao, sugestoes) =
            executar_uma_vez(total_clientes, total_produtos);

        medicoes.cadastro.push(cadastro);
        medicoes.compras.push(compras);
        medicoes.recomendacao.push(recomendacao);
        medicoes.quantidade_sugestoes = sugestoes;
    }

    println!(
        "{:>8} | {:>8} | {:>10.3} | {:>10.3} | {:>10.3} | {:>10.3} | {:>10.3} | {:>8}",
        total_clientes,
        total_produtos,
        media_ms(&medicoes.cadastro),
        mediana_ms(&medicoes.cadastro),
        media_ms(&medicoes.compras),
        media_ms(&medicoes.recomendacao),
        mediana_ms(&medicoes.recomendacao),
        medicoes.quantidade_sugestoes,
    );
}

fn main() {
    println!("=== Benchmark do ConectaStore ===");
    println!("Tempos em milissegundos; cada cenário executado 5 vezes.");
    println!(
        "{:>8} | {:>8} | {:>10} | {:>10} | {:>10} | {:>10} | {:>10} | {:>8}",
        "Clientes",
        "Produtos",
        "Cad. media",
        "Cad. mediana",
        "Compras",
        "Rec. media",
        "Rec. mediana",
        "Sugestoes",
    );

    for (clientes, produtos) in [(100, 100), (1_000, 1_000), (5_000, 5_000)] {
        executar_benchmark(clientes, produtos);
    }
}
