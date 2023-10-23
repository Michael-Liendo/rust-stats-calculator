use std::io::stdin;

fn main() {
    let mut number_list_string = String::new();
    println!("Introduce a number lists separates with colon:");

    // Get the list of numbers
    stdin()
        .read_line(&mut number_list_string)
        .expect("Did not enter a correct string");

    if number_list_string.len() < 1 {
        println!("You need enter numbers");
    }

    number_list_string = number_list_string.trim().to_string();

    let number_list: Vec<usize> = number_list_string
        .split(",")
        .map(|item| item.parse().unwrap_or(0))
        .collect();

    let average = calculate_average(number_list);

    println!("The number average is: {}", average)
}

fn calculate_average(numbers: Vec<usize>) -> usize {
    let number_quantity = numbers.len();

    let mut number_total: usize = 0;

    for number in numbers {
        number_total += number;
    }

    number_total / number_quantity
}

fn get_median(numbers: Vec<usize>) -> usize {
    todo!()
}

fn calculate_standard_deviation(numbers: Vec<usize>) -> usize {
    todo!()
}

fn calculate_range(numbers: Vec<usize>) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media() {
        let numbers: Vec<usize> = vec![1, 2, 3, 4, 5];
        assert_eq!(calculate_average(numbers), 3);
    }
}
