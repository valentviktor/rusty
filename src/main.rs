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
    println!("Hello, {}", name);

    name = "Akhoi";
    println!("Hello, {}", name);
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
    let a = 10;
    let b = 20;

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

#[test]
fn tuple() {
    // Immutable tuple
    let data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}",  data);

    // Akses item tuple by index
    let a = data.0;
    let b = data.1;
    let c = data.2;
    println!("Tuple item access: {} {} {}", a, b,  c);

    // Destructuring tuple
    let (x, y, z) = data;
    println!("Destructuring tuple: {} {} {}", x, y, z);

    // Mutable tuple
    let mut new_data = (10, 10.5, true);

    new_data.0 = 20;
    new_data.1 =  25.7;
    new_data.2 = false;
    println!("{:?}", new_data);
}

// Unit (tuple kosong)
fn unit() {
    println!("Hello ini unit (tuple kosong)");
}

#[test]
fn test_unit() {
    let result = unit(); // Tipe datanya tuple kosong
    println!("{:?}", result);

    let test: () = ();
    println!("{:?}", test);
}

// Array mirip tuple, namun hanya 1 tipe data saja
#[test]
fn array() {
    // Immutable array
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    // Akses item array
    let a = array[0];
    let b  = array[1];
    println!("Akses item array: {} {}", a, b);

    // Mutable array
    let mut new_array: [i32; 5] = [1, 2, 3, 4, 5];
    new_array[0] = 10;
    new_array[1] = 20;
    println!("{:?}", new_array);

    // Hitung panjang array, hanya di array bisa hitung panjangnya/jumlah datanya
    let array_length = array.len();
    let new_array_length = new_array.len();
    println!("panjang array: {} & panjang new array: {}", array_length, new_array_length)
}

#[test]
fn two_dimensional_array() {
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6]
    ];

    println!("{:?}", matrix);
    println!("{}", matrix[0][0]);
    println!("{}", matrix[0][1]);
    println!("{}", matrix[1][0]);
    println!("{}", matrix[1][2]);
}

// Constant: tidak pernah berubah dan harus deklarasi ekspresi diawal
const MAXIMUM: i32 = 100;

#[test]
fn constant() {
    const MINIMUM: i32 = 0;
    println!("Min: {} & Max: {}", MINIMUM, MAXIMUM);
}

#[test]
fn variable_scope() {
    let firstname = "Viktor";

    {   // inner scope
        println!("firstname: {}", firstname);
        let lastname = "Valent";
        println!("lastneme: {}", lastname);
    }

    // println!("coba akses inner: {}", lastname); // akan error
}

// Memory management: Stack & Heap
#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10;
    let b = String::from("Viktorius");

    println!("{} {}", a, b);
}

fn function_b() {
    let a = 10;
    let b = String::from("Akhoi");

    println!("{} {}", a, b);
}

// String
#[test]
fn string_type() {
    let mut name: String = String::from("Viktorius");
    name.push_str(" Valentino");
    println!("{}", name);

    let udin = name.replace("Viktorius", "Udin");
    println!("{}", udin);
}

// Ownership
#[test]
fn ownership_rules() {
    // a tdak bisa diakses disini, belum di deklarasikan
    let a = 10;

    { // b tidak bisa diakses disini, karena belum di deklarasikan
        let b = 20; //  b bisa diakses mulai disini
        println!("{}",  b);
    }  // scope b selesai, b dihapus, b tidak bisa diakses lagi

    println!("{}", a);
} //  scope a selesai, a dihapus, a tidak bisa diakses lagi

#[test]
fn data_copy() {
    let a = 10;
    let b  = a; // copy data dari a ke b

    println!("{} {}", a,  b);
}

#[test]
fn ownership_movement() {
    let name1: String = String::from("Viktor");
    println!("{}", name1);

    // ownership dari name1 dipindahkan ke name2
    let name2: String = name1;
    // name1 tidak bisa diakses disini

    println!("{}", name2);
}

#[test]
fn clone() { // penggunaan clone agak berat, kalau datanya banyak
    let name1 = String::from("Viktor");
    let name2 = name1.clone();

    println!("{} {}", name1, name2)
}

// If expression
#[test]
fn if_else_expression() {
    let value  = 4;

    if value >= 8 { // tidak wajib pake kurung ()
        println!("Good");
    } else {
        println!("Not Good");
    }
}

#[test] 
fn else_if_expression() {
    let value  = 8;

    if value >= 8 { // tidak wajib pake kurung ()
        println!("Good");
    } else if  value >= 5 {
        println!("Not Bad");
    } else if value >= 3 {
        println!("So Bad");
    } else {
        println!("You fail");
    }
}

#[test] 
fn let_statement() {
    // cara biasa
    let value  = 6;
    let result: &str;

    if value >= 8 {
        result =  "Good";
    } else if  value >= 5 {
        result =  "Not Bad";
    } else if value >= 3 {
        result =  "Bad";
    } else {
        result =  "Very Bad";
    }
    // end cara biasa

    // cara let statement
    let results: &str = if value >= 8 {
        "Good"
    } else if  value >= 5 {
        "Not Bad"
    } else if value >= 3 {
        "Bad"
    } else {
        "Very Bad"
    };

    // end cara let statement

    println!("cara biasa: {:?} \ncara let statement: {}", result, results);
}

#[test] 
fn loop_expression() { // loop yang dikendalikan dengan break dan continue
    let mut counter: i32 = 0;

    loop {
        counter += 1;
        if counter > 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }

        println!("Counter: {}", counter);
    }
}

#[test]
fn loop_return_value() { // seperti let statement, bisa langsung return value
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result: {}", result);
}

#[test]
fn loop_label() { // loop didalam loop, diberi label untuk loop parentnya
    let mut number = 1;
    'outer: loop { // label loop pertama
        let mut i = 1;
        loop { 
            if number > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", number,  i, number * i);

            i += 1;
            if i > 10 {
                break;
            }
        }
        number += 1;
        print!("\n");
    }
}

#[test]
fn while_loop() { // ada kondisinya, namun tetap bisa menggunakan break dan continue
    let mut counter = 0;

    while counter <= 10 {
        if counter % 2 == 0 {
            println!("Counter: {}", counter);
        }
        counter += 1;
    }
}

#[test]
fn array_iteration() {
    // cara while
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    while index < array.len() {
        println!("While value: {}", array[index]);
        index += 1;
    }
    // end cara while

    println!("");

    // for loop, mirip foreach
    for value in array {
        println!("For value: {}", value);
    }
    // end for
}

#[test] 
fn range() {
    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    let range = 0..5; //  exclusive: 0, 1, 2, 3, 4 
    println!("Start: {}", range.start);
    println!("End: {}\n", range.end);

    for i in range {
        println!("r1 value: {}", array[i]);
    }

    println!("");

    let range2 = 0..=4; // inclusive: 0, 1, 2, 3, 4 
    println!("Start: {}", range2.start());
    println!("End: {}\n", range2.end());

    for i in range2 {
        println!("r2 value: {}", array[i]);
    }

    println!("");

    for i in 2..=4 { // langsung range disini
        println!("r2 value: {}", array[i]);
    }
}

// Function
fn greeting() {
    println!("Shalom!");
}

#[test]
fn test_greeting() {
    greeting();
}

fn goodbye(first_name: &str, last_name: &str) {
    println!("Sampai jumpa lagi, {first_name} {last_name}!");
}

#[test]
fn test_goobye() {
    goodbye("John", "Doe");
    goodbye("Sopo", "Jarwo");
}