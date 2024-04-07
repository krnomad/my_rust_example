fn main() {
    {
        let s: String = String::from("Hello, world!");
        string_lenth(&s);
        println!("{}", s);
    }

    {
        let s: String = String::from("Hello, world!");
        let i: usize = string_lenth2(&s); // s is not moved to the function. s is borrowed
        println!("{} {}", s, i);        
    }

    {
        let s: String = String::from("Hello, world!");
        let s2: String = string_append1(s, "!!!");
        // println!("{}", s); // error: s is moved to the function above
        println!("{}", s2);
    }

    {
        let mut s: String = String::from("Hello, world!");
        string_append2(&mut s, "!!!");
        println!("{}", s);  // Hello, world!!!
    }

    // If mutable reference exists, immutable reference cannot exist at the same time
    {
        let mut s: String = String::from("Hello, world!");
        let r1: &String = &s;
        let r2: &String = &s;
        //let r3: &mut String = &mut s; // error: cannot borrow `s` as mutable because it is also borrowed as immutable
        println!("r1: {}, r2: {}", r1, r2);
    }

    {
        let mut s: String = String::from("Hello, world!");
        let s1: &String = &s;
        let s2: &String = &s;
        println!("s1: {}, s2: {}", s1, s2); // Hello, world!

        let s3: &mut String = &mut s; // if line 44 is commented out, this line will be error!
        // let s4: &mut String = &mut s; // error: cannot borrow `s` as mutable because it is also borrowed as immutable    
        //println!("s1: {}, s2: {}, s3: {}", s1, s2, s3); // error: cannot borrow `s` as mutable because it is also borrowed as immutable
    }

    {
        let mut s: String = String::from("Hello, world!");
        string_append3(&mut s, "!!!");
        string_append3(&mut s, "!!!");
        println!("{}", s);
    }
}

fn string_lenth(s: &String) {
    println!("The length of '{}' is {}.", s, s.len());
}

fn string_lenth2(s: &String) -> usize {
    s.len()
}

fn string_append1(s: String, append: &str) -> String {
    let mut s = s;
    s.push_str(append);
    s
}

fn string_append2(s: &mut String, append: &str) {
    s.push_str(append);
}

fn string_append3<'a>(s: &'a mut String, append: &str) -> &'a mut String {
    s.push_str(append);
    s
}