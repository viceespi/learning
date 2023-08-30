fn main() {
    let nenega = Dog {
        name: String::from("Nina"),
        age: 3,
        color: Color::Black,
        breed: Breed::Labrador,
    };
    nenega.bark();
    Dog::bark(&nenega);
    let mut name: String = String::new();
    std::io::stdin().read_line(&mut name);
    let angus = Dog::born(name, Color::Chocolate, Breed::Labrador);
    angus.bark();
    do_the_thing(angus);
}

fn do_the_thing(foo: impl LivingBeing ) {
    foo.eat();
}
/*
Structs:
Structs são tipos elaborados pelo programador, tendo sempre um padrão característico composto pelos Campos.
No exemplo, Dog é um tipo que sempre tem que ter os campos name, age, color e breed.
Os structs são como blueprints, e servem para alocar na memória o espaço necessário pra ele, sendo esse espaço
composto da soma dos espaços necessários para alocar os tipos dos campos.
Quando se cria uma variável do tipo construido, é necessário que ela tenha todos os campos preenchidos.
Os campos do struct podem ser qualquer variáveis de qualquer tipo, incluindo structs (incluindo ela mesma)
e enums.

Implementation:
Impl é a criação de funções que são utilizadas pelo struct criado, sendo utilizada em rust da mesma maneira
em que outras funções específicas de tipos são utilizadas, usando ::. Essa implementação possui dois tipos,
uma em que você precisa já da variável com valor, e outra que age diretamente no struct sem valor.
A primeira, utiliza como parâmetro (&self), que serve para referenciar a variável na qual a função foi
chamada. No exemplo, temos a função bark, que necessita de um Dog existente para que aconteça.
A segunda, utiliza da implementação na qual seus parâmetros são os campos da struct, mas não todos e, no
seu corpo, ela atribui um valor para o campo que falta necessário pelo tipo struct que ela será. Da maneira
em que foi feita, por retornar Self, essa função já cria uma variável do tipo Dog e que já tem valor, já
que seus parâmetros são utilizados para construir o struct.

Na função bark, da maneira em que foi implementada, há tanto a possibilidade de ser chamada como dog.bark()
ou como Dog::bark(&dog).
*/
struct Dog {
    name: String,
    age: i32,
    color: Color,
    breed: Breed,
}

impl Dog {
    fn bark(&self) {
        println!("{} says: Au Au", self.name);
    }

    fn born(name: String, color: Color, breed: Breed) -> Self {
        Dog {
            name: name,
            color: color,
            breed: breed,
            age: 0,
        }
    }

    fn groom(&self) {
        let shampoo: &str = match self.color {
            Color::Black => "Black Shampoo",
            Color::Chocolate => "Dessert Shampoo",
            Color::Golden => "Acid Shampoo",
        };
    }
}

impl LivingBeing for Dog {
    fn eat(&self) {
        println!("Humm, que papa gostoso!");
    }
}
/*
Enum:
Os enums são types que sempre possuem valores que são definitivos e são definidos pelo usuário no momento
de sua criação.

Os enums podem ser utilizados em uma função, mas quando são utilizados, deve-se incluir a utilização de todos
os valores pré-definidos
*/
enum Color {
    Black,
    Chocolate,
    Golden,
}

enum Breed {
    Labrador,
    GoldenRetriever,
    Boxer,
    Beagle,
    Mutt,
}

/*

*/

trait LivingBeing {
    fn eat(&self);
}