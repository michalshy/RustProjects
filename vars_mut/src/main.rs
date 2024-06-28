const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;

fn main() {
    { 
        let mut x = 5;
        println!("The value of x is: {x}");
        x = 6;
        println!("The value of x is: {x}");
    }

    {
        let y = 5;
        let y = y + 5;
        {
            let y = y * THREE_HOURS_IN_SECONDS;
            println!("The value of y in the inner scope is: {y}");
        }
        println!("The value of x is: {y}");
    }

    {
        let spaces = "   ";
        let spaces = spaces.len();
    }


}
