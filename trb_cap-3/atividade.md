# 🦀 Exercícios Práticos - Capítulo 3 do The Rust Book

## 📋 Como Usar Este Guia

1. Leia o **Problema** de cada capítulo
2. Tente resolver sozinho criando um novo arquivo `.rs`
3. Só olhe o **Gabarito** depois de tentar
4. Execute com `rustc arquivo.rs && ./arquivo` ou use `cargo run`

---

## 3.1 - Variáveis e Mutabilidade

### 🎯 PROBLEMA: Sistema de Saldo Bancário

Você foi contratado para criar um sistema simples de conta bancária. O sistema precisa:

**Requisitos:**
1. Começar com um saldo inicial de R$ 1000,00
2. Adicionar um depósito de R$ 250,00
3. Realizar um saque de R$ 150,00
4. Usar **shadowing** para transformar o saldo final em uma mensagem formatada
5. Criar uma **constante** para a taxa de juros mensal (0.5%)
6. Imprimir todas as operações

**Regras Rust:**
- Pense em quando usar `let` vs `let mut`
- Use constantes para valores que nunca mudam
- Use shadowing quando quiser "transformar" um valor mantendo o mesmo nome

**Exemplo de saída esperada:**
```
Saldo inicial: R$ 1000.00
Após depósito: R$ 1250.00
Após saque: R$ 1100.00
Taxa de juros mensal: 0.5%
Status: Saldo disponível: R$ 1100.00
```

---

## 3.2 - Tipos de Dados

### 🎯 PROBLEMA: Ficha de Personagem RPG

Você está desenvolvendo um jogo RPG e precisa criar uma ficha de personagem. 

**Requisitos:**
1. Armazene informações do personagem usando tipos apropriados:
   - Nome (texto)
   - Nível (número inteiro positivo pequeno, max 255)
   - Pontos de vida (número inteiro, pode ser negativo em alguns casos)
   - Pontos de experiência (número inteiro grande)
   - Taxa de crítico (número decimal de precisão simples)
   - Está vivo? (booleano)
   - Classe do personagem (caractere: 'G' = Guerreiro, 'M' = Mago, 'A' = Arqueiro)

2. Crie uma **tupla** para armazenar a posição do personagem (x, y, z)

3. Crie um **array** com os 4 itens do inventário inicial

4. Calcule e imprima:
   - Todos os atributos do personagem
   - A posição atual (desestruturando a tupla)
   - Cada item do inventário
   - Se o personagem pode subir de nível (XP >= 1000)

**Dica:** Escolha os tipos mais apropriados (i8, u8, i32, u64, f32, etc.)

**Exemplo de saída esperada:**
```
=== Ficha do Personagem ===
Nome: Thorin
Nível: 5
HP: 100
XP: 1250
Crítico: 15.5%
Vivo: true
Classe: G (Guerreiro)

Posição: x=10, y=20, z=0

Inventário:
- Espada de Ferro
- Poção de Vida
- Escudo de Madeira
- Mapa

Pode subir de nível: Sim
```

---

## 3.3 - Funções

### 🎯 PROBLEMA: Calculadora de Salário

Você precisa criar um sistema de cálculo de salário líquido para uma empresa.

**Requisitos:**

1. Crie uma função `calcular_desconto_inss` que:
   - Recebe o salário bruto (f64)
   - Retorna o valor do desconto (f64)
   - Regra: 11% sobre o salário bruto

2. Crie uma função `calcular_desconto_ir` que:
   - Recebe o salário bruto (f64)
   - Retorna o valor do desconto (f64)
   - Regra: 
     - Até R$ 2000: isento (0%)
     - De R$ 2000 a R$ 4000: 7.5%
     - Acima de R$ 4000: 15%

3. Crie uma função `calcular_salario_liquido` que:
   - Recebe o salário bruto (f64)
   - Chama as duas funções acima
   - Retorna o salário líquido (salário - INSS - IR)

4. Crie uma função `exibir_contracheque` que:
   - Recebe nome do funcionário (&str) e salário bruto (f64)
   - Não retorna nada (apenas imprime)
   - Mostra todas as informações formatadas

5. Na `main`, teste com 3 funcionários diferentes

**Exemplo de saída esperada:**
```
=== Contracheque ===
Funcionário: João Silva
Salário Bruto: R$ 3500.00
(-) INSS (11%): R$ 385.00
(-) IR (7.5%): R$ 262.50
(=) Salário Líquido: R$ 2852.50
```

---

## 3.4 - Comentários

### 🎯 PROBLEMA: Documentação de Código Legado

Você recebeu um código sem documentação e precisa adicionar comentários apropriados.

**Requisitos:**

1. Pegue o código abaixo e adicione:
   - **Comentários de documentação** (///) para cada função pública
   - **Comentários de linha** (//) explicando lógica complexa
   - **Comentários de bloco** (/* */) no cabeçalho do arquivo

2. Documentação deve incluir:
   - O que a função faz
   - Parâmetros e seus significados
   - O que retorna
   - Um exemplo de uso
   - Possíveis casos especiais

**Código para documentar:**

```rust
const PRECO_BASE: f64 = 50.0;

fn calcular_preco_final(quantidade: u32, tem_desconto: bool, eh_cliente_vip: bool) -> f64 {
    let mut total = PRECO_BASE * quantidade as f64;
    
    if quantidade >= 10 {
        total *= 0.9;
    }
    
    if tem_desconto {
        total *= 0.95;
    }
    
    if eh_cliente_vip {
        total *= 0.85;
    }
    
    total
}

fn main() {
    let preco = calcular_preco_final(15, true, false);
    println!("Preço final: R$ {:.2}", preco);
}
```

**Sua tarefa:** Reescreva este código com TODA a documentação apropriada.

---

## 3.5 - Fluxo de Controle

### 🎯 PROBLEMA: Sistema de Avaliação de Desempenho

Você precisa criar um sistema que avalia o desempenho de vendedores.

**Requisitos:**

1. **Função `classificar_vendedor`**
   - Recebe o total de vendas do mês (i32)
   - Usa `if/else` para retornar a classificação:
     - Menos de 1000: "Iniciante"
     - 1000 a 4999: "Intermediário"
     - 5000 a 9999: "Avançado"
     - 10000 ou mais: "Expert"

2. **Função `calcular_bonus`**
   - Recebe vendas e classificação (&str)
   - Calcula o bônus baseado na classificação:
     - Iniciante: 2% das vendas
     - Intermediário: 5% das vendas
     - Avançado: 8% das vendas
     - Expert: 10% das vendas + R$ 500 fixo

3. **Função `relatorio_trimestral`**
   - Recebe um array com vendas dos 3 meses [mes1, mes2, mes3]
   - Usa `loop` para processar cada mês
   - Usa `while` para contar quantos meses foram acima de 5000
   - Usa `for` para calcular a média trimestral
   - Imprime relatório completo

4. **Função `fibonacci_vendas`**
   - Recebe número de metas (n: u32)
   - Gera os primeiros N números de Fibonacci como metas progressivas
   - Usa para criar metas de vendas (multiplica cada número por 1000)

5. **Na main**
   - Crie dados de 2 vendedores
   - Processe e exiba tudo

**Exemplo de saída esperada:**
```
=== Sistema de Avaliação de Vendedores ===

Vendedor: Maria Santos
Mês 1: 3500 - Classificação: Intermediário - Bônus: R$ 175.00
Mês 2: 6200 - Classificação: Avançado - Bônus: R$ 496.00
Mês 3: 12000 - Classificação: Expert - Bônus: R$ 1700.00

Relatório Trimestral:
- Total vendido: R$ 21700.00
- Média mensal: R$ 7233.33
- Meses acima de R$ 5000: 2
- Bônus total: R$ 2371.00

Metas progressivas Fibonacci (próximos 5 meses):
Mês 1: R$ 1000
Mês 2: R$ 1000
Mês 3: R$ 2000
Mês 4: R$ 3000
Mês 5: R$ 5000
```

---

## 🎓 DESAFIO INTEGRADOR - Jogo da Adivinhação

Combine TODOS os conceitos do capítulo 3!

### 🎯 PROBLEMA: Jogo Completo de Adivinhação

**Requisitos:**

1. **Configuração (Variáveis e Constantes)**
   - Número secreto: 42
   - Máximo de tentativas: 5
   - Pontuação inicial: 100
   - Taxa de penalidade por erro: 15 pontos

2. **Sistema de Pontuação (Tipos e Funções)**
   - Função que calcula pontos restantes baseado em tentativas
   - Função que classifica o resultado final:
     - 100 pontos: "Mestre"
     - 70-85: "Expert"
     - 40-55: "Bom"
     - 25: "Regular"
     - 10 ou menos: "Iniciante"

3. **Mecânica do Jogo (Fluxo de Controle)**
   - Use array com palpites pré-definidos: [25, 50, 38, 45, 42]
   - Loop principal do jogo
   - Para cada tentativa:
     - Verifique se acertou
     - Dê dicas: "Muito baixo!", "Muito alto!", "Quase lá!" (diferença < 5)
     - Atualize pontuação
     - Conte tentativas

4. **Relatório Final**
   - Mostrar se ganhou ou perdeu
   - Número de tentativas usadas
   - Pontuação final
   - Classificação do jogador

**Bônus:** Adicione comentários de documentação em todas as funções!

---

# 📝 GABARITOS

## ⚠️ Tente resolver sozinho primeiro!

<details>
<summary>🔓 Clique para ver o Gabarito 3.1 - Variáveis e Mutabilidade</summary>

```rust
// Gabarito 3.1 - Sistema de Saldo Bancário

const TAXA_JUROS: f64 = 0.005; // 0.5%

fn main() {
    // Saldo inicial - imutável
    let saldo = 1000.0;
    println!("Saldo inicial: R$ {:.2}", saldo);
    
    // Após depósito - usando shadowing
    let saldo = saldo + 250.0;
    println!("Após depósito: R$ {:.2}", saldo);
    
    // Após saque - usando shadowing novamente
    let saldo = saldo - 150.0;
    println!("Após saque: R$ {:.2}", saldo);
    
    println!("Taxa de juros mensal: {}%", TAXA_JUROS * 100.0);
    
    // Transformando em mensagem usando shadowing
    let saldo = format!("Saldo disponível: R$ {:.2}", saldo);
    println!("Status: {}", saldo);
}
```

**Conceitos aplicados:**
- `let` para variáveis imutáveis
- Shadowing para reutilizar nome
- `const` para valores fixos
- Shadowing pode mudar tipo (f64 → String)

</details>

<details>
<summary>🔓 Clique para ver o Gabarito 3.2 - Tipos de Dados</summary>

```rust
// Gabarito 3.2 - Ficha de Personagem RPG

fn main() {
    // Atributos do personagem com tipos apropriados
    let nome: &str = "Thorin";
    let nivel: u8 = 5;                    // 0-255, sem sinal
    let hp: i32 = 100;                    // pode ser negativo
    let xp: u64 = 1250;                   // números grandes
    let taxa_critico: f32 = 15.5;         // decimal simples
    let vivo: bool = true;
    let classe: char = 'G';               // 'G' = Guerreiro
    
    // Tupla para posição (x, y, z)
    let posicao: (i32, i32, i32) = (10, 20, 0);
    
    // Array de inventário (tamanho fixo)
    let inventario: [&str; 4] = [
        "Espada de Ferro",
        "Poção de Vida",
        "Escudo de Madeira",
        "Mapa"
    ];
    
    // Imprimir ficha
    println!("=== Ficha do Personagem ===");
    println!("Nome: {}", nome);
    println!("Nível: {}", nivel);
    println!("HP: {}", hp);
    println!("XP: {}", xp);
    println!("Crítico: {}%", taxa_critico);
    println!("Vivo: {}", vivo);
    println!("Classe: {} (Guerreiro)", classe);
    
    // Desestruturar tupla
    let (x, y, z) = posicao;
    println!("\nPosição: x={}, y={}, z={}", x, y, z);
    
    // Iterar array
    println!("\nInventário:");
    for item in &inventario {
        println!("- {}", item);
    }
    
    // Verificação usando if como expressão
    let pode_subir = if xp >= 1000 { "Sim" } else { "Não" };
    println!("\nPode subir de nível: {}", pode_subir);
}
```

**Conceitos aplicados:**
- Tipos específicos (u8, i32, u64, f32)
- Tuplas com tipos heterogêneos
- Arrays com tamanho fixo
- Desestruturação de tuplas
- If como expressão

</details>

<details>
<summary>🔓 Clique para ver o Gabarito 3.3 - Funções</summary>

```rust
// Gabarito 3.3 - Calculadora de Salário

fn calcular_desconto_inss(salario_bruto: f64) -> f64 {
    salario_bruto * 0.11
}

fn calcular_desconto_ir(salario_bruto: f64) -> f64 {
    if salario_bruto <= 2000.0 {
        0.0
    } else if salario_bruto <= 4000.0 {
        salario_bruto * 0.075
    } else {
        salario_bruto * 0.15
    }
}

fn calcular_salario_liquido(salario_bruto: f64) -> f64 {
    let inss = calcular_desconto_inss(salario_bruto);
    let ir = calcular_desconto_ir(salario_bruto);
    salario_bruto - inss - ir
}

fn exibir_contracheque(nome: &str, salario_bruto: f64) {
    let inss = calcular_desconto_inss(salario_bruto);
    let ir = calcular_desconto_ir(salario_bruto);
    let liquido = calcular_salario_liquido(salario_bruto);
    
    println!("=== Contracheque ===");
    println!("Funcionário: {}", nome);
    println!("Salário Bruto: R$ {:.2}", salario_bruto);
    println!("(-) INSS (11%): R$ {:.2}", inss);
    
    let percentual_ir = if salario_bruto <= 2000.0 {
        0.0
    } else if salario_bruto <= 4000.0 {
        7.5
    } else {
        15.0
    };
    println!("(-) IR ({}%): R$ {:.2}", percentual_ir, ir);
    println!("(=) Salário Líquido: R$ {:.2}", liquido);
    println!();
}

fn main() {
    exibir_contracheque("João Silva", 3500.0);
    exibir_contracheque("Maria Santos", 1800.0);
    exibir_contracheque("Pedro Costa", 5500.0);
}
```

**Conceitos aplicados:**
- Funções com parâmetros e retorno
- Funções que chamam outras funções
- Funções sem retorno (unit type `()`)
- Expressões como retorno (sem `;`)
- Parâmetros de diferentes tipos

</details>

<details>
<summary>🔓 Clique para ver o Gabarito 3.4 - Comentários</summary>

```rust
// Gabarito 3.4 - Documentação de Código Legado

/*
 * Sistema de Cálculo de Preços
 * 
 * Este módulo calcula o preço final de produtos
 * aplicando descontos baseados em quantidade,
 * promoções e status VIP do cliente.
 * 
 * Autor: Seu Nome
 * Data: 2024
 */

/// Preço base do produto em reais
const PRECO_BASE: f64 = 50.0;

/// Calcula o preço final de um produto aplicando descontos progressivos
///
/// # Argumentos
///
/// * `quantidade` - Quantidade de itens a comprar
/// * `tem_desconto` - Se há promoção ativa (true = 5% desconto)
/// * `eh_cliente_vip` - Se o cliente é VIP (true = 15% desconto adicional)
///
/// # Retorno
///
/// Retorna o valor total em reais (f64) após todos os descontos
///
/// # Descontos Aplicados
///
/// 1. Quantidade >= 10: 10% de desconto
/// 2. Promoção ativa: 5% de desconto adicional
/// 3. Cliente VIP: 15% de desconto adicional
///
/// Os descontos são cumulativos e aplicados em sequência.
///
/// # Exemplos
///
/// ```
/// // Cliente comum comprando 5 unidades sem promoção
/// let preco = calcular_preco_final(5, false, false);
/// assert_eq!(preco, 250.0);
///
/// // Cliente VIP comprando 15 unidades com promoção
/// let preco = calcular_preco_final(15, true, true);
/// // 750 * 0.9 * 0.95 * 0.85 = 544.69
/// ```
fn calcular_preco_final(quantidade: u32, tem_desconto: bool, eh_cliente_vip: bool) -> f64 {
    // Calcula o total base (preço unitário * quantidade)
    let mut total = PRECO_BASE * quantidade as f64;
    
    // Aplica desconto por quantidade (10% para 10+ itens)
    if quantidade >= 10 {
        total *= 0.9; // 10% de desconto
    }
    
    // Aplica desconto promocional se ativo
    if tem_desconto {
        total *= 0.95; // 5% de desconto
    }
    
    // Aplica desconto VIP (o maior benefício)
    if eh_cliente_vip {
        total *= 0.85; // 15% de desconto
    }
    
    total
}

fn main() {
    // Exemplo: 15 unidades, com promoção, cliente não-VIP
    let preco = calcular_preco_final(15, true, false);
    println!("Preço final: R$ {:.2}", preco);
}
```

**Conceitos aplicados:**
- Comentário de bloco `/* */` para cabeçalho
- Comentários de documentação `///` para funções
- Comentários de linha `//` para explicar lógica
- Seções organizadas (Argumentos, Retorno, Exemplos)
- Markdown na documentação

</details>

<details>
<summary>🔓 Clique para ver o Gabarito 3.5 - Fluxo de Controle</summary>

```rust
// Gabarito 3.5 - Sistema de Avaliação de Desempenho

fn classificar_vendedor(vendas: i32) -> &'static str {
    if vendas < 1000 {
        "Iniciante"
    } else if vendas < 5000 {
        "Intermediário"
    } else if vendas < 10000 {
        "Avançado"
    } else {
        "Expert"
    }
}

fn calcular_bonus(vendas: i32, classificacao: &str) -> f64 {
    let vendas_f64 = vendas as f64;
    
    match classificacao {
        "Iniciante" => vendas_f64 * 0.02,
        "Intermediário" => vendas_f64 * 0.05,
        "Avançado" => vendas_f64 * 0.08,
        "Expert" => vendas_f64 * 0.10 + 500.0,
        _ => 0.0,
    }
}

fn relatorio_trimestral(vendas_meses: [i32; 3], nome: &str) {
    println!("\nVendedor: {}", nome);
    
    let mut total_vendas = 0;
    let mut meses_acima_5000 = 0;
    let mut bonus_total = 0.0;
    let mut mes_numero = 1;
    
    // Loop para processar cada mês
    loop {
        if mes_numero > 3 {
            break;
        }
        
        let vendas = vendas_meses[mes_numero - 1];
        let classificacao = classificar_vendedor(vendas);
        let bonus = calcular_bonus(vendas, classificacao);
        
        println!("Mês {}: {} - Classificação: {} - Bônus: R$ {:.2}",
                 mes_numero, vendas, classificacao, bonus);
        
        total_vendas += vendas;
        bonus_total += bonus;
        
        mes_numero += 1;
    }
    
    // While para contar meses acima de 5000
    let mut i = 0;
    while i < 3 {
        if vendas_meses[i] > 5000 {
            meses_acima_5000 += 1;
        }
        i += 1;
    }
    
    // For para calcular média
    let mut soma = 0;
    for vendas in &vendas_meses {
        soma += vendas;
    }
    let media = soma as f64 / vendas_meses.len() as f64;
    
    println!("\nRelatório Trimestral:");
    println!("- Total vendido: R$ {}", total_vendas);
    println!("- Média mensal: R$ {:.2}", media);
    println!("- Meses acima de R$ 5000: {}", meses_acima_5000);
    println!("- Bônus total: R$ {:.2}", bonus_total);
}

fn fibonacci_vendas(n: u32) {
    println!("\nMetas progressivas Fibonacci (próximos {} meses):", n);
    
    let mut anterior = 1;
    let mut atual = 1;
    
    for mes in 1..=n {
        let meta = anterior * 1000;
        println!("Mês {}: R$ {}", mes, meta);
        
        let proximo = anterior + atual;
        anterior = atual;
        atual = proximo;
    }
}

fn main() {
    println!("=== Sistema de Avaliação de Vendedores ===");
    
    // Vendedor 1
    let vendas_maria = [3500, 6200, 12000];
    relatorio_trimestral(vendas_maria, "Maria Santos");
    
    // Vendedor 2
    let vendas_joao = [2500, 4800, 5200];
    relatorio_trimestral(vendas_joao, "João Silva");
    
    // Metas Fibonacci
    fibonacci_vendas(5);
}
```

**Conceitos aplicados:**
- `if/else` encadeado
- `loop` com `break`
- `while` para contagem
- `for` para iteração
- `match` para pattern matching
- Arrays e indexação

</details>

<details>
<summary>🔓 Clique para ver o Gabarito - DESAFIO INTEGRADOR</summary>

```rust
// Gabarito - Desafio Integrador: Jogo da Adivinhação Completo

/*
 * JOGO DA ADIVINHAÇÃO - Versão Completa
 * 
 * Um jogo onde o jogador tenta adivinhar um número secreto
 * em até 5 tentativas, com sistema de pontuação progressiva.
 */

/// Número secreto do jogo
const NUMERO_SECRETO: i32 = 42;

/// Máximo de tentativas permitidas
const MAX_TENTATIVAS: u32 = 5;

/// Pontuação inicial do jogador
const PONTUACAO_INICIAL: i32 = 100;

/// Penalidade em pontos por erro
const PENALIDADE: i32 = 15;

/// Calcula a pontuação restante baseado no número de erros
///
/// # Argumentos
///
/// * `tentativas_usadas` - Número de tentativas já realizadas
///
/// # Retorno
///
/// Pontuação atual (mínimo 0)
fn calcular_pontuacao(tentativas_usadas: u32) -> i32 {
    let erros = tentativas_usadas as i32;
    let pontos = PONTUACAO_INICIAL - (erros * PENALIDADE);
    
    if pontos < 0 {
        0
    } else {
        pontos
    }
}

/// Classifica o desempenho do jogador baseado na pontuação final
///
/// # Argumentos
///
/// * `pontuacao` - Pontuação final do jogador
///
/// # Retorno
///
/// String com a classificação
fn classificar_jogador(pontuacao: i32) -> &'static str {
    if pontuacao == 100 {
        "Mestre"
    } else if pontuacao >= 70 {
        "Expert"
    } else if pontuacao >= 40 {
        "Bom"
    } else if pontuacao >= 25 {
        "Regular"
    } else {
        "Iniciante"
    }
}

/// Verifica e dá dicas sobre o palpite
///
/// # Argumentos
///
/// * `palpite` - Número chutado pelo jogador
///
/// # Retorno
///
/// true se acertou, false se errou
fn verificar_palpite(palpite: i32) -> bool {
    if palpite == NUMERO_SECRETO {
        println!("🎉 PARABÉNS! Você acertou!");
        true
    } else {
        let diferenca = (palpite - NUMERO_SECRETO).abs();
        
        if diferenca < 5 {
            println!("🔥 Quase lá! Muito próximo!");
        } else if palpite < NUMERO_SECRETO {
            println!("📉 Muito baixo! Tente um número maior.");
        } else {
            println!("📈 Muito alto! Tente um número menor.");
        }
        
        false
    }
}

/// Exibe o relatório final do jogo
///
/// # Argumentos
///
/// * `ganhou` - Se o jogador venceu
/// * `tentativas` - Número de tentativas usadas
/// * `pontuacao` - Pontuação final
fn exibir_relatorio(ganhou: bool, tentativas: u32, pontuacao: i32) {
    println!("\n{'='*40}");
    println!("           RELATÓRIO FINAL");
    println!("{'='*40}");
    
    if ganhou {
        println!("✅ Resultado: VITÓRIA!");
    } else {
        println!("❌ Resultado: DERROTA");
        println!("   O número secreto era: {}", NUMERO_SECRETO);
    }
    
    println!("🎯 Tentativas usadas: {}/{}", tentativas, MAX_TENTATIVAS);
    println!("⭐ Pontuação final: {} pontos", pontuacao);
    
    let classificacao = classificar_jogador(pontuacao);
    println!("🏆 Classificação: {}", classificacao);
    
    println!("{'='*40}\n");
}

fn main() {
    println!("\n🎮 BEM-VINDO AO JOGO DA ADIVINHAÇÃO! 🎮\n");
    println!("Tente adivinhar o número entre 1 e 100");
    println!("Você tem {} tentativas!", MAX_TENTATIVAS);
    println!("Pontuação inicial: {} pontos", PONTUACAO_INICIAL);
    println!("Cada erro: -{} pontos\n",