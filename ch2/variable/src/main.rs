fn main() {
    let mut x = 5;
    println!("val is {}", x);
    x = 6;
    println!("val is {}", x);
    prac_loop(4);
}

fn prac_loop(rep: u32)
{
    let mut idx = 0;
    while idx < rep
    {
        println!("again! {}", idx);
        idx += 1;
    }
    
    // using range
    for i in idx..idx + 4
    {
        println!("again! {}", i);
    }
}
