fn main() {
    let x =  false;
    let y = true;
    
    match (x , y){
        (true,false) => println!("x is true and y is false"),
        (true,true) => println!("x is true and y is true"),
        (false,false) => println!("x is false and y is false"),
        _ => println!("Invalid Combination"),
        
    }
}
