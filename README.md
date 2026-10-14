# ConectaStore — Sistema de Recomendação de Produtos

## 1. Sobre o projeto

O ConectaStore é uma aplicação desenvolvida em Rust que utiliza grafos para representar relações entre clientes e produtos e recomendar produtos com base em compras compartilhadas.

O projeto foi desenvolvido para fins acadêmicos, com foco em estruturas de dados, algoritmos de busca, organização do código e avaliação de desempenho.

## 2. Funcionalidades

- Cadastro de clientes e produtos.
- Prevenção de cadastros duplicados por identificador.
- Registro de compras e prevenção de compras duplicadas.
- Representação das relações entre clientes e produtos por meio de um grafo não direcionado.
- Recomendação de produtos utilizando busca em largura (BFS).
- Testes unitários e de integração.
- Benchmark de cadastro, registro de compras e geração de recomendações.

## 3. Tecnologias e estruturas utilizadas

- **Rust:** linguagem de programação.
- **HashMap:** armazenamento de clientes, produtos e listas de adjacência do grafo.
- **HashSet:** controle de compras registradas e prevenção de duplicidades.
- **VecDeque:** fila utilizada na busca em largura (BFS).
- **Testes automatizados:** validação de funcionalidades e regras do sistema.

## 4. Estrutura do projeto

```text
megastore/
├── benches/
│   └── performance.rs
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── models.rs
│   ├── graph.rs
│   └── recommendation.rs
├── tests/
│   └── recommendation_test.rs
├── Cargo.toml
└── README.md
```

## 5. Como executar

É necessário ter o Rust e o Cargo instalados.

Para executar a aplicação:

```bash
cargo run
```

Para executar os testes automatizados:

```bash
cargo test
```

Para executar o benchmark:

```bash
cargo bench --bench performance
```

## 6. Estratégia de recomendação

O sistema representa clientes e produtos como vértices de tipos diferentes. Uma compra cria uma aresta entre o cliente e o produto.

A busca em largura percorre as relações do grafo a partir do cliente consultado. Produtos encontrados por meio de clientes que compartilham compras podem ser apresentados como recomendações.

A implementação limita a busca a uma distância de três arestas, correspondente ao caminho cliente → produto → cliente → produto.

## 7. Testes

Os testes automatizados verificam:

- Criação de conexões nos dois sentidos.
- Atualização de uma conexão existente sem duplicá-la.
- Recomendação de produtos comprados por clientes com compras em comum.
- Não recomendação de produtos já comprados pelo cliente.
- Prevenção do registro duplicado da mesma compra.

## 8. Benchmark de desempenho

O benchmark executa cinco repetições para cada cenário, com os seguintes volumes:

| Clientes | Produtos |
|---:|---:|
| 100 | 100 |
| 1.000 | 1.000 |
| 5.000 | 5.000 |

Na execução registrada, os tempos médios foram:

| Clientes | Cadastro | Registro de compras | Recomendação |
|---:|---:|---:|---:|
| 100 | 0,085 ms | 0,061 ms | 0,062 ms |
| 1.000 | 0,804 ms | 0,903 ms | 0,635 ms |
| 5.000 | 3,625 ms | 11,847 ms | 3,475 ms |

Os valores são resultados observados em uma execução específica e podem variar conforme o ambiente. O cenário utiliza um padrão controlado de compras compartilhadas e não representa necessariamente o comportamento de uma loja real.

## 9. Limitações

O projeto utiliza dados em memória, sem persistência em banco de dados. O benchmark avalia apenas os cenários programados, e a recomendação é calculada para um cliente por chamada.

## 10. Considerações finais

O ConectaStore demonstra a aplicação de grafos, tabelas hash e busca em largura em um problema de recomendação de produtos, com testes automatizados e medições de desempenho em diferentes volumes de dados.