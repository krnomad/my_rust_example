fn main() {
    let s = "Hello, world!"; // string literal
    let s2: String = String::from("Hello, world!"); // heap-allocated string
    // let s3: String = "is it possible?"; // error: type annotations needed
    let s3: String = "is it possible?".to_string(); // heap-allocated string - to_string() method

    // 러스트에서 소유권은 어느한 시점에 하나의 변수만이 소유할 수 있다.
    // 소유자가 범위를 벗어나면 메모리가 해제된다.
    {
        let x = 3;
        let y = x; // copy value of x to y because x is a primitive type
        println!("x: {}, y: {}", x, y); // x: 3, y: 3
    }

    {
        let s1 = String::from("hello");
        println!("s1: {}", s1); // hello

        let s2 = s1; // move s1 to s2
        // println!("s1: {}", s1); // error: value borrowed here after move
        println!("s2: {}", s2); // hello
    }
    
    {
        let s1 = String::from("hello");
        let s2 = s1; // move s1 to s2
        println!("s2: {}", s2); // hello
        // println!("s1: {}", s1); // error: value borrowed here after move
    }

    {
        let s1 = String::from("hello");
        let s2 = s1.clone(); // deep copy
        println!("s1: {}, s2: {}", s1, s2); // hello, hello
    }   

    {
        let s: String = String::from("Hello, world!");
        string_lenth(s);
        // println!("{}", s); // error: value borrowed here after move

        let x: i32 = 5;
        let y: i32 = double(x); // copy value of x to y because x is a primitive type
        println!("x: {}, y: {}", x, y); // x: 5, y: 10
    }

    {
        let s: String = String::from("Hello, world!");
        let s2: String = string_lenth2(s); // return value ownership to s2. s is moved to s2 in the function
        println!("{}", s2);
        // println!("{}", s.len()); // error: s is moved to s2 in the function above
    }

    {
        let s: String = String::from("Hello, world!");
        let (length, s): (usize, String) = string_length3(s); // return multiple values
        println!("The length of '{}' is {}.", s, length);        
    }
}

fn string_lenth(s: String) {
    println!("The length of '{}' is {}.", s, s.len());
}

fn string_lenth2(s: String) -> String {
    println!("The length of '{}' is {}.", s, s.len());
    s
}

fn string_length3(s: String) -> (usize, String) {
    (s.len(), s)
}

// primitive type is copied when passed to a function
fn double(x: i32) -> i32 {
    x * 2
}