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

    let number_list: Vec<f64> = number_list_string
        .split(",")
        .map(|item| item.parse().unwrap_or(0.0))
        .collect();

    let average = calculate_average(&number_list);
    let median = get_median(&number_list);
    let standard_deviation = calculate_standard_deviation(&number_list);
    // let range = calculate_range(&number_list);

    println!("The number average is: {}", average);
    println!("The number median is: {}", median);
    println!("The number standard deviation is: {}", standard_deviation);
    // println!("The number range is: {}", range);
}

fn calculate_average(numbers: &Vec<f64>) -> f64 {
    let number_quantity = numbers.len() as f64;

    let mut number_total: f64 = 0.0;

    for number in numbers {
        number_total += number;
    }

    number_total / number_quantity
}

fn get_median(numbers: &Vec<f64>) -> f64 {
    let mut numbers = numbers.clone();

    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let n = numbers.len();

    if n % 2 == 0 {
        let middle1 = numbers[(n - 1) / 2];
        let middle2 = numbers[n / 2];
        return (middle1 + middle2) / 2.0;
    } else {
        return numbers[n / 2];
    }
}

fn calculate_standard_deviation(numbers: &Vec<f64>) -> f64 {
    let media = calculate_average(numbers);

    let numbers = numbers
        .clone()
        .iter()
        .map(|number| (number - media).powf(2.0))
        .collect::<Vec<f64>>();

    let average = calculate_average(&numbers);

    average.sqrt()
}

fn calculate_range(numbers: &Vec<f64>) -> f64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media() {
        let numbers: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 6.0];
        assert_eq!(calculate_average(&numbers), 3.0);
    }

    #[test]
    fn test_median() {
        let numbers: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 6.0];
        assert_eq!(get_median(&numbers), 3.0);
    }

    #[test]
    fn test_standard_deviation() {
        let numbers = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 9.0];
        let std_dev = calculate_standard_deviation(&numbers);
        assert_eq!(std_dev, 2.0);
    }
}
