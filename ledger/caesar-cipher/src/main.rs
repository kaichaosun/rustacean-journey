fn main() {
    println!("Start decrypt caesar cipher");
    
    // plain text: PENCIL PURSE ROOF SIREN
    let input = "NCLAGJ NSPQC PMMD QGPCL";

    

    let output = 
        input.chars().into_iter()
            .map(|c| match c { 
                ' ' => c,
                _ => std::char::from_u32(c as u32 + 2).unwrap()
            })
            .collect::<String>();

    println!("The plain text: {:?}", output);
}
