use std::error::Error;

fn check_input_only_paren(input: &String) -> Result<(), Box<dyn Error>> {
    if !input.chars().clone().all(|x| x == '(' || x == ')') {
        return Err("input contains non-paranthesis".into());
    }
    Ok(())
}

fn longest_parenthesis_easy_method(input: &String) -> Result<i32, Box<dyn Error>> {
    check_input_only_paren(input)?;
    let vec = input.as_bytes();
    let mut max: usize = 0;
    let mut last_index: usize = 0;
    let mut tmp: i32 = 0;

    let mut i = 0;
    while i < vec.len() {
        if vec[i] == b')' {
            i += 1;
            continue;
        }
        tmp = 1;
        last_index = i;
        for j in i + 1..vec.len() {
            if vec[j] == b')' {
                tmp -= 1;
            } else {
                tmp += 1;
            }
            if tmp == 0 {
                // println!("i: {i}, j: {j}");
                max = std::cmp::max(max, 1 + j - i);
                last_index = j;
            } else if tmp < 0 {
                i += 1;
                break;
            }
        }

        if last_index != i {
            i = last_index;
        } else {
            i += 1
        }
    }
    Ok(max.try_into().unwrap())
}

fn longest_parenthesis(input: &String) -> Result<i32, Box<dyn Error>> {
    check_input_only_paren(input)?;
    let iter = input.chars();
    let mut max: i32 = 0;
    let mut cur_max: i32 = 0;
    let mut tmp: i32 = 0;

    for i in iter {
        match i {
            '(' => tmp += 1,
            _ => tmp -= 1,
        };

        if tmp > 0 {
            cur_max += 1;
        } else if tmp == 0 {
            cur_max += 1;
            max = std::cmp::max(max, cur_max);
        } else {
            tmp = 0
        }
        max = std::cmp::max(max, cur_max - tmp);
    }
    Ok(max)
}

pub fn longest_valid_parentheses(s: &String) -> i32 {
    let mut longest = 0;
    let mut current = 0;
    let mut stack = Vec::new();

    for x in s.chars() {
        match x {
            '(' => {
                stack.push(current);
                current = 0;
            }
            _ => match stack.pop() {
                Some(val) => {
                    current = current + val + 2;
                    if current > longest {
                        longest = current;
                    }
                }
                _ => {
                    stack.clear();
                    current = 0
                }
            },
        }
    }

    if current > longest {
        current
    } else {
        longest
    }
}

fn main() {
    let parenthesis = String::from("((())((((()))");
    let res = longest_parenthesis_easy_method(&parenthesis)
        .expect("Failed to Execute longest_parenthesis");
    println!("{}", res);
    let res = longest_parenthesis(&parenthesis).expect("Failed to Execute longest_parenthesis");
    println!("{}", res);
    let res = longest_valid_parentheses(&parenthesis);
    println!("{res}");
}
