#![allow(unused)]

fn main() {
    /*
        https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
        ownership adalah aturan RUST untuk MANAJEMEN MEMORY

        MEMORY TYPE (HEAP, STACK)
        
        STACK
        bersifat COPY (DI RUST)
        memiliki panjang value yg tetap, baik selama run-time atau compile-time
        digunakan oleh tipe data primitive:
            bool
            f32, f64
            char, *str
            i8, i16, i32 ...
            slice, tuple (dengan value yg semuanya juga primitive) (i32, i32) || (i32, char, &str)

        HEAP
        bersifat MOVED (DI RUST)
        memiliki panjang value berubah2, baik selama run-time atau compile-time
        penyebab memory leak apabila tidak hati2
        digunakan oleh tipe data non primitive:
            Vector
            String::from()


        RULE
        tiap variable memiliki OWNER
        tiap variable hanya bisa memiliki SATU OWNER pada SATU WAKTU
        ketika OWNER keluar dari SCOPE, maka VALUE akan DIDROP
    */



    // BASIC EXAMPLE   
    // THIS PRIMITIVE TYPE USING STACK MEMORY
    let s: i32 = 10; // PEMILIK value 10
    let b: i32 = s; // melakukan COPY value dari variable s dan menjadi PEMILIK 10
    println!("BASIC EXAMPLE: {} {}", s, b);

    // THIS NON PRIMITIVE TYPE USING HEAP MEMORY
    let s: String = String::from("neko"); // PEMILIK pertama value
    // s is valid in here
    let b: String = s; // mengambil HAK MILIK value dari variable s dan menjadi PEMILIK kedua value
    // but after b takes ownership value of s, s is NOT VALID in here
    // println!("BASIC EXAMPLE: {} {}", s, b); // generate error "value borrowed here after move"



    // FUNCTION EXAMPLE 
    // THIS PRIMITIVE TYPE USING STACK MEMORY
    let s: i32 = 10; // PEMILIK value 10
    let b: i32 = make_copy(s); // melakukan COPY value dari variable s dan menjadi PEMILIK 10
    println!("FUNCTION EXAMPLE: {} {}", s, b);

    // THIS NON PRIMITIVE TYPE USING HEAP MEMORY
    let s: String = String::from("neko"); // PEMILIK pertama value
    // s is valid in here
    let b: String = take_ownership(s); // mengambil HAK MILIK value dari variable s dan menjadi PEMILIK kedua value
    // but after b takes ownership value of s, s is NOT VALID in here
    // println!("BASIC EXAMPLE: {} {}", b, s); // generate error "value borrowed here after move"
}

fn make_copy(val: i32) -> i32 {
    val * 2
}

fn take_ownership(val: String) -> String {
    let mut new_val = String::from(" chan");
    new_val.insert_str(0, &val);
    new_val
}