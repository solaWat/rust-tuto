fn add(x: i32, y: i32) -> i32 {
    x + y
}

// fn main() {
//     let added = add(10, 20);
//     println!("{}", added);

//     let z = 20;
//     let add_z = |x: i32| x + z;
//     println!("{}", add_z(10));
//     println!("Hello, world!");
// }

fn main() {
    {
        let x = String::from("hello");
        let y = x;

        println!("y:{}", y);
    }

    {
        let z = String::from("hello");
        {
            let w = &z;

            println!("w:{}", w);
        }
        println!("z:{}", z);
    }
}
