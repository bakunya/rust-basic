fn main() {
    /*
     * INTEGERS
     * integers adalah data tipe number
     * memiliki banyak size 8, 32, 16, 64, 128, dan arch bit (https://doc.rust-lang.org/book/ch03-02-data-types.html)
     * unsigned adalah integers bernilai positif (+) atau negatif (-)
     * signed adalah integers yang hanya bernilai positif (+)
     * DEFAULTnya adalah u32
     * 
     * EXAMPLE
     * let some_int: u32 = -10;
     * let some_int: u32 = 10;
     * let some_int: i32 = 11;
     * 
     * 
     * 
     * FLOATING POINT
     * integers adalah data tipe floating point number
     * memiliki size f32 dan f64
     * DEFAULTnya adalah f64 karena modern cpu sudah support f64 dan kecepatannya sama dengan f32
     * 
     * EXAMPLE
     * let some_float: f32 = 10.11;
     * let some_float: f64 = 10.11;
     * 
     * 
     * 
     * CHARACTER
     * menggunakan diawali dan diakhiri tanda petik satu
     * panjangnya hanya 1
     * 
     * EXAMPLE
     * let some_char: char = 'a';
     * 
     * 
     * 
     * STRING LITERALS
     * pada dasarnya string adalah slice dari character
     * diawali dan diakhiri petik dua
     * karena merupakan slice dari character dan panjang value dari tipe str tidak diketahui secara default saat compile,
     * kita harus menggunakan reference dari tipe str itu sendiri.
     * 
     * EXAMPLE
     * let some_string: &str  = "lorem ipsum";
     * 
     * 
     * 
     * TUPLE
     * tuple mirip array di javascript
     * diawali dan diakhiri tanda kurung
     * gunakan koma sebagai pemisah antar item/value
     * valuenya bisa bertipe MIX atau CAMPURAN
     * akses tiap elemen berdasarkan index menggunakan titik.
     * bersifat COPY apabila value tidak mengandung tipe data NON-PRIMITIVE
     * 
     * EXAMPLE
     * let some_tuple: (&str, char, u32) = ("lorem", 'a', 32); //copy
     * let some_tuple: (&str, char, String) = ("lorem", 'a', String:from("lorem")); //move
     * 
     * println!("{}", some_tupple.0) // lorem
     * 
     * 
     * 
     * ARRAY
     * array di javascript
     * diawali dan diakhiri tanda kurung siku
     * gunakan koma sebagai pemisah antar item/value
     * valuenya HARUS bertipe SAMA
     * akses tiap elemen berdasarkan index menggunakan kurung siku.
     * bersifat COPY apabila value tidak mengandung tipe data NON-PRIMITIVE
     * 
     * EXAMPLE
     * let some_array_1: [u32, u32, u32] = [10, 11, 32]; //copy
     * let some_array_2: [String, String] = [String:from("lorem"), String:from("lorem_2")]; //move
     * 
     * println!("{}", some_array_1[0]) //10
     * println!("{}", some_array_2[0]) //lorem
     */
}