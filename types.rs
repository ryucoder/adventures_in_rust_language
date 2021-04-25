/* Primitive Types 
Integers 
Unsigned - u8, u16, u32, u64, u128
Signed - i8, i16, i32, i64, 128

Flaots f32, f64

Boolean bool 

Characters char

Tuples 

Arrays 


*/ 

pub fn run(){
    let i = 1;
    
    let f = 2.5;
    
    let y: i64 = 234234234234;
    let b: bool = false;
    let c: bool = 9 > 3 ;
    let d: char = 'a';
    let face = '\u{1F600}';

    println!("Max i32 {}", std::i32::MAX );
    println!("Max i64 {}", std::i64::MAX );
    println!("Boolean {}", b );
    println!("Boolean {}", c );
    println!("Boolean {}", d );
    println!("Boolean {}", face );

}