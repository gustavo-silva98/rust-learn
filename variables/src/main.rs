fn shadowing() {
    let x = 5;
    
    let x = x + 1;

    {
        let x = x*2;
        println!("O valor de x inner scope é: {x}")
    }

    println!("O valor de x fora de scope é: {x}")
}

fn main() {
    let mut x = 5;
    println!("O valor de x é: {x}");
    x = 6;
    println!("O valor de x é : {x}");

    println!("Vamos rodar o shadowing");
    shadowing();
}
