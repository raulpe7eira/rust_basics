static mut GLOBAL:i32 = 1;

fn sum(a:i32, b:i32) -> i32 {
    let result = a + b;
    println!("{} + {} = {}", a, b, result);

    result
}

fn shadow() {
    let a = 123;
    {
        let b = 456;
        println!("dentro, b = {}", b);

        let a = 777;
        println!("dentro, a = {}", a);
    }
    println!("fora, a = {}", a);
}

fn scope() {
    unsafe {
        println!("GLOBAL = {}, size = {} bytes",
            GLOBAL,
            std::mem::size_of_val(&GLOBAL));
    }

    const PI:f32 = 3.14;
    println!("PI = {}, size = {} bytes",
        PI,
        std::mem::size_of_val(&PI));

    let variable:i32 = 128;
    println!("variable = {}, size = {} bytes",
        variable,
        std::mem::size_of_val(&variable));
    let variable:i32 = 120;
    println!("variable = {}, size = {} bytes",
        variable,
        std::mem::size_of_val(&variable));

    let decimal:f32 = 2.5;
    println!("decimal = {}, size = {} bytes",
        decimal,
        std::mem::size_of_val(&decimal));

    let mut boolean:bool = false;
    boolean = true;
    println!("boolean = {}, size = {} bytes",
        boolean,
        std::mem::size_of_val(&boolean));

    let letter:char = 'C';
    println!("letter = {}, size = {} bytes",
        letter,
        std::mem::size_of_val(&letter));

}

fn conditionals() {
    let idade:u8 = 17;
    let parent_authorized = true;

    if idade >= 18 {
        println!("Pode entrar na balada");
    } else if idade > 16 && parent_authorized {
        println!("Pode entrar com assinatura do responsável");
    } else {
        println!("Não pode entrar na balada");
    }

    let condition = if idade >= 18 { "maior" } else { "menor" };
    println!("É {} de idade", condition);

    let language = "C#";
    let propose = match language {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        _ => "Unkown"
    };

    println!("O propósito de {} é {}", language, propose);
}

fn loops() {
    let multiple:u8 = 5;

    let mut count:u8 = 0;
    while count < 10 {
        count += 1;

        if count == 5 {
            continue;
        }

        println!("{} x {} = {}", multiple, count, multiple * count);
    }

    count = 0;
    loop {
        count += 1;
        println!("{} x {} = {}", multiple, count, multiple * count);

        if count == 10 {
            break;
        }
    }

    for count in 1..=10 {
        println!("{} x {} = {}", multiple, count, multiple * count);
    }
}

fn borrowing(string: &mut String) {
    string.push_str(" Dias");
    println!("{}", string);
}

fn ownership() {
    let mut one_string = String::from("Vinicius");
    borrowing(&mut one_string);

    println!("{}", one_string);
}

fn pattern_matching() {
    for x in 1..=20 {
        println!("{}: {}", x, match x {
            1 => "Pouco",
            2 | 3 => "Um pouquinho",
            4..=10 => "Um bocado",
            _ if x % 2 == 0 => "Uma boa quantidade",
            _ => "Muito"
        });
    }
}

fn result() -> Result<String, u8> {
    Err(42)
}

fn error() {
    match result() {
        Ok(msg) => println!("String de sucesso = {}", msg),
        Err(err) => println!("Código de erro = {}", err)
    };
}

fn main() {
    scope();
    shadow();
    println!("som = {}", sum(2, 2));
    conditionals();
    loops();
    ownership();
    pattern_matching();
    error();
}
