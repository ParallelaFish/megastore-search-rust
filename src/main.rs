use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Produto {
    id: u32,
    nome: String,
    categoria: String,
    preco: f64,
}

fn main() {
    let mut catalogo = HashMap::new();

    // Simulando a indexação de produtos
    let p1 = Produto { id: 101, nome: "Smartphone".to_string(), categoria: "Eletrônicos".to_string(), preco: 2500.0 };
    catalogo.insert(p1.id, p1);

    println!("Sistema MegaStore Iniciado.");
    
    // Exemplo de busca O(1)
    let busca_id = 101;
    match catalogo.get(&busca_id) {
        Some(p) => println!("Produto Encontrado: {} - R${}", p.nome, p.preco),
        None => println!("Produto não encontrado."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_busca_sucesso() {
        let mut map = HashMap::new();
        map.insert(1, "Teste".to_string());
        assert_eq!(map.get(&1), Some(&"Teste".to_string()));
    }
}
