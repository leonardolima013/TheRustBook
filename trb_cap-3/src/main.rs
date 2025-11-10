fn main() {

    println!("ATIVIDADE 3.1");
    let juros: f32 = 0.5;
    let saldo: f32 = 1000.00;
    println!("Seu saldo é de: R${:.2}",saldo);

    let saldo = saldo + 250.00;
    println!("Seu saldo após deposito é de: R${:.2}",saldo);

    let saldo = saldo - 150.00;
    println!("Seu saldo após saque é de: R${:.2}",saldo);
    println!("Taxa de juros mensal: {}%",juros);
    println!("Satus: Saldo disponivel: R${:.2}",saldo);


    println!("\n\nATIVIDADE 3.2");
    let nome: &str = "Leonardo";
    let nivel: u8 = 23;
    let pv: i64 = 100;
    let xp: u64 = 500;
    let crit: f64 = 0.5;
    let alive: bool = true;
    let class: char = 'G';

    let p: (i8,i8,i8) = (120,80,0);
    let (x,y,z) = p;

    let itens: [&str; 4] = ["Espada","Escudo","Poção","Mapa"];

    println!("== FICHA DO PERSONAGEM ==");
    println!("Nome: {}",nome);
    println!("Nivel: {}",nivel);
    println!("HP: {}",pv);
    println!("XP: {}",xp);
    println!("Critico: {}%",crit*100.0);
    println!("Vivo: {}",alive);
    println!("Classe: {}",class);

    println!("\nPosição: x={}, y={}, z={}",x,y,z);

    println!("Inventário: ");
    println!("- {}",itens[0]);
    println!("- {}",itens[1]);
    println!("- {}",itens[2]);
    println!("- {}",itens[3]);

    println!("\n\nATIVIDADE 3.3\n\n");

    fn calcular_desconto_inss(salario_bruto: f64) -> f64{
        salario_bruto * 0.11

    }

    fn calcular_desconto_ir(salario_bruto: f64) -> f64{
        if salario_bruto <= 2000.00{
            0.0
        }
        else if salario_bruto <=4000.00{
            salario_bruto*0.075
        }
        else{
            salario_bruto*0.15
        }
    }

    fn calcular_liquido(salario_bruto: f64) -> f64{
        salario_bruto-calcular_desconto_inss(salario_bruto)-calcular_desconto_ir(salario_bruto)
    }

    fn exibir_contracheque(nome: &str, salario_bruto: f64){
        println!("Funcionario: {}",nome);
        println!("Salario bruno: R${:.2}",salario_bruto);
        println!("(-) INSS R${:.2}",calcular_desconto_inss(salario_bruto));
        println!("(-) IR R${:.2}",calcular_desconto_ir(salario_bruto));
        println!("(=) Salário Liquido R${:.2}",calcular_liquido(salario_bruto));
    }

    let funcionario: &str = "Leonardo";
    let salario: f64 = 8000.00;
    exibir_contracheque(funcionario,salario);
}
