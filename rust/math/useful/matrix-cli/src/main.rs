/* 
    This program is a cli application that allows creation and modification, and
    serialization of matrices. It also allows for the useage of matrix 
    operations.
*/

use std::io::stdin;
use ndarray::Array2;
use ndarray_linalg::solve::Determinant;

struct Matrix {
    name: String,
    matrix: Array2<i32>,
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.matrix)
    }
}

fn main() {
    let mut input = String::new();
    let mut matrices: Vec<Matrix> = Vec::new();
    let stdin = stdin();

    println!("Welcome to the Matrix CLI!");
    println!("Type 'help' for a list of commands.");

    loop {
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "help" => {
                println!("help: Displays this message.");
                println!("list: Lists all matrices.");

                println!("new: Creates a new matrix. Takes 3 arguments: name, rows, columns.");
                println!("del: Deletes a matrix. Takes 1 argument: matrix name.");
                println!("set: Sets a value in a matrix. Takes 4 arguments: matrix name, row, column, value.");
                println!("get: Gets a value from a matrix. Takes 3 arguments: matrix name, row, column.");
                println!("print: Prints a matrix. Takes 1 argument: matrix name.");
                println!("clear: Deletes all matrices. Takes no arguments.");

                println!("add: Adds two matrices. Takes 3 arguments: 1st matrix name, 2nd matrix name, 3rd matrix name.");
                println!("sub: Subtracts two matrices. Takes 3 arguments: 1st matrix name, 2nd matrix name, 3rd matrix name.");
                println!("scalarmult: Multiplies a matrix by a scalar. Takes 3 arguments: matrix name, scalar, new matrix name.");
                println!("scalardiv: Divides two matrices. Takes 3 arguments: 1st matrix name, 2nd matrix name, 3rd matrix name.");
                println!("mul: Multiplies two matrices. Takes 3 arguments: 1st matrix name, 2nd matrix name, 3rd matrix name.");

                println!("determinant: Calculates the determinant of a matrix. Takes 2 arguments: matrix name, new matrix name.");
                println!("inverse: Calculates the inverse of a matrix. Takes 2 arguments: matrix name, new matrix name.");
                println!("transpose: Calculates the transpose of a matrix. Takes 2 arguments: matrix name, new matrix name.");
                println!("ref: Calculates the reduced echelon form of a matrix. Takes 2 arguments: matrix name, new matrix name.");
                println!("rref: Calculates the reduced row echelon form of a matrix. Takes 2 arguments: matrix name, new matrix name.");

                println!("save: Saves a matrix to a file. Takes 2 arguments: matrix name, file name. (Each matrix is save as one file)");
                println!("load: Loads a matrix from a file. Takes 2 arguments: matrix name, file name. (Each matrix is save as one file)");
                
                println!("exit: Exits the program.");
            },
            "list" => {
                println!("List of matrices:");
                if matrices.is_empty() {
                    println!("No matrices.");
                } else {
                    for matrix in &matrices {
                        println!("{}, {}", matrix.name, matrix);
                    }
                }
            },

            "new" => {
                let mut name = String::new();
                let mut rows = String::new();
                let mut columns = String::new();

                println!("Enter the name of the matrix:");
                stdin.read_line(&mut name).unwrap();
                let name = name.trim();

                println!("Enter the number of rows:");
                stdin.read_line(&mut rows).unwrap();
                let rows: usize = rows.trim().parse().unwrap();

                println!("Enter the number of columns:");
                stdin.read_line(&mut columns).unwrap();
                let columns: usize = columns.trim().parse().unwrap();

                let matrix = Array2::zeros((rows, columns));
                matrices.push(Matrix { name: name.to_string(), matrix });
            },
            "del" => {
                let mut name = String::new();
                
                println!("Enter the name of the matrix:");
                stdin.read_line(&mut name).unwrap();
                let name = name.trim();
                
                for i in 0..matrices.len() {
                    if matrices[i].name == name {
                        matrices.remove(i);
                    }
                }
            },
            "set" => {
                let mut name = String::new();
                let mut row = String::new();
                let mut column = String::new();
                let mut value = String::new();

                println!("Enter the name of the matrix:");
                stdin.read_line(&mut name).unwrap();
                let name = name.trim();

                println!("Enter the row:");
                stdin.read_line(&mut row).unwrap();
                let row: usize = row.trim().parse().unwrap();

                println!("Enter the column:");
                stdin.read_line(&mut column).unwrap();
                let column: usize = column.trim().parse().unwrap();

                println!("Enter the value:");
                stdin.read_line(&mut value).unwrap();
                let value: i32 = value.trim().parse().unwrap();

                for matrix in &mut matrices {
                    if matrix.name == name {
                        matrix.matrix[[row, column]] = value;
                    }
                }
            },
            "get" => {
                let mut name = String::new();
                let mut row = String::new();
                let mut column = String::new();

                println!("Enter the name of the matrix:");
                stdin.read_line(&mut name).unwrap();
                let name = name.trim();

                println!("Enter the row:");
                stdin.read_line(&mut row).unwrap();
                let row: usize = row.trim().parse().unwrap();

                println!("Enter the column:");
                stdin.read_line(&mut column).unwrap();
                let column: usize = column.trim().parse().unwrap();

                for matrix in &matrices {
                    if matrix.name == name {
                        println!("{}", matrix.matrix[[row, column]]);
                    }
                }
            },
            "print" => {
                let mut name = String::new();

                println!("Enter the name of the matrix:");
                stdin.read_line(&mut name).unwrap();
                let name = name.trim();

                for matrix in &matrices {
                    if matrix.name == name {
                        println!("{}", matrix);
                    }
                }
            },
            "clear" => {
                matrices.clear();
            },

            "add" => {
                let mut name1 = String::new();
                let mut name2 = String::new();
                let mut name3 = String::new();

                println!("Enter the name of the first matrix:");
                stdin.read_line(&mut name1).unwrap();
                let name1 = name1.trim();

                println!("Enter the name of the second matrix:");
                stdin.read_line(&mut name2).unwrap();
                let name2 = name2.trim();

                println!("Enter the name of the new matrix:");
                stdin.read_line(&mut name3).unwrap();
                let name3 = name3.trim();

                let mut matrix1 = Array2::zeros((0, 0));
                let mut matrix2 = Array2::zeros((0, 0));

                for matrix in &matrices {
                    if matrix.name == name1 {
                        matrix1 = matrix.matrix.clone();
                    } else if matrix.name == name2 {
                        matrix2 = matrix.matrix.clone();
                    }
                }

                // make sure they are the same size
                if matrix1.shape() != matrix2.shape() {
                    println!("Matrices are not the same size.");
                    continue;
                }

                let matrix3 = matrix1 + matrix2;
                matrices.push(Matrix { name: name3.to_string(), matrix: matrix3 });
            },
            "sub" => {
                let mut name1 = String::new();
                let mut name2 = String::new();
                let mut name3 = String::new();

                println!("Enter the name of the first matrix:");
                stdin.read_line(&mut name1).unwrap();
                let name1 = name1.trim();

                println!("Enter the name of the second matrix:");
                stdin.read_line(&mut name2).unwrap();
                let name2 = name2.trim();

                println!("Enter the name of the new matrix:");
                stdin.read_line(&mut name3).unwrap();
                let name3 = name3.trim();

                let mut matrix1 = Array2::zeros((0, 0));
                let mut matrix2 = Array2::zeros((0, 0));

                for matrix in &matrices {
                    if matrix.name == name1 {
                        matrix1 = matrix.matrix.clone();
                    } else if matrix.name == name2 {
                        matrix2 = matrix.matrix.clone();
                    }
                }

                // make sure they are the same size
                if matrix1.shape() != matrix2.shape() {
                    println!("Matrices are not the same size.");
                    continue;
                }

                let matrix3 = matrix1 - matrix2;
                matrices.push(Matrix { name: name3.to_string(), matrix: matrix3 });
            },
            "scalarmult" => {
                let mut name1 = String::new();
                let mut name2 = String::new();
                let mut scalar = String::new();

                println!("Enter the name of the matrix:");
                stdin.read_line(&mut name1).unwrap();
                let name1 = name1.trim();

                println!("Enter the name of the new matrix:");
                stdin.read_line(&mut name2).unwrap();
                let name2 = name2.trim();

                println!("Enter the scalar:");
                stdin.read_line(&mut scalar).unwrap();
                let scalar: i32 = scalar.trim().parse().unwrap();

                let mut matrix1 = Array2::zeros((0, 0));

                for matrix in &matrices {
                    if matrix.name == name1 {
                        matrix1 = matrix.matrix.clone();
                    }
                }

                let matrix2 = matrix1 * scalar;
                matrices.push(Matrix { name: name2.to_string(), matrix: matrix2 });
            },
            "scalardiv" => {
                let mut name1 = String::new();
                let mut name2 = String::new();
                let mut scalar = String::new();

                println!("Enter the name of the matrix:");
                stdin.read_line(&mut name1).unwrap();
                let name1 = name1.trim();

                println!("Enter the name of the new matrix:");
                stdin.read_line(&mut name2).unwrap();
                let name2 = name2.trim();

                println!("Enter the scalar:");
                stdin.read_line(&mut scalar).unwrap();
                let scalar: i32 = scalar.trim().parse().unwrap();

                let mut matrix1 = Array2::zeros((0, 0));

                for matrix in &matrices {
                    if matrix.name == name1 {
                        matrix1 = matrix.matrix.clone();
                    }
                }

                let matrix2 = matrix1 / scalar;
                matrices.push(Matrix { name: name2.to_string(), matrix: matrix2 });
            },
            "mult" => {
                let mut name1 = String::new();
                let mut name2 = String::new();
                let mut name3 = String::new();

                println!("Enter the name of the first matrix:");
                stdin.read_line(&mut name1).unwrap();
                let name1 = name1.trim();

                println!("Enter the name of the second matrix:");
                stdin.read_line(&mut name2).unwrap();
                let name2 = name2.trim();

                println!("Enter the name of the new matrix:");
                stdin.read_line(&mut name3).unwrap();
                let name3 = name3.trim();

                let mut matrix1 = Array2::zeros((0, 0));
                let mut matrix2 = Array2::zeros((0, 0));

                for matrix in &matrices {
                    if matrix.name == name1 {
                        matrix1 = matrix.matrix.clone();
                    } else if matrix.name == name2 {
                        matrix2 = matrix.matrix.clone();
                    }
                }

                // make sure they are the same size
                if matrix1.shape() != matrix2.shape() {
                    println!("Matrices are not the same size.");
                    continue;
                }

                let matrix3 = matrix1.dot(&matrix2);
                matrices.push(Matrix { name: name3.to_string(), matrix: matrix3 });
            },

            "determinant" => {
                let mut name1 = String::new();
                let mut name2 = String::new();

                println!("Enter the name of the matrix:");
                stdin.read_line(&mut name1).unwrap();
                let name1 = name1.trim();

                println!("Enter the name of the new matrix:");
                stdin.read_line(&mut name2).unwrap();
                let name2 = name2.trim();

                let mut matrix1 = Array2::zeros((0, 0));

                for matrix in &matrices {
                    if matrix.name == name1 {
                        matrix1 = matrix.matrix.clone();
                    }
                }

                let matrix2 = matrix1.det();
                matrices.push(Matrix { name: name2.to_string(), matrix: matrix2 });
            },

            "exit" => {
                println!("Exiting...");
                return;
            },

            _ => {
                println!("Invalid command. Type 'help' for a list of commands.");
            }
        }
    }
}
