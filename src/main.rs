fn main(){
    let unsigned: u8 = 2;
    let signed: i8 = -2;
    let float: f64 = 1.2;
    let isTrue: bool = true;
    let emoji = "\u{1F603}";
    println!("Hello {}", emoji); 


    let arr: [u8; 3] = [1, 2, 3];
    println!("{}", arr[0]);
    println!("{:?}", arr);

    let tuple: (u8, bool, f32) = (5, true, 2.2);

    let (a, b, c) = tuple;

    println!("first {}, second {}, third {}", a, b, c);

    println!("{}", is_even(3));

    is_even(3);

    let mut num = 5;
    num = 2;
    println!("{}", num);


    let arr = [0,1,2,3];
    let slice = &arr[1 .. 4];
    println!("{:?}", slice);




    let str: &str = "hello";


    let mut string: String = String::from("Hello");

    string = string.replace("Hello", "Chlta hu bhai");

    string.push_str(" ji");


    println!("{}", string);

    println!("{:?}", 0..5);


    for i in 0..5 {
        println!("{}", i)
    }



    let matchI: i8 = 4;

    match matchI {
        0 => println!("0"),
        1 | 2 => println!("1 or 2"),
        3..=4 => println!("3"),
        _ => println!("Something else"), // This arm catches all other values
    }


    let name = String::from("Yash");

    let dev = Developer { 
        name, age: 10
    };

    dev.print_name();
    println!("{}", dev.is_web_dev());

    let mut vec: Vec<i64> = vec![1,2,3];

    vec.remove(1);

    vec.push(3);

    println!("{:?}", vec);


    let a: Myenum = Myenum::A;
    let b: Myenum = Myenum::B(3);
    let c: Myenum = Myenum::C { x: 1, y: 2 };

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);


    
}


pub fn is_even(num: u8) -> bool {

    let digit: u8 = num % 2;
    digit == 0
}


struct Developer {
    name: String,
    age: i8
}


impl Developer {

    fn print_name(&self){
        println!(
            "{}, Age: {}", self.name, self.age
        )
    }
    
}

impl Web for Developer {
    fn is_web_dev(&self) -> bool{
        true
    }
}


trait Web {
    fn is_web_dev(&self) -> bool;
    fn is_game_dev(&self) -> bool{
        false
    }
}

#[derive(Debug)]
enum Myenum {
    A, 
    B(i32),
    C {x: i32, y: i32}
}