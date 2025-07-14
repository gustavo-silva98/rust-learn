fn main() {
    let numero_teste : i32 = 42;
    let valida_par= is_even(numero_teste);

    if valida_par == true {
        println!("O número {numero_teste} é par") 
    } 
    else {
        println!("O número {numero_teste} NÃO é par")
    }
    
    counter_until_x(10)    
}

// Função com bloco if para teste de control flow
fn is_even(number : i32) -> bool {
    if number % 2 == 0 {
        true
    } 
    else {
        false
    }
}

fn counter_until_x(x:i32) {
    let mut count = 0;

    let result = loop{
        count +=1;

        if count == x {
            break count;
        }
    };

    println!("O resultado é {result}")
}