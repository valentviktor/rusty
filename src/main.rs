fn main() {
    println!("Hello, World! Saya belajar rust dasar sekarang.");
}

#[test]
fn hello_test() {
    println!("Hello Test");
}

#[test]
fn test_variable() {
    let mut name = "Devtor";
    println!("Hello, {}", &name);

    name = "Akhoi";
    println!("Hello, {}", &name);
}

#[test]
fn shadowing() {
    let name = "Devtor";
    println!("Name: {}", name);

    let name = 10;
    println!("Number: {}", name);
}

#[test]
fn explicit() {
    let age: i32 = 24;
    println!("Age: {}", age);
}

#[test]
fn number() {
    let a: i32 = 10;
    println!("{}", a);

    let b: f32 = 10.5;
    println!("{}", b);
}

#[test]
fn number_conversion() {
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = 10 + a as i16;
    println!("{}", b);

    let c: i32 = 10 + b as i32;
    println!("{}", c);

    let d: i64 = 1_000_000_000; //  Contoh error integer overflow alias tidak bisa ditampung dan hasilnya 0.
    let e: i8 = d as i8;
    println!("{}", e);
}

#[test]
fn numeric_operation() {
    let a = 10;
    let b = 10;

    let c = a + b;
    println!("c: {}", c);

    let d = a - b;
    println!("d: {}", d);

    let e = a / b;
    println!("e: {}", e);

    let f = a * b;
    println!("f: {}", f);

    let g = a % b;
    println!("g: {}", g);
}

#[test]
fn augmented_assigment() {
    let mut a = 10;
    println!("{}", a);

    a += 10;
    println!("a+: {}", a);

    a -= 10;
    println!("a-: {}", a);
}

#[test]
fn boolean() {
    let a = true;
    let b: bool = false;

    println!("{} {}", a, b);
}

#[test]
fn comparison_operators() {
    let a= 10 ;
    let b= 20 ;

    let i: bool = a > b;
    println!("{}", i);

    let j: bool = a < b;
    println!("{}", j);

    let k: bool = a >= b;
    println!("{}", k);
    
    let l: bool = a <= b;
    println!("{}", l);

    let m: bool = a == b;
    println!("{}", m);
}

#[test]
fn boolean_operator() {
    let absen = 75;
    let nilai = 80;

    let lulus = absen >= 75;
    let lulus_nilai_akhir = nilai >= 75;

    let lulus_final = lulus && lulus_nilai_akhir;
    println!("{}", lulus_final);
}

#[test]
fn char_type() {
    let a: char = 'a';
    let b: char = 'b';
    println!("{} {}", a, b);
}