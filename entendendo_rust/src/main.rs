use std::io; //importa o módulo io da biblioteca std.

fn ler_vetor() -> Vec<i32>{ // função criada para ler os vetores (listas) 1 e 2

    let mut input = String::new(); // cria uma string vazia que será preenchida com o input do usuário.

    io::stdin() //chama a função stdin() com os métodos read_line
        .read_line(&mut input) // lê o input do usuario e armazena em uma variável mutável chamada "input"
        .expect("Erro ao ler a linha."); // O método read_line pode gerar um valor Result que tem duas variantes (Err ou Ok) que indicam, respectivamente, se houve um erro ou não. 
                                        // Usando o método expect, podemos exibir uma mensagem de erro relacionada à leitura da linha.

    let vetor_lido: Vec<i32> = input // Aqui criamos um vetor imutavel de numeros de 32 bits chamado vetor_lido, que recebe a variável input tratada com os respectivos métodos:
        .split_whitespace() // Esse método separa o texto lido por espaços.
        .map(|s| s.parse().expect("Por favor digite apenas números")) // Esse método vai pegar cada item separado anteriormente e aplicar o método parse sobre eles, transformando-os em números de 32 bits, de acordo com a anotação de tipo.
        .collect(); // Esse método pega os elementos tratados e os junta em uma coleção(o vetor de inteiros de 32 bits)

    vetor_lido
}
fn main() { // função principal
    let mut vetor_resultado: Vec<i32> = Vec::new(); //cria um vetor de inteiros de 32 bits vazio
    let mut a = 0; // variável de controle mutável a
    let mut b = 0; // variável de controle mutável b
    
    println!("Digite os elementos do primeiro vetor: (Separe com espaço)");
    let vetor1 = ler_vetor(); // utilizamos a função detalhada acima para ler o vetor 1
    
    
    println!("Digite os elementos do segundo vetor: (Separe com espaço)");
    let vetor2 = ler_vetor(); // Agora lemos o vetor 2
    

    loop{ // laço infinito do rust, que pode ser interrompido com um break
        if a >= vetor1.len() && b >= vetor2.len(){ //estrutura condicional que checa se as variáveis de controle são maiores que ou iguais aos tamanhos de seus respectivos vetores.
                break; // Saída do loop caso a condição acima seja atingida.
            }
        if a < vetor1.len(){ // Checa se a variável a é menor que o tamanho do vetor 1, enquanto for verdade, adiciona o elemento no índice a ao vetor_resultado
            vetor_resultado.push(vetor1[a]);
            a = a + 1; // variavel de controle é incrementada
        }
        if b < vetor2.len(){ // Checagem da varíavel de controle b
            vetor_resultado.push(vetor2[b]);
            b = b + 1; // Variavel de controle é incrementada
            }
        

    
    
    
}
    println!("Vetor Resultado {:?}", vetor_resultado); // Vetor resultado é impresso com a formatação {:?} sendo usada principalmente para Debug.
}
