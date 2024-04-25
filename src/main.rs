fn main()
{
    let x = 23;
    println!("x = {}", x);

    // tipo i64 - inteiro de 64 bits
    let y: i64 = 24;
    println!("y: i64 = {}", y);

    // tipo i8 - inteiro de 8 bits
    let z: i8 = 25;
    println!("z: i8 = {}", z);

    // unsigned type: sem sinal, logo, sempre positivo.
    let a: u8 = 26;
    println!("a: u8 = {}", a);

    // bool
    let b: bool = false;
    println!("b: bool = {}", b);

    let c: bool = true;
    println!("c: bool = {}", c);

    // não funciona
    // let d: bool = 0;
    // println!("d: bool = {}", d);

    // não funciona
    // let e: bool = 1;
    // println!("e: bool = {}", e);

    let f: f32 = 15.65;
    println!("f: f32 = {}", f);

}
