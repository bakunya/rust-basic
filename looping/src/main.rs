fn main() {
    loop_keyword(20);
    labelled_loop();
    while_loop(10);
    for_loop();
}

fn for_loop() ->() {
    /*
        For loop atau for in loop, mirip for in range di python
        Jika ingin menggunakan hal mirip index, tinggal pakai metode tuple (0..5). DIBACA 0 kurang dari 5.
    */

    // EXAMPLE 1
    let arr = [1,2,3,4,5];
    for number in arr {
        println!("Number: {}", number);
    }

    // EXAMPLE 2
    for index in 0..5 /* DIBACA 0 sampai kurang dari 5. atau bisa juga pakai kurung (0..5) */ {
        println!("Index: {}", index);
    }
}

fn while_loop(length: i32) -> () {
    /*
        seperti while loop biasa, tidak ada yg spesial.
    */

    let mut i = 0;

    while i < length {
        println!("index: {}", i);
        i += 1;
    }
}

fn loop_keyword(length: i32) -> () {
    /*
    * Menggunakan keyword LOOP harus secara eksplisit menuliskan break
    * untuk mencegah infinity loop;
    *
    * contoh di bawah akan menghasilkan infinity loop
    * loop {}
    */

    let mut i = 0;

    loop {
        i += 1;
        println!("{}", i);
        if i >= length {
            break;
        }
    }
}

fn labelled_loop() -> () {
    /*
        Labelled loop menghilangkan keambiguan saat menggunakan loop bersarang
        Labelled loop memungkinkan untuk menggunakan keyword break atau continue untuk loop parent saat waktu eksekusi berada di loop children
        Penulisan labelled loop wajib diawali dengan petik satu '

        EXAMPLE
        'label_name: loop{}

        EXAMPLE BREAK
        break 'label_name

        EXAMPLE CONTINUE
        continue 'label_name
    */
    let mut  count_1 = 0;

    'loop_1: loop {
        let mut count_2  = 0;

        loop {
            println!("Loop pertama ke-{}. Loop kedua ke-{}", count_1, count_2);

            if count_2 >= 5 {
                break;
            }

            if count_1 >= 3 {
                break 'loop_1;
            }

            count_2 += 1
        }

        count_1 += 1
    }
}