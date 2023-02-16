use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::thread;
use std::time;

fn main() {
    println!("Adivina el número:");

    loop{
    println!("Ingresa tu numero:");

    let mut guess = String::new(); //mutable
    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => 0,
    };

    let mut random_number_generator = rand::thread_rng();
    let secret_number = random_number_generator.gen_range(1,101);
    println!("{}",secret_number);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too Small!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => println!("You Win"),
    }

    }

}

/*fn main(){
    let mut vueltas = 0;
    loop{
        let mut random_generator = rand::thread_rng();
        let first_random = random_generator.gen_range(0,1001);
        println!("The first number is {}", first_random);

        let second_random = random_generator.gen_range(0,1001);
        println!("The second number is {}", second_random);

        match first_random.cmp(&second_random){
            Ordering::Equal => {println!("They are equals"); break;},
            Ordering::Greater => println!("Fallo"),
            Ordering::Less => println!("Fallo"),
        }
        vueltas = vueltas + 1;
        //let ten_millis = time::Duration::from_millis(1000);
        //thread::sleep(ten_millis);
    }
    println!("Dio {} vueltas!",vueltas);
}*/
