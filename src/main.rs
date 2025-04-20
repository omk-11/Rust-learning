fn main() {
    print!("{}",print_fibonachi(10));
}

fn print_fibonachi(num: i32) -> i32{
    let mut first = 1;
    let mut second = 0;

    if num == 0{
        return 0;
    }

    if num ==1 {
        return 1;
    }
    
    for _i in 1..num{
        let temp = second;
        second = first + second;
        first = temp;
    }
    return second;
}


