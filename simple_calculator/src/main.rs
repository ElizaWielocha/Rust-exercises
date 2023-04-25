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
    
}

fn main() {
    // proste menu
    println!("Functions:");
    println!("1. Addition (+)");
    println!("2. Subtraction (-)");
    println!("3. Multiplication (*)");
    println!("4. Division (/)");
    println!("5. Compounding (^)");
    println!("6. Logarithm (log10)");
    println!("7. Factorial (!)");

    let mut key = String::new();                                    // zmienna z wartoscia funkcji kalkulatora 
    println!("Enter number of function you want to perform: ");
    io::stdin().read_line( &mut key ).ok();                             // pobierz linię od użytkownika
    let key: i32 = key.trim().parse().unwrap();                             // zmiana na int
    println!("----------------------");

    if key == 1{
        println!("++ ADDITION ++");
        addition();
    }
    else if key == 2{
        println!("-- SUBTRACTION --");
        subtraction();
    }
    else if key == 3{
        println!("");
    }
    else if key == 4{
        println!("");
    }
    else if key == 5{
        println!("");
    }
    else if key == 6{
        println!("");
    }
    else if key == 7{
        println!("");
    }
    else {
        println!("Nothing was entered!")
    }


}
