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

fn tuple_descr() {
    let tup: (i32, f64, u8) = (500,10.2, 1);
    
    let (x, y, z) = tup;
    
    println!("O valor da tupla é {x},{y},{z}");

    let primeiro = tup.0;
    let segundo = tup.1;
    let terceiro = tup.2;
    println!("O valor primeiro da tupla é {primeiro}");
    println!("O valor segundo da tupla é {segundo}");
    println!("O valor terceiro da tupla é {terceiro}");

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
    println!("----------------------------------");
    println!("Função de Tuplas");
    tuple_descr();
    list_descr();
}

fn list_descr() {
    let list:[i32;3] = [1,2,3]; 

    let list_1 = list[0];
    let list_2 = list[1];
    let list_3 = list[2];

    println!("Primeiro elemento da lista é : {list_1}");
    println!("Segundo elemento da lista é : {list_2}");
    println!("Terceiro elemento da lista é : {list_3}");
}