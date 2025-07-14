fn main() {
    let x = plus_one(5);
    println!("Valor de x é {x}");
}

fn plus_one(x:i8) -> i8 {
    // Exemplo de funcao com tipagem de argumento e retorno
    // Podemos retornar com return, mas geralmente é implicito conforme abaixo
    // Importante aprender bem diferença entre statement e expression
    x+1
}

