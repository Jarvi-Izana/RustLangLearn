fn main()
{
    // immutable by default
    let mut s1: String = String::from("guihao");
    s1 = pass_string_and_return(s1);
    println!("ch4 >>> {}", s1);
    mut_string(&mut s1);
    mut_string(&mut s1);
    println!("ch4 >>> {}", s1);
}

fn pass_string_and_return(s: String) -> String
{
    return s;
}

fn mut_string(s: &mut String)
{
    s.push_str("hehe");
}
