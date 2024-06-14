use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("数字をGUESSするゲーム(白目)");

    let secret_number = rand::thread_rng().gen_range(1..101); // ランダム

    loop { // ループ
        println!("予想を入力してみてね。");

        let mut guess = String::new(); // String型

        io::stdin() // 読み取り
            .read_line(&mut guess)
            .expect("なんだこれーー！");
        
        let guess: u32 = match guess.trim().parse() {
           Ok(num) => num,
        Err(_) => continue,
        }; // u32型にする
        //    .expect("数字にしろーー！");

        println!("予想: {}", guess);

        match guess.cmp(&secret_number) { // 比較, Lessで小さい/Greater:大きい/Equal:一致
            Ordering::Less => println!("小さすぎ"),
            Ordering::Greater => println!("大きすぎ"),
            Ordering::Equal => {
                println!("正解！ 終了～");
                break;
            },
        }
    }
}
