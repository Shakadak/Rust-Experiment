fn main()
{
    for n in range(1u, 101u)
    {
        match n
        {
            n if n % 15 == 0 => println!("FizzBuzz"),
            n if n % 3 == 0 => println!("Fizz"),
            n if n % 5 == 0 => println!("Buzz"),
            _ => println!("{}", n),
        }
    }
}
