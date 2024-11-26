
// // // // // // use std::io;//input and output 

// // // // // // fn main() {
// // // // // //     println!("enetr your size of array");
// // // // // //     let mut size = String::new();//6
// // // // // //     io::stdin().read_line(&mut size).unwrap(); // 6 
// // // // // //     let size = size.trim().parse().expect("give correct type in size");

// // // // // //     println!("fill our array of elements");
// // // // // //     let mut array = Vec::new();
// // // // // //     for _  in 1..=size {
// // // // // //         let mut element = String::new();
// // // // // //         io::stdin().read_line(&mut element).unwrap();
// // // // // //         let element: usize = element.trim().parse().expect("enter valid elements");
// // // // // //         array.push(element);// 1,2,3,4,5..
// // // // // //     }

// // // // // //     let mut sum = 0;
// // // // // //     for &i in &array {
// // // // // //         sum += i;
// // // // // //     }

// // // // // //     println!("Sum:  {}", sum);
// // // // // // }


// // // // // use std::io;//input and output 

// // // // // fn main() {
// // // // //     //Alice array
// // // // //     println!("enetr your size of Alice's array");
// // // // //     let mut size_alice = String::new();
// // // // //     io::stdin().read_line(&mut size_alice).unwrap(); // 3 
// // // // //     let size_alice = size_alice.trim().parse().expect("give correct type in size");

// // // // //     //Bob array
// // // // //     println!("enetr your size of Bob's array");
// // // // //     let mut size_bob = String::new();
// // // // //     io::stdin().read_line(&mut size_bob).unwrap(); // 3 
// // // // //     let size_bob = size_bob.trim().parse().expect("give correct type in size");

// // // // //     //Filling Alice's array
// // // // //     println!("fill Alice's array of elements");
// // // // //     let mut array_alice = Vec::new();
// // // // //     for _ in 1..=size_alice {
// // // // //         let mut element_a = String::new();
// // // // //         io::stdin().read_line(&mut element_a).unwrap();
// // // // //         let element_a: usize = element_a.trim().parse().expect("enter valid elements");
// // // // //         array_alice.push(element_a);// 1,2,3,4,5..
// // // // //     }

// // // // //      //Filling Bob's array
// // // // //      println!("fill Alice's array of elements");
// // // // //      let mut array_bob = Vec::new();
// // // // //      for _ in 1..=size_bob {
// // // // //          let mut element_b = String::new();
// // // // //          io::stdin().read_line(&mut element_b).unwrap();
// // // // //          let element_b: usize = element_b.trim().parse().expect("enter valid elements");
// // // // //          array_bob.push(element_b);// 1,2,3,4,5..
// // // // //      }

// // // // //      if size_alice != size_bob {
// // // // //         println!("enter correct sizes for person arrays, there size shoulda be equale");
// // // // //         return;
// // // // //      }

// // // // //      let mut counter_alice = 0;
// // // // //      let mut counter_bob = 0;

// // // // //      for i in 0..size_alice {
// // // // //         if array_alice[i] > array_bob[i] {
// // // // //             counter_alice +=1;
// // // // //         } 
// // // // //         else if array_bob[i] > array_alice[i] {
// // // // //             counter_bob +=1;
// // // // //         }
// // // // //      }

// // // // //     println!("Score Alice {} : Bob {}", counter_alice, counter_bob);
// // // // // }


// // // // use std::io;

// // // // fn main() {
// // // //     // Get the number of arrays
// // // //     println!("Enter the number of arrays:");
// // // //     let mut n_input = String::new();
// // // //     io::stdin().read_line(&mut n_input).unwrap();
// // // //     let n: usize = n_input
// // // //         .trim()
// // // //         .parse()
// // // //         .expect("Please enter a valid number of arrays.");

// // // //     let mut cumulative_sum = [0i64; 10];

// // // //     for i in 0..n {
// // // //         println!("Enter 10-digit array {}:", i + 1);

// // // //         loop {
// // // //             let mut line = String::new();
// // // //             io::stdin().read_line(&mut line).unwrap();
// // // //             let line = line.trim(); 

// // // //             if line.len() == 10 && line.chars().all(|c| c.is_digit(10)) {
// // // //                 for j in 0..10 {

// // // //                     let digit = line.chars().nth(j).unwrap().to_digit(10).unwrap() as i64;
// // // //                     cumulative_sum[j] += digit;
// // // //                 }
// // // //                 break;
// // // //             } else {
// // // //                 println!("Error: Input must be exactly 10 numeric digits.");
// // // //             }
// // // //         }
// // // //     }

// // // //     let mut result_number = 0;
// // // //     for j in 0..10 {
// // // //         result_number = result_number * 10 + cumulative_sum[j];
// // // //     }

// // // //     println!("Result : {}", result_number);
// // // // }


// // // use std::io;

// // // fn main() {
// // //     println!("enter size of matrix");
// // //     let mut size = String::new();
// // //     io::stdin().read_line(&mut size).unwrap();
// // //     let size: usize = size.trim().parse().expect("failed size of matrix");

// // //     println!("filling your matrix: ");
// // //     let mut matrix: Vec<Vec<i64>> = Vec::new();
// // //     for _ in 0..size {
// // //         let mut element_matrix = String::new();
// // //         io::stdin().read_line(&mut element_matrix).unwrap();
// // //         let element_matrix: Vec<i64> = element_matrix
// // //         .trim()
// // //         .split_whitespace()
// // //         .map(|x| x.parse().expect("failed type data in matrix"))
// // //         .collect();

// // //         if element_matrix.len() != size {
// // //             panic!("each row and column in matrix should have exactly elements");
// // //         }

// // //         matrix.push(element_matrix);
// // //     }

// // //     let mut diagonal_right = 0;
// // //     let mut diagonal_left = 0;
// // //     for i in 0..size {
// // //         diagonal_right += matrix[i][i];
// // //         diagonal_left += matrix[i][size - i - 1];
// // //     }

// // //     let difference = (diagonal_right - diagonal_left).abs();
// // //     println!("right diagonal: {} ", diagonal_right);
// // //     println!("left diagonal: {}", diagonal_left);
// // //     println!("diff: {}", difference);
// // // }


// // // use std::io;

// // // fn main() {
// // //     println!("enter size of array");
// // //     let mut size = String::new();
// // //     io::stdin().read_line(&mut size).unwrap();
// // //     let size: usize = size.trim().parse().expect("failed size of matrix");

// // //     println!("filling your array: ");
// // //     let mut array = Vec::new();
// // //     for _ in 0..size {
// // //         let mut element = String::new();
// // //         io::stdin().read_line(&mut element).unwrap();
// // //         let element: i32 = element
// // //         .trim()
// // //         .parse()
// // //         .expect("inccorect type in array");
// // //         array.push(element);
// // //     }

// // //     let mut positive = 0;
// // //     let mut negative = 0;
// // //     let mut zero = 0;
// // //     for &i in &array {
// // //         if i < 0 {
// // //             negative+=1;
// // //         }
// // //         else if i > 0 {
// // //             positive+=1;
// // //         }
// // //         else {
// // //             zero +=1;
// // //         }
// // //     }

// // //     let positive_res = (positive as f64) / (size as f64);
// // //     let negative_res = (negative as f64) / (size as f64);
// // //     let zero_res = (zero as f64) / (size as f64);
// // //     println!("ressult: {:.6}, {:.6}, {:.6}",positive_res, negative_res,zero_res);
// // // }


// // use std::io;

// // fn main() {
// //     println!("enter size of array");
// //     let mut size = String::new();
// //     io::stdin().read_line(&mut size).unwrap();
// //     let size: usize = size.trim().parse().expect("failed size of arrat");
// //     //4 1-4 = 3  +1 = 4
// //     for i in 1..size+1 {
// //         let mut line = String::new();
// //         for _ in 0..(size-i) {
// //             line.push(' ');
// //         }
// //         for _ in 0..i {
// //             line.push('#');
// //         }
// //         println!("{}", line);
// //     }
    
// // }


// use std::io;

// fn main() {
//     println!("enter size of array");
//     let mut size = String::new();
//     io::stdin().read_line(&mut size).unwrap();
//     let size: usize = size.trim().parse().expect("failed size of arrat");
 
//     println!("filling your array: ");
//     let mut array = Vec::new();
//     for _ in 0..size {
//         let mut element = String::new();
//         io::stdin().read_line(&mut element).unwrap();
//         let element: i32 = element
//         .trim()
//         .parse()
//         .expect("failed type of array");
//         array.push(element);
//     }

//     let mut min_element = array[0];
//     let mut max_element = array[0];

//     if array.len() < 2 {
//         println!("enter more elements");
//     }

//     for &i in &array {
//         if i < min_element {
//             min_element = i;
//         }
//         if i >  max_element {
//             max_element = i;
//         }
//     }

//     let sum: i32 = array.iter().sum();
//     let sum_min = sum - min_element;
//     let sum_max = sum - max_element;

//     println!("sum without max element: {}", sum_max);
//     println!("sum without min element: {}", sum_min);

// }