fn main() {
    let x = 5;
    println!("O valor de x é: {x}");
    let x = x + 1;
    println!("O novo valor de x é: {x} !");

    {
        let x = x * 2;
        println!("O Valor de x no escopo interno é {x}");
    }

    println!("Valor de x {x}");
}
