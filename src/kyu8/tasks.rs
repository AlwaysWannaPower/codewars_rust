// Grade book
// Complete the function so that it finds the average of the three scores passed to it and returns the letter value associated with that grade.
// Numerical Score	Letter Grade
// 90 <= score <= 100	'A'
// 80 <= score < 90	'B'
// 70 <= score < 80	'C'
// 60 <= score < 70	'D'
// 0 <= score < 60	'F'
// Tested values are all between 0 and 100. Theres is no need to check for negative values or values greater than 100.
pub fn get_grade(s1: u16, s2: u16, s3: u16) -> char {
    let avg = (s1 + s2 + s3) / 3;

    match avg {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => panic!("Invalid score"),
    }
}

// Write a function that removes the spaces from the string, then return the resultant string.

// Examples (Input -> Output):

// "8 j 8   mBliB8g  imjB8B8  jl  B" -> "8j8mBliB8gimjB8B8jlB"
// "8 8 Bi fk8h B 8 BB8B B B  B888 c hl8 BhB fd" -> "88Bifk8hB8BB8BBBB888chl8BhBfd"
// "8aaaaa dddd r     " -> "8aaaaaddddr"
pub fn no_space(x: String) -> String {
    x.replace(" ", "")
}

// Your goal is to return multiplication
// table for number that is always an integer from 1 to 10.

// For example, a multiplication table (string) for number == 5 looks like below:

// 1 * 5 = 5
// 2 * 5 = 10
// 3 * 5 = 15
// 4 * 5 = 20
// 5 * 5 = 25
// 6 * 5 = 30
// 7 * 5 = 35
// 8 * 5 = 40
// 9 * 5 = 45
// 10 * 5 = 50
// P. S. You can use \n in string to jump to the next line.

// Note: newlines should be added between rows, but there should be no trailing newline at the end. If you're unsure about the format, look at the sample tests.

pub fn multi_table(n: u64) -> String {
    (1..=10)
        .map(|i| format!("{} * {} = {}", i, n, i * n))
        .collect::<Vec<_>>()
        .join("\n") // join НЕ добавляет \n в конце
}

// Jenny has written a function that returns a greeting for a user. However, she's in love with Johnny, and would like to greet him slightly different. She added a special case to her function, but she made a mistake.

// Can you help her?

// fn greet(input : &str) -> String {
//   return format!("Hello, {}!", input);
//   if input == "Johnny" {
//     return "Hello, my love!".to_string();
//   };
// }
pub fn greet(input: &str) -> String {
    if input == "Johnny" {
        return "Hello, my love!".to_string();
    };

    return format!("Hello, {}!", input);
}

// Given three integers a, b, and c, return the largest number obtained after inserting the operators +, *, and parentheses (). In other words, try every combination of a, b, and c with the operators, without reordering the operands, and return the maximum value.

// Example
// With the numbers 1, 2, and 3, here are some possible expressions:

// 1 * (2 + 3) = 5
// 1 * 2 * 3 = 6
// 1 + 2 * 3 = 7
// (1 + 2) * 3 = 9
// The maximum value that can be obtained is 9.

// Notes
// The numbers are always positive, in the range 1 ≤ a, b, c ≤ 10.
// You can use the same operation more than once.
// It is not necessary to use all the operators or parentheses.
// You cannot swap the operands. For example, with the given numbers, you cannot get the expression (1 + 3) * 2 = 8.
// Input and Output Examples
// expressionsMatter(1, 2, 3) ==> 9, because (1 + 2) * 3 = 9.
// expressionsMatter(1, 1, 1) ==> 3, because 1 + 1 + 1 = 3.
// expressionsMatter(9, 1, 1) ==> 18, because 9 * (1 + 1) = 18.
pub fn expressions_matter(a: u64, b: u64, c: u64) -> u64 {
        let mut max = 0;
    
    // Все возможные комбинации без скобок (слева направо)
    max = max.max(a + b + c);
    max = max.max(a * b * c);
    max = max.max(a + b * c);
    max = max.max(a * b + c);
    
    // С скобками (приоритет сложения)
    max = max.max((a + b) * c);
    max = max.max(a * (b + c));
    
    max
}

// The function is not returning the correct values. Can you figure out why?

// Example (Input --> Output ):

// 3 --> "Earth"
pub fn get_planet_name(id: u32) -> String {
    match id {
        1 => "Mercury".to_string(),
        2 => "Venus".to_string(),
        3 => "Earth".to_string(),
        4 => "Mars".to_string(),
        5 => "Jupiter".to_string(),
        6 => "Saturn".to_string(),
        7 => "Uranus".to_string(),
        8 => "Neptune".to_string(),
        _ => unreachable!(),
    }
}

// Your task is to find the nearest square number of a positive integer n. In mathematics, a square number or perfect square is an integer that is the square of an integer; in other words, it is the product of some integer with itself.

// For example, if n = 111, then the nearest square number equals 121, since 111 is closer to 121, the square of 11, than 100, the square of 10.

// If n is already a perfect square (e.g. n = 144, n = 81, etc.), you need to just return n.

// Good luck :)
pub fn nearest_sq(n: u32) -> u32 {
let sqrt = (n as f32).sqrt();
    let lower = sqrt.floor() as u32;
    let upper = sqrt.ceil() as u32;
    
    let lower_sq = lower * lower;
    let upper_sq = upper * upper;
    
    // Выбираем ближайший, если равны - берем верхний (как в условии)
    if n - lower_sq <= upper_sq - n {
        lower_sq
    } else {
        upper_sq
    }
}
//   ((n as f64).sqrt().round() as u32).pow(2)

// Write a method, that will get an integer array as parameter and will process every number from this array.

// Return a new array with processing every number of the input-array like this:

// If the number has an integer square root, take this, otherwise square the number.

// Example
// [4,3,9,7,2,1] -> [2,9,3,49,4,1]
// Notes
// The input array will always contain only positive numbers, and will never be empty or null.
pub fn square_or_square_root(arr: &[u32]) -> Vec<u32> {
    arr.iter()
        .map(|&n| {
            let sqrt = (n as f32).sqrt();
            if sqrt.fract() == 0.0 {
                sqrt as u32
            } else {
                n * n
            }
        })
        .collect()
}
