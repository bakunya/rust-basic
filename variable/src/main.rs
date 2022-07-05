fn main() {
    /*
     * MUTABLE AND IMMUTABLE
     * 
     * di rust, semua variable adalah immutable (tidak bisa di re-assigned), tabi bisa menjadi mutable
     * apabila ditambahi keyword 'mut'
     * 
     * __DONT__
     * let x = 10;
     * x = 20;
     * println!("{}", x);
     * 
     * __DO__
     * let mut x = 10;
     * println!("{}", x);
     * x = 20;
     * println!("{}", x);
    */


    /*
     * CONSTANTA
     * di rust, membuat constanta diawali dengan keyword "const".
     * constanta tidak bisa direassign meski ditambahi keyword "mut".
     * constanta wajib diberikan jenis tipe data, jika tidak maka akan error.
     * constanta wajib menggunakan nama berhuruf kapital
     * 
     *__DONT__
     * const mut x = 1;
     * println!("{}", x)
     * x = 2;
     * println!("{}", x)
     * 
     * __DO__
     * const X: u8 = 1;
     * println!("{}", x)
    */



    /*
     * SHADOWING
     * 
     * di rust, kita tidak bisa reassign suatu variable dengan tipe data yang berbeda.
     * namun, jika diperlukan bisa menggunakan konsep "SHADOWING"
     * 
     * shadowing adalah recreate suatu variable dengan tipe data yang berbeda tapi tetap memiliki
     * nama yang sama.
     * shadowing tidak memerlukan keyword "mut" karena bersifat "recreate"
     * 
     * __DONT__
     * let mut x = 10;
     * println!("{}", x);
     * x = "string";
     * println!("{}", x);
     * 
     * __DO__
     * let x = 10;
     * println!("{}", x);
     * let x = "string";
     * println!("{}", x);
     */

     

    /*
     * MOVED AND COPY
     * 
     * di rust, variable memiliki behavior berupa moved and copy
     * 
     * moved adalah pemindahan hak milik atau ownership dari satu variable ke variable lain
     * atau ketika suatu variable di-passing ke argumen function. Namun, hal ini bisa diantisipasi dengan
     * menggunakan teknik "give and back" atau "reference".
     * 
     * __EXAMPLE GIVE AND BACK__
     * val adalah variable x yang dikembalikan dari function "some_function"
     * let (val, response) = some_function(x);
     * __EXAMPLE USING REFERENCE__
     * let response = come_function(&x);
     * 
     * 
     * copy adalah meng-copy hak milik atau ownership dan valuenya dari satu variable ke variable lain
     * atau ketika suatu variable di-passing ke argumen function.
     * 
     * moved berlaku ke variable NON-PRIMITIVE
     * copy berlaku ke variable PRIMITIVE
     * 
     * __MOVED BEHAVIOR NON PRIMITIVE VARIABLE__
     * __DONT__
     * let x = String::from("hello");
     * let y = x;
     * println!("{} {}", x, y);
     * 
     * __DO__
     * let x = String::from("hello");
     * let y = &x;
     * println!("{} {}", x, y);
    */   
}