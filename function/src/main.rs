#[allow(unused_variables)]
#[allow(unused)]

fn main() {
    // GIVE BACK EXAMPLE
    let mut a = String::from("neko");
    let (a_val, new_val) = add_string_give_back(a);
    println!("A value {}", a_val);
    println!("B value {}", new_val);
    
    // REFERENCE EXAMPLE
    let mut c = String::from("neko");
    let d = add_string_ref(&c);
    println!("C value {}", c);
    println!("D value {}", d);

    // FUNCTION AS ARGUMENTS
    func_args(func);

    // NOTE
    /*
    let mut a = String::from("neko");
    let mut b = String::from(" chan");
    b.insert_str(0, &a.to_string()); //is &a.to_string() copying itself value then give to b.insert_str ?
    a.push_str(" kawaii"); // if its not, why b variable not change to "neko kawaii chan" ?
    let mut c = &a;
    println!("{}", a); //neko kawaii
    println!("{}", b); //neko chan
    println!("{}", c); //neko chan
    */
}

/*
 * FUNCTION
 * harus secara EKSPLISIT menuliskan tipe data pada ARGUMEN dan RETURN VALUE
 * return value bisa secara eksplisit diketikkan atau tidak, tapi tidak menggunakan titik koma
 * fn() { "some" }
 * fn() { return "some"; }
 *
 * secara default mengambil ownership atau "hak milik" suatu variable apabila dipassing ke dalam argument
 * bisa diatasi dengan teknik REFERENCE atau GIVE AND BACK
 */

//  GIVE BACK EXAMPLE
 fn add_string_give_back(some_string: String) -> (String, String) {
    let mut chan = String::from(" chan");
    chan.insert_str(0, &some_string);
    (some_string, chan)
 }

//  REFERENCE EXAMPLE
fn add_string_ref(some_string: &String) -> String {
    let mut chan = String::from(" chan");
    chan.insert_str(0, &some_string);
    chan
}

// FUNCTION AS ARGUMENT EXAMPLE
fn func() { 
    println!("callback from main");
}

fn func_args(func: fn() -> ()) {
    func();
}