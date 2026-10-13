#[derive(Debug)] // Linha essencial para a formatação das saídas utilizando o índice {:#?}
struct Aluno { //Struct Aluno contendo código, nome e CPF.
    codigo: u32,
    nome: String,
    cpf: String,
}


fn main() { // Função principal
    let mut Alunos: Vec<Aluno> = Vec::new(); // Vetor mutável vazio usando o struct Aluno
    let mut aluno1 = Aluno{ //Primeiro aluno, com os campos preenchidos
        codigo:7,
        nome: String::from("Carlos"),
        cpf: String::from("065-422-391-21"),
    };
    let mut aluno2 = Aluno{ // Segundo aluno, com os campos preenchidos
        codigo:3,
        nome: String::from("Ana"),
        cpf: String::from("070-222-598-21"),
    };
    let mut aluno3 = Aluno{ // Terceiro aluno, com os campos preenchidos
        codigo:1,
        nome: String::from("Maria"),
        cpf: String::from("051-399-392-91"),
    };
    let mut aluno4 = Aluno{ // Quarto aluno, com os campos preenchidos
        codigo:8,
        nome: String::from("José"),
        cpf: String::from("035-532-591-70"),
    };

    // Sessão onde são adicionados os alunos ao vetor de Struct.
    Alunos.push(aluno1);
    Alunos.push(aluno2);
    Alunos.push(aluno3);
    Alunos.push(aluno4);

    // Sessão de organização dos alunos no vetor, usando Bubble Sort.
    let mut organizado = 0; // Varíavel mutável que será utilizada pra checar se o vetor está organizado.
 
    loop {
        if organizado == 1{ 
            break; // Caso o vetor esteja organizado, o break será acionado e o programa sairá do laço loop
        }
        organizado = 1; 
        for i in 0..Alunos.len() - 1{ // Laço for utilizado para organizar os alunos dentro do Vetor de Struct utilizando o código como chave. 
        
        if Alunos[i].codigo > Alunos[i + 1].codigo{ // Condicional que faz a verificação dos códigos.
            Alunos.swap(i, i + 1); // Método que permite a troca entre dois elementos por índice dentro de uma coleção
            organizado = 0; // Se houve troca, então o vetor ainda não está organizado. Se não houve, então está organizado.
        }
        }
    }
    
    println!("{:#?}", Alunos); // Impressão de todos os elementos do vetor de maneira organizada.

}
