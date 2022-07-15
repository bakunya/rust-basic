fn main() {
    /*
        REFERENCE BORROWING
        di rust, suatu variable non primitive memiliki sifat MOVED
        sehingga apabila kita ingin menggunakan variable sebelumnya, akan memiliki masalah yaitu
        value dari variable sebelumnya telah BERPINDAH dan variable tersebut menjadi tidak valid lagi
        di "current execution"

        let a = String::from("neko");
        let b = a;
        // a is not valid in here

        Sehingga kita membutuhkan teknik REFERENCE-BORROWING untuk mengatasi hal tersebut.
        BORROWING menggunakan teknik REFERENCE ke variable yg dimaksud

        let a = String::from("neko");
        let b = &a;
        // a is valid in here
        // b -> <reference to> -> a -> <reference to> -> String memory



        MUTABLE REFERENCE
        mutable reference adalah teknik yang dapat mengubah variable yg direferensikan dapat diubah
        atau menjadi mutable, meski berada di eksekusi context / block kode yg berbeda.

        let mut s = String::from("neko");
        println!("{}", s); // neko
        push_chan(&mut s);
        println!("{}", s); // neko chan

        fn push_chan(val: &mut String) {
            val.insert_str(val.len(), " chan");
        }

        MUTABLE REFERENCE RULE
        tapi mutable reference memiliki rule yg harus dipatuhi
            HARUS SATU MUTABLE REFERENSI DALAM SATU WAKTU UNTUK MENGHINDARI DATA RACE SAAT RUN-TIME
        
        // ERROR
        let mut s = String::from("neko");
        let s2 = &mut s;
        let s3 = &mut s;
        println!("{} {}", s2, s3); // ERROR: cannot borrow `s` as mutable more than once at a time

        // VALID
        let mut s = String::from("neko");
        let s2 = &mut s;
        println!("{}", s2);
        let s3 = &mut s;
        println!("{}", s3);
    */
}
