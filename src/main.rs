mod inputs;

fn main() {
    println!("Choose two numbers:");
    let x = inputs::return_float();
    let y = inputs::return_float();

    println!("Welocme to the Rust Calculator!!");
    println!("1 -> Addition (+)");
    println!("2 -> Subtraction (-)");
    println!("3 -> Multiptly (*)");
    println!("4 -> Division (/)");
    println!("Choose an operation:");
    
    let choice= inputs::return_int();
    if choice == 1{
        let result = addition(x, y);
        println!("The result of the two values is {}\n", result);
    }
    else if choice == 2{
        let result = subtraction(x, y);
        println!("The result of the two values is {}\n", result);
    }
    else if choice == 3{
        let result = multiply(x, y);
        println!("The result of the two values is {}\n", result);
    } 
    else if choice == 4{
        let result = division(x, y);
        println!("The result of the two values is {}\n", result);
    }
    else{
        print!("Invalid Operation")
    }
    
}
fn addition(x:f32,y:f32)->f32{
    let result = x + y;
    return result;
}
fn multiply(x:f32,y:f32)->f32{
    let result = x * y;
    return result;
}
fn subtraction(x:f32,y:f32)->f32{
    let result = x - y;
    return result;
}
fn division(x:f32,y:f32)->f32{
    let result = x / y;
    return result;
}