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
}

fn main() {
    scope();
    shadow();
    println!("som = {}", sum(2, 2));
    conditionals();
}
