use collections_ch8::vectors;

fn main() {
    // Vectors - Median and Mode
    let vec = vec![9, 5, 2, 9, 4, 3, 2, 2, 1];

    let median = vectors::find_median(&vec);
    match median {
        Some(x) => println!("Found median: {}", x),
        None => println!("No median found!"),
    }

    let mode = vectors::find_mode(&vec);
    match mode {
        Some(x) => println!("Found mode: {}", x),
        None => println!("No mode found!"),
    }
}
