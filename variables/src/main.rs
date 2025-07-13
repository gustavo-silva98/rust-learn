fn shadowing() {
    let x = 5;
    
    let x = x + 1;

    {
        let x = x*2;
        println!("O valor de x inner scope é: {x}");
    }

    println!("O valor de x fora de scope é: {x}");
}

fn math_operations() {
    let sum = 5 + 10;
    println!("Soma é {sum}");
    
    let sub = 10.0 - 5.5;
    println!("Subtração é {sub}");

    let product = 4 * 30;
    println!("Produto é {product}");

    let quotient = 56.7 / 32.2;
    let truncated = -5/3;

    println!("Quociente é {quotient}");
    println!("Truncado é {truncated}");

    let remainder = 10 % 5;
    println!("Remainder é {remainder}");
}

fn main() {
    let mut x = 5;
    println!("O valor de x é: {x}");
    x = 6;
    println!("O valor de x é : {x}");

    println!("Vamos rodar o shadowing");
    shadowing();
    println!("----------------------------------");
    println!("Função de Operações matemáticas");
    math_operations();

}
