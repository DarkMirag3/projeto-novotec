fn main() {
    //shadow var
    let x = 2;
    let x = x + 1;
    {
        let x = x * 2;
        println!("{x}");
    }
    println!("{x}");
    //tupla
    let tupla = (1, 2, 3);
    let (x, y, z) = tupla;
    println!("{z}, {x}, {y}");
    //array
    let _array = [1, 2, 3];
    let _array2 = [3; 5]; //repete 3 cinco vezes;
    //function
    hello();
    soma(1, 2);
    let fives = five();
    println!("{fives}");
    //control flow if - else - else if  
    //if in a statement 
    let condition = true;
    let number2 = if condition {4} else {6};
    println!("the value of number is: {number2}", );
    //ownership
    let texto = String::from("TESTEKKKK");
    let texto2 = texto.clone();
    println!("{texto}, agora o clone {texto2}", );
}

fn hello() {
    println!("Hello World!");
}

fn soma(x: i32, y: i32) {
    let add = x + y;
    println!("The result is: {add}", );
}

fn five() -> i32 {
    5
}