enum e<T, W> {
    a(T),
    b(W,)
}

fn main() {    
    
    let mut line = String::new();
    println!("Enter your name :");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();

    let v: e<String, i32> = e::a(line);
    match v {
        e::a(v) => {
            let x = v;
            println!("{:?}", x);
        }
        e::b(v) => {
            let x = v;
            println!("{:?}", x);
        }
    }
}