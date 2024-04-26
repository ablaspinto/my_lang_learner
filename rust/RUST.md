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

> i8 8-bit SIGNED integer, i-16 16-bit SIGNED integer, i-32 32-bit SIGNED integer, i-64 64-bit SIGNED integer, i-128 128-bit SIGNED integer 
### Raw, unsafe pointers *const T, *mut T
* let my_num: i32 = 10; 
let my_num_ptr   *const i32 = &my_num


> Reference &T &mut T 
> Slice [T] 
> Tuple  (E, T , ... ) finit sequence 
> u16, u32, u64, u128 UNSIGNED integers with their respective bit sizes
> unit () 
> usize - pointer sized unsigned integer type 


## Modules: 
 alloc - memory allocation api 
 any - used for dynamic typing  --> not as good as statically typed rust 
 arch SIMD and vendor intrinsics module 
 
### array  - Module core::array 
* from_fn: 
> creates an array of type[T;N] where each T element is the returned value from cb which  is using that elements index  
> 
> cb - callback where the passed argument is the current array index 

* from_mut 
> converts mutuable reference to T into a mutable reference to any array of length 1 (does not copy )

* from_ref 
> converts a reference to T into a reference to any array of length 1 (does not copy )


* 
    
 
