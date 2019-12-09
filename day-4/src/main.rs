const PWD_START: u32 = 356261;
const PWD_END: u32 = 846303;

fn is_valid_pwd_1(number: &Vec<u32>) -> bool {
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

fn is_valid_pwd_2(number: &Vec<u32>) -> bool {
    let mut current = 0;
    let mut num_current_repeating = 1;

    for num in number {
        if *num == current {
            num_current_repeating += 1;
        } else {
            if num_current_repeating == 2 {
                return true;
            }

            current = *num;
            num_current_repeating = 1;
        }
    }

    num_current_repeating == 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_pwd_2() {
        let case_1: Vec<u32> = vec![1, 1, 2, 2, 3, 3];
        let case_2: Vec<u32> = vec![1, 2, 3, 4, 4, 4];
        let case_3: Vec<u32> = vec![1, 1, 1, 1, 2, 2];
        let case_4: Vec<u32> = vec![1, 1, 1, 3, 5, 5];
        let case_5: Vec<u32> = vec![3, 4, 4, 5, 5, 5];
        let case_6: Vec<u32> = vec![3, 3, 4, 5, 5, 5];

        assert!(is_valid_pwd_2(&case_1));
        assert!(!is_valid_pwd_2(&case_2));
        assert!(is_valid_pwd_2(&case_3));
        assert!(is_valid_pwd_2(&case_4));
        assert!(is_valid_pwd_2(&case_5));
        assert!(is_valid_pwd_2(&case_6));
    }
}

fn main() {
    let pwd_string = PWD_START.to_string();
    let mut number: Vec<u32> = pwd_string
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    let mut num_valid_pwd_1 = 0;
    let mut num_valid_pwd_2 = 0;

    for i in PWD_START..PWD_END {
        if is_valid_pwd_1(&number) {
            num_valid_pwd_1 += 1;

            if is_valid_pwd_2(&number) {
                num_valid_pwd_2 += 1;
            }
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

    println!("Part 1: {}", num_valid_pwd_1);
    println!("Part 2: {}", num_valid_pwd_2);
}
