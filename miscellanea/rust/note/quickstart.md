# Rust Quick Start and Comparison With other Languages

## Printing

```
let vec : Vec<u32> = vec!(1,2,3);
dbg!(&vec);  // pass reference to debug
println!("{:?}", vec);  // println automatically take reference
```

## Control Flow

Rust does not have a function like `range` in python. 
Nor does rust support C styled `for (int i=0; i<10; ++i)`.

Instead, rust use iterators.

```
// iterator does not include 0 but not 5. 
// to include 5 use `0..=5`
for i in 0..5 {
    print!("{} ", i); //0, 1, 2, 3, 4
}

// reverse iterator 
for i in (0..5).rev() {
    print!("{} ", i); //4, 3, 2, 1, 0
}
```
