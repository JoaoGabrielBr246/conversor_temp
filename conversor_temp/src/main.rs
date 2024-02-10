use std::io;

fn main() {
    println!("Digite a opção: ");
    println!("1-) Converter Celsius: ");
    println!("2-) Converter Fahrenheit: ");
    println!("3-) Converter Kelvin: ");
    println!("4-) Sair");

    let mut op = String::new();
    io::stdin().read_line(&mut op).expect("Erro ao ler a linha");
    let opcao: u16 = op.trim().parse().expect("Digite um número válido!");

    match opcao {
        1 => {
            println!("Digite a temperatura em Celsius: (apenas números)");
            let mut temp = String::new();
            io::stdin().read_line(&mut temp).expect("Erro ao ler a linha");
            let temp: f32 = temp.trim().parse().expect("Digite um número válido");

            let kelvin = temp + 273.15;
            let fahrenheit = (temp * 9.0 / 5.0) + 32.0;
            println!("{} Celsius é igual a {} Kelvin e {} Fahrenheit", temp, kelvin, fahrenheit);
        },
        2 => {
            println!("Digite a temperatura em Fahrenheit: (apenas números)");
            let mut temp = String::new();
            io::stdin().read_line(&mut temp).expect("Erro ao ler a linha");
            let temp: f32 = temp.trim().parse().expect("Digite um número válido");

            let celsius = (temp - 32.0) * 5.0 / 9.0;
            let kelvin = celsius + 273.15;
            println!("{} Fahrenheit é igual a {} Celsius e {} Kelvin", temp, celsius, kelvin);
        },
        3 => {
            println!("Digite a temperatura em Kelvin: (apenas números)");
            let mut temp = String::new();
            io::stdin().read_line(&mut temp).expect("Erro ao ler a linha");
            let temp: f32 = temp.trim().parse().expect("Digite um número válido");

            let celsius = temp - 273.15;
            let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
            println!("{} Kelvin é igual a {} Celsius e {} Fahrenheit", temp, celsius, fahrenheit);
        },
        4 => {
            println!("Saindo...");
        },
        _ => {
            println!("Opção inválida");
        }
    }
}
