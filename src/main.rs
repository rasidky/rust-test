// use std::io;

// fn main() {
//     println!("Kalkulator Test Rust");

//     // Ambil Angka
//     println!(
//         "Masukkan angka pertama : "
//     );
//     let mut input1 = String::new();
//     io::stdin()
//         .read_line(&mut input1)
//         .expect("Gagal membaca input");
//     let angka1: f64 = input1.trim().parse().expect("input tidak valid");

//     // Ambil Angka kedua
//     println!(
//         "Masukkan angka kedua : "
//     );
//     let mut input2 = String::new();
//     io::stdin()
//         .read_line(&mut input2)
//         .expect("Gagal membaca input");
//     let angka2: f64 = input2.trim().parse().expect("input tidak valid");

//     //ambil operator
//     println!("Masukkan operator (+, -, *, /): ");
//     let mut operator = String::new();
//     io::stdin()
//         .read_line(&mut operator)
//         .expect("Gagal membaca input");
//     let operator = operator.trim();

//     // Hitung hasil
//     let hasil = match operator {
//         "+" => angka1 + angka2,
//         "-" => angka1 - angka2,
//         "*" => angka1 * angka2,
//         "/" => {
//             if angka2 == 0.0 {
//                 println!("Error: Pembagian dengan nol tidak diperbolehkan.");
//                 return;
//             }
//             angka1 / angka2
//         }
//         _ => {
//             println!("Operator tidak valid. Gunakan salah satu dari +, -, *, /.");
//             return;
//         }
//     };

//     // Tampilkan hasil
//     println!(
//         "Hasil dari {} {} {} = {}",
//         angka1, operator, angka2, hasil
//     );
//     println!("Terima kasih telah menggunakan Kalkulator Test Rust!");
//     println!("Sampai jumpa lagi! 😊");
// }

// fn main (){
//     let mut x = 5;
//     println!("Nilai x adalah: {}", x);
//     x = 10;
//     println!("Nilai x setelah diubah adalah: {}", x);
// }

// fn main(){
//     for i in 1..=10 {
//         println!("Angka: {}", i);
//     }
// }

// fn main(){
//     let x = 5;
//     println!("Nilai x adalah: {}", x);
//     x = 6;
// }

// fn main(){
//     let mut x = 5;
//     println!("Nilai x adalah: {}", x);

//     x = 6;
//     println!("Nilai x setelah diubah adalah: {}", x);
// }

fn main(){
    let mut saldo = 100_000;

    println!("Saldo awal: {}", saldo);
    
    saldo = saldo + 50_000;
    println!("Saldo setelah penambahan: {}", saldo);

    saldo = saldo - 30_000;
    println!("Saldo setelah pengurangan: {}", saldo);

    saldo = saldo * 2;
    println!("Saldo setelah penggandaan: {}", saldo);

    if saldo >= 200_000 {
        saldo = saldo / 2;
        println!("Saldo setelah pembagian: {}", saldo);
    } else {
        println!("Saldo tidak cukup untuk pembagian.");
    }
    println!("Saldo akhir: {}", saldo);
}