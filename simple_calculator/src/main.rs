use std::io;

// Dodawanie
fn addition() {
    let mut x = String::new();
    println!("Enter first number: ");
    io::stdin().read_line( &mut x ).ok();
    let x: f64 = x.trim().parse().unwrap();

    let mut y = String::new();
    println!("Enter second number: ");
    io::stdin().read_line( &mut y ).ok();
    let y: f64 = y.trim().parse().unwrap();

    println!("{} + {} = {:.3}", x, y, (x+y));           // wynik zaokraglam do 3 miejsc po przecinku
}

// Odejmowanie
fn subtraction() {
    let mut x = String::new();
    println!("Enter first number: ");
    io::stdin().read_line( &mut x ).ok();
    let x: f64 = x.trim().parse().unwrap();

    let mut y = String::new();
    println!("Enter second number: ");
    io::stdin().read_line( &mut y ).ok();
    let y: f64 = y.trim().parse().unwrap();

    println!("{} - {} = {:.3}", x, y, (x-y));           // wynik zaokraglam do 3 miejsc po przecinku
}

// Mnozenie
fn multiplication() {
    let mut x = String::new();
    println!("Enter first number: ");
    io::stdin().read_line( &mut x ).ok();
    let x: f64 = x.trim().parse().unwrap();

    let mut y = String::new();
    println!("Enter second number: ");
    io::stdin().read_line( &mut y ).ok();
    let y: f64 = y.trim().parse().unwrap();

    println!("{} * {} = {:.3}", x, y, (x*y));           // wynik zaokraglam do 3 miejsc po przecinku
}

// Dzielenie
fn division() {
    let mut x = String::new();
    println!("Enter first number: ");
    io::stdin().read_line( &mut x ).ok();
    let x: f64 = x.trim().parse().unwrap();

    let mut y = String::new();
    println!("Enter second number: ");
    io::stdin().read_line( &mut y ).ok();
    let y: f64 = y.trim().parse().unwrap();

    println!("{} / {} = {:.3}", x, y, (x/y));           // wynik zaokraglam do 3 miejsc po przecinku
}

// Potegowanie
fn compounding() {
    let mut x = String::new();
    println!("Enter power base: ");
    io::stdin().read_line( &mut x ).ok();
    let x: f64 = x.trim().parse().unwrap();

    let mut a = String::new();
    println!("Enter power exponent: ");
    io::stdin().read_line( &mut a ).ok();
    let a: i64 = a.trim().parse().unwrap();

    let mut result: f64 = 1.0;
    for _i in 1..(a+1) {
        result = result * x;
    }
    println!("( {} )^{} = {:.3}", x, a, result);
}

// Logarytm
fn logarithm() {
    let mut a = String::new();
    println!("Enter base of the logarithm: ");
    io::stdin().read_line( &mut a ).ok();
    let a: f64 = a.trim().parse().unwrap();

    let mut x = String::new();
    println!("Enter logarithmic number : ");
    io::stdin().read_line( &mut x ).ok();
    let x: f64 = x.trim().parse().unwrap();

    println!("log{}({}) = {:.3}", a, x, x.log(a));
}

// Silnia
fn factorial() {
    let mut x = String::new();
    println!("Enter number to factor: ");
    io::stdin().read_line( &mut x ).ok();
    let x: i64 = x.trim().parse().unwrap();

    let mut result: i64 = 1;
    for i in 1..(x+1) {
        result = result * i;
    }
    println!("{}! = {:.3}", x, result);
}

fn main() {
    let mut exit: bool = false;
    while exit == false {
        // Proste menu
        println!("\n ___WELCOME TO SIMPLE CALCULATOR IN RUST!___");
        println!("|Functions:                                 |");
        println!("|1. Addition (+)                            |");
        println!("|2. Subtraction (-)                         |");
        println!("|3. Multiplication (*)                      |");
        println!("|4. Division (/)                            |");
        println!("|5. Compounding (^)                         |");
        println!("|6. Logarithm (log)                         |");
        println!("|7. Factorial (!)                           |");
        println!("|8. Exit                                    |");
        println!("|___________________________________________|");

        let mut key = String::new();                                    // zmienna z wartoscia funkcji kalkulatora 
        println!("Enter number of function you want to perform: ");
        io::stdin().read_line( &mut key ).ok();                             // pobierz linię od użytkownika
        let key: i32 = key.trim().parse().unwrap();                             // zmiana na int
        //println!("--------------------------------------------");

        if key == 1 {
            println!("_______________++ ADDITION ++_______________");
            addition();
        }
        else if key == 2 {
            println!("______________-- SUBTRACTION --_____________");
            subtraction();
        }
        else if key == 3 {
            println!("____________** MULTIPLICATION **____________");
            multiplication();
        }
        else if key == 4 {
            println!("_______________// DIVISION //_______________");
            division();
        }
        else if key == 5 {
            println!("_____________^^ COMPOUNDING ^^______________");
            compounding()
        }
        else if key == 6 {
            println!("______________log LOGARITHM log_____________");
            logarithm()
        }
        else if key == 7 {
            println!("_______________!! FACTORIAL !!______________");
            factorial();
        }
        else if key == 8 {
            exit = true;
        }
        else {
            println!("Wrong number entered!")
        }
        println!(".\n.\n.");

    }

}
