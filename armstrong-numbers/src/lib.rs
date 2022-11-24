pub fn is_armstrong_number(num: u32) -> bool {
    let num_digits = count_digits(num);

    let mut remainder = num;
    let mut sum = 0;

    while remainder > 0 {
        sum += (remainder % 10).pow(num_digits);
        remainder /= 10;
    }

    sum == num
}

fn count_digits(num: u32) -> u32 {
    let mut num_digits = 1;
    let mut remainder = num;

    while remainder > 10 {
        remainder /= 10;
        num_digits += 1;
    }

    num_digits
}
