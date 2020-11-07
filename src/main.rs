fn main() {
    let mut count=0u32;
    println!("Hello, world - to Infinity and Beyond!"); //I think this is an inf. loop
    loop{
        count +=1;
        if count == 24{
            println!("wanna know what's funnier than 24?");
            continue
        }
        if count == 25{
            println!("25....");
            continue
        }

        println!("{}",count);
        if count == 35{
            print!("k bye bby luv u");
            break;
        }
    }

}
