const PWD_START: u32 = 356261;
const PWD_END: u32 = 846303;

fn is_valid_pwd(number: &Vec<u32>) -> bool {
    let mut current = 0;
    let mut doubles = false;

    for num in number {
        if *num < current {
            return false;
        }

        if *num == current {
            doubles = true;
        }

        current = *num;
    }

    doubles
}

fn main() {
    let pwd_string = PWD_START.to_string();
    let mut number: Vec<u32> = pwd_string
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    let mut valid_pwd: Vec<Vec<u32>> = Vec::new();

    for i in PWD_START..PWD_END {
        if is_valid_pwd(&number) {
            valid_pwd.push(number.to_vec());
        }

        let mut carry = 1;
        let mut ind = number.len() - 1;

        while carry == 1 && ind >= 0 {
            if number[ind] < 9 {
                number[ind] += carry;
                carry = 0;
            } else {
                number[ind] = 0;
                ind -= 1;
            }
        }
    }

    println!("Part 1: {}", valid_pwd.len());
}
