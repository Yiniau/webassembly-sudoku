
extern crate rand;
#[macro_use]
extern crate lazy_static;

#[allow(dead_code)]
extern {
    fn log_true();
    fn log_false();
    fn log_num(num: usize);
    fn log_matrix(ptr: *mut usize);
    fn log_ptr(ptr: *mut usize);

    fn random_int(min: usize, max: usize) -> usize;
}

mod wasm_mem;
pub use wasm_mem::*;

use std::os::raw::c_void;
//use std::os::raw::c_int;
use std::sync::Mutex;

/// Return a random usize number with given bound
/// e.g. s = 1, e = 5, return [1,5) usize
///
/// # examples
///
/// println!("this output number should between [1, 5], result: {}", random_num(1,5));
pub fn random_num(s: usize, e: usize) -> usize {
//    use rand::random;
//    random::<usize>() % e + s

    unsafe { random_int(s, e) }
}


/// sudoku game matrix
///
/// # Porps
///
/// * `content` a vec two-dimensional array like 9x9 matrix
///
/// # example
///
///
pub struct SudokuMatrix {
    data: [u8; 81]
}

impl SudokuMatrix {

    /// generate 9*9 matrix from data
    fn get_matrix_from_data(&self) -> Vec<Vec<u8>> {
        let data = &self.data;
        let mut m: Vec<Vec<u8>> = vec![vec![0; 9]; 9];
        for y in 0..9 {
            for x in 0..9 {
                m[y][x] = data[x + (y * 9)];
            }
        }
        m
    }

    /// Check if there is is any duplication in given array
    fn arr_repeat_check(arr: &Vec<u8>) -> bool {
        let len: usize = arr.len();
        for i in 0..(len - 1) {
            for j in (i + 1)..len {
                if arr[i] != 0 && arr[i] == arr[j] {
                    return false;
                }
            }
        }
        true
    }

    /// Check is the matrix legal
    /// return true if matrix legal
    ///
    /// # examples
    ///
    /// ```
    /// #[cfg(test)]
    /// mod tests {
    ///     use super::*;
    ///
    ///     #[test]
    ///     fn matrix_check_works() {
    ///         let test_example = vec![
    ///             vec![1,2,3,4,5,6,7,8,9],
    ///             vec![4,5,6,7,8,9,1,2,3],
    ///             vec![7,8,9,1,2,3,4,5,6],
    ///             vec![2,3,4,5,6,7,8,9,1],
    ///             vec![5,6,7,8,9,1,2,3,4],
    ///             vec![8,9,1,2,3,4,5,6,7],
    ///             vec![3,4,5,6,7,8,9,1,2],
    ///             vec![6,7,8,9,1,2,3,4,5],
    ///             vec![9,1,2,3,4,5,6,7,8],
    ///         ];
    ///
    ///         assert!(matrix_check(&test_example));
    ///     }
    ///     #[test]
    ///     fn matrix_check_works_2() {
    ///         let test_example = vec![
    ///             vec![1,2,3,4,5,6,7,8,9],
    ///             vec![0;9],
    ///             vec![7,8,9,1,2,3,4,5,6],
    ///             vec![2,3,4,5,6,7,8,9,1],
    ///             vec![5,6,7,8,9,1,2,3,4],
    ///             vec![8,9,1,2,3,4,5,6,7],
    ///             vec![3,4,5,6,7,8,9,1,2],
    ///             vec![6,7,8,9,1,2,3,4,5],
    ///             vec![0;9],
    ///         ];
    ///
    ///         assert!(matrix_check(&test_example));
    ///     }
    ///
    ///     #[test]
    ///     fn matrix_check_works_3() {
    ///         let test_example = vec![
    ///             vec![1,2,3,4,0,0,7,8,9],
    ///             vec![0;9],
    ///             vec![7,8,9,1,2,3,4,5,6],
    ///             vec![2,3,4,5,6,7,8,9,1],
    ///             vec![5,6,7,8,9,1,2,3,4],
    ///             vec![8,9,1,2,3,4,5,6,7],
    ///             vec![3,4,5,6,7,8,9,1,2],
    ///             vec![6,7,8,9,1,2,3,4,5],
    ///             vec![0;9],
    ///         ];
    ///
    ///         assert!(matrix_check(&test_example));
    ///     }
    ///
    ///     #[test]
    ///     fn matrix_check_works_err() {
    ///         let test_example = vec![
    ///             vec![2,2,3,4,5,6,7,8,9],
    ///             vec![4,5,6,7,8,9,1,2,3],
    ///             vec![7,8,9,1,2,3,4,5,6],
    ///             vec![2,3,4,5,6,7,8,9,1],
    ///             vec![5,6,7,8,9,1,2,3,4],
    ///             vec![8,9,1,2,3,4,5,6,7],
    ///             vec![3,4,5,6,7,8,9,1,2],
    ///             vec![6,7,8,9,1,2,3,4,5],
    ///             vec![9,1,2,3,4,5,6,7,8],
    ///         ];
    ///
    ///         assert_eq!(matrix_check(&test_example), false);
    ///     }
    ///
    ///     #[test]
    ///     fn matrix_check_works_err_2() {
    ///         let test_example = vec![
    ///             vec![1,2,3,4,5,6,7,8,9],
    ///             vec![4,5,6,7,8,9,1,5,3],
    ///             vec![7,8,9,1,2,3,4,5,6],
    ///             vec![2,3,4,5,6,7,8,9,1],
    ///             vec![5,6,7,8,9,1,2,3,4],
    ///             vec![8,9,1,2,3,4,5,6,7],
    ///             vec![3,4,5,6,7,8,9,1,2],
    ///             vec![6,7,8,9,1,2,3,4,5],
    ///             vec![9,1,2,3,4,5,6,7,8],
    ///         ];
    ///
    ///         assert_eq!(matrix_check(&test_example), false);
    ///     }
    /// }
    /// ```
    pub fn matrix_check(m: &Vec<Vec<u8>>) -> bool {
        for y in 0..9 { // check row
            let mut checked_value: Vec<u8> = vec![];
            // 直接使用 序列操作符[] 获得的是实际的对象而不是一个reference | pointer | copy
            let row = &m[y];
            row.into_iter()
                .for_each(|&x| {
                    checked_value.push(x);
                });
            checked_value.sort();
            if !&SudokuMatrix::arr_repeat_check(&checked_value) {
                return false;
            }
        }

        for x in 0..9 {
            { // check if there is any duplication of numbers in a column
                let mut checked_value: Vec<u8> = vec![];
                for y in 0..9 { // check column
                    checked_value.push(m[y][x]);
                }
                checked_value.sort();
                if !&SudokuMatrix::arr_repeat_check(&checked_value) {
                    return false;
                }
            }
            { // check 3 x 3 matrix
                // x use to point which matrix
                let mut mm_pos: Vec<(usize, usize)> = vec![];
                let y_range = match x / 3 {
                    0 => 0..3,
                    1 => 3..6,
                    2 => 6..9,
                    _ => panic!("index err"),
                };
                let x_range = match x % 3 {
                    0 => 0..3,
                    1 => 3..6,
                    2 => 6..9,
                    _ => panic!("index err"),
                };
                for y in y_range {
                    for x_inm in x_range.clone() {
                        mm_pos.push((y, x_inm));
                    }
                }
                let mut checked_value: Vec<u8> = vec![];
                mm_pos.into_iter().for_each(|(y, x)| {
                    checked_value.push(m[y][x]);
                });
                checked_value.sort();
                if !&SudokuMatrix::arr_repeat_check(&mut checked_value) {
                    return false;
                }
            }
        }
        true
    }

    /// generate sudoku matrix
    pub fn init(&mut self) -> &mut Self {
        let mut matrix: Vec<Vec<u8>> = vec![
            vec![0,0,0,0,0,0,0,0,0],
            vec![0,0,0,0,0,0,0,0,0],
            vec![0,0,0,0,0,0,0,0,0],
            vec![0,0,0,0,0,0,0,0,0],
            vec![0,0,0,0,0,0,0,0,0],
            vec![0,0,0,0,0,0,0,0,0],
            vec![0,0,0,0,0,0,0,0,0],
            vec![0,0,0,0,0,0,0,0,0],
            vec![0,0,0,0,0,0,0,0,0]
        ];

        let mut y = 0;
        let mut x = 0;
        let mut c_g = 0; // 回退次数计数
        loop {
            if y >= 9 {
                break;
            }
            if c_g >= 10 { // 回退次数过多时释放整个 row
                {
                    let m = &mut matrix[y];
                    m.into_iter()
                        .for_each(|x| {
                            *x = 0;
                        });
                    y -= 1;
                    c_g = 0;
                }
                {
                    let m = &mut matrix[y];
                    m.into_iter()
                        .for_each(|x| {
                            *x = 0;
                        });
                    x = 0;
                }
            }

            matrix[y][x] = random_num(1, 10) as u8;

            let mut c = 0; // 计数器

            while !Self::matrix_check(&matrix) {
                c += 1;
                matrix[y][x] = random_num(1, 10) as u8;
                if c >= 20 {
                    c_g += 1;
                    matrix[y][x] = 0;
                    if x == 0 && y > 0 { // matrix 换行
                        y -= 1;
                        x = 8;
                    } else if x > 0 {
                        x -= 1;
                    }
                    c = 0;
                }
            }
            x += 1;
            if x >= 9 { // matrix 换行
                y += 1;
                x = 0;
            }
        }

        for y in 0..9 {
            for x in 0..9 {
                self.data[x + (9 * y)] = matrix[y][x];
            }
        }

        // return self to enable link like invoke
        self
    }
}

static mut SUDOKU: SudokuMatrix = SudokuMatrix {
    data: [0; 81]
};

/// check sudoku matrix is allowed
#[no_mangle]
pub extern fn check() -> usize {
    let sudoku = unsafe { &SUDOKU };
    match &SudokuMatrix::matrix_check(&sudoku.get_matrix_from_data()) {
        true => {
            return 1;
        },
        false => {
            return 0;
        },
    }
}

/// return a sudoku data memory address
#[no_mangle]
pub extern fn new() -> &'static SudokuMatrix {
    unsafe { &SUDOKU }
}

#[no_mangle]
pub extern fn get_data() -> &'static [u8; 81] {
    unsafe { &SUDOKU.data }
}

/// generate sudoku matrix
#[no_mangle]
pub extern fn init() -> &'static [u8; 81] {
    unsafe {
        &SUDOKU
            .init()
            .data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = unsafe { Box::from_raw(ptr) };
        unsafe {
            s.generate_sudoku_matrix();
        }
        s.data.iter().map(|&x| {
            println!("{}", *x);
        });
        assert_eq!(s.check(), true);
    }
}
