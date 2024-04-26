# RUST LANG DOC 


## Primitive Types: 

### Arrays 

* ARRAY SYNTAX: let mut array[T;N]  T is element type, N is constant size
EX: 
let mut array[i32: 3 ] = [0;3 ] ;
    for x in  array{
        print!("{x}");
}

EX MAPPING: 

 let a = ["1","2","3"]

 let b = a.try_map(|v| v.parse::<u32>.unwrap.map(|v| v + 1))

 ---

* bool : let b = true;

### fn - function pointers syntax fn(usize) -> bool 
* FUNCTION POINTERS SYNTAX: 
EX: 
 fn add_one(x: usize) -> usize {
    x + 1
}
> i8 8-bit SIGNED integer 
> i-16 16-bit SIGNED integer 
> i-32 32-bit SIGNED integer 
> i-64 64-bit SIGNED integer 
> i-128 128-bit SIGNED integer 
> isize pointer sized signed integer type 
> Raw, unsafe pointers *const T, *mut T
> Reference &T &mut T 
> Slice [T] 
> Tuple  (E, T , ... ) finit sequence 
> u16, u32, u64, u128 UNSIGNED integers with their respective bit sizes
> unit () 
> usize - pointer sized unsigned integer type 


*Modules: 
> alloc - 
