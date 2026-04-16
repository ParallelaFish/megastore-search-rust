Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore
1. Descrição do Projeto
Este projeto consiste em um sistema de busca de alta performance desenvolvido para a "MegaStore". O objetivo é resolver problemas de lentidão e imprecisão no catálogo de produtos, utilizando estruturas de dados avançadas para garantir buscas rápidas e escalabilidade.

2. Tecnologias Utilizadas
Linguagem: Rust (Edição 2021)

Gerenciador de Pacotes: Cargo

Estrutura de Dados Principal: std::collections::HashMap

3. Arquitetura do Sistema
O sistema foi estruturado em módulos para garantir a separação de responsabilidades:

Modelo de Dados: Estrutura Produto que armazena ID, nome, categoria e preço.

Motor de Busca: Utiliza uma Tabela Hash (HashMap) para indexar os produtos pelo seu identificador único, permitindo acesso direto.

4. Algoritmos e Estruturas de Dados
A escolha da Tabela Hash (HashMap) foi estratégica para atender aos requisitos de desempenho da MegaStore:

Complexidade de Tempo: A busca por ID possui complexidade média de O(1), o que significa que o tempo de resposta é constante, independentemente de o catálogo ter mil ou milhões de itens.

Indexação: Ao carregar o catálogo, o sistema cria um índice em memória, eliminando a necessidade de percorrer listas lineares (O(n)).

5. Como Executar o Sistema
Certifique-se de ter o Rust instalado. No terminal, execute:
cargo run

6. Como Executar os Testes
Para validar a integridade das buscas e a funcionalidade do sistema:
cargo test

7. Exemplos de Uso
O sistema permite inserir produtos e buscá-los instantaneamente por ID. Caso o produto não exista, o sistema retorna uma mensagem amigável de erro, evitando falhas de processamento.

8. Considerações sobre Desempenho e Escalabilidade
Graças à segurança de memória do Rust e à eficiência da Tabela Hash, o sistema está preparado para o crescimento contínuo da MegaStore. A solução evita "gargalos" comuns em métodos tradicionais de busca.

9. Licença
Este projeto está sob a licença MIT.
