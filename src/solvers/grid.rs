use crate::input_handler::read_grid_from_file;

#[allow(dead_code)]
fn flatten_rows<T>(v: Vec<Vec<T>>) -> Vec<T> {
    v.into_iter().flatten().collect()
}

fn flatten_rows_with_delimiter<T: Clone>(v: Vec<Vec<T>>, d: T) -> Vec<T> {
    let mut out = Vec::new();

    for (idx, row) in v.into_iter().enumerate() {
        if idx > 0 {
            out.push(d.clone());
        }
        out.extend(row);
    }

    out
}

#[allow(dead_code)]
fn flatten_cols<T: Clone>(v: Vec<Vec<T>>) -> Vec<T> {
    if v.is_empty() {
        return Vec::new();
    }

    let rows = v.len();
    let cols = v.iter().map(|row| row.len()).max().unwrap_or(0);
    let mut out = Vec::with_capacity(rows * cols);

    for c in 0..cols {
        for r in 0..rows {
            if c < v[r].len() {
                out.push(v[r][c].clone());
            }
        }
    }

    out
}

fn flatten_cols_with_delimiter<T: Clone>(v: Vec<Vec<T>>, d: T) -> Vec<T> {
    if v.is_empty() {
        return Vec::new();
    }

    let rows = v.len();
    let cols = v.iter().map(|row| row.len()).max().unwrap_or(0);
    let mut out = Vec::new();

    for c in 0..cols {
        if c > 0 {
            out.push(d.clone());
        }

        for r in 0..rows {
            if c < v[r].len() {
                out.push(v[r][c].clone());
            }
        }
    }

    out
}

fn flatten_diag_with_delimiter<T: Clone>(v: Vec<Vec<T>>, d: T) -> Vec<T> {
    if v.is_empty() {
        return Vec::new();
    }

    let rows = v.len();
    let cols = v.iter().map(|row| row.len()).max().unwrap_or(0);
    let mut out = Vec::new();

    // Diagonals starting from the bottom row (bottom-left to bottom-right)
    for c_start in 0..cols {
        if !out.is_empty() {
            out.push(d.clone());
        }

        let mut r = rows - 1;
        let mut c = c_start;

        // Walk up-right: (r-1, c+1)
        loop {
            if r >= rows || c >= v[r].len() {
                break;
            }
            out.push(v[r][c].clone());

            if r == 0 {
                break;
            }
            r -= 1;
            c += 1;
        }
    }

    // Diagonals starting from the first column, moving from bottom-1 up to top
    for r_start in (0..rows - 1).rev() {
        out.push(d.clone());

        let mut r = r_start;
        let mut c = 0;

        // Walk up-right: (r-1, c+1)
        loop {
            if r >= rows || c >= v[r].len() {
                break;
            }
            out.push(v[r][c].clone());

            if r == 0 {
                break;
            }
            r -= 1;
            c += 1;
        }
    }

    out
}

fn flatten_anti_diag_with_delimiter<T: Clone>(v: Vec<Vec<T>>, d: T) -> Vec<T> {
    if v.is_empty() {
        return Vec::new();
    }

    let rows = v.len();
    let cols = v.iter().map(|row| row.len()).max().unwrap_or(0);
    let mut out = Vec::new();

    if cols == 0 {
        return out;
    }

    let last_col = cols - 1;

    // Anti-diagonals starting from the bottom row (bottom-right to bottom-left)
    for c_start in (0..cols).rev() {
        if !out.is_empty() {
            out.push(d.clone());
        }

        let mut r = rows - 1;
        let mut c = c_start;

        // Walk up-left: (r-1, c-1)
        loop {
            if r >= rows || c >= v[r].len() {
                break;
            }
            out.push(v[r][c].clone());

            if r == 0 || c == 0 {
                break;
            }
            r -= 1;
            c -= 1;
        }
    }

    // Anti-diagonals starting from the last column, from bottom-1 up to top
    for r_start in (0..rows - 1).rev() {
        out.push(d.clone());

        let mut r = r_start;
        let mut c = last_col;

        // Walk up-left: (r-1, c-1)
        loop {
            if r >= rows || c >= v[r].len() {
                break;
            }
            out.push(v[r][c].clone());

            if r == 0 || c == 0 {
                break;
            }
            r -= 1;
            c -= 1;
        }
    }

    out
}

fn get_max_product(v: &[i64], product_length: usize) -> i64 {
    if product_length == 0 || v.len() < product_length {
        return 0;
    }

    // Initial product of the first `product_length` elements
    let mut product: i64 = 1;
    for i in 0..product_length {
        product *= v[i];
    }

    let mut max_product = product;

    // Slide the window across the vector
    for i in product_length..v.len() {
        let leaving = v[i - product_length];
        let entering = v[i];

        // If the leaving element is zero or the current product is zero,
        // recompute the product for the current window from scratch.
        if leaving == 0 || product == 0 {
            product = 1;
            for j in (i + 1 - product_length)..=i {
                product *= v[j];
            }
        } else {
            product = (product / leaving) * entering;
        }

        if product > max_product {
            max_product = product;
        }
    }

    max_product
}

pub fn largest_product_in_grid(problem_number: i64, product_length: i64) -> i64 {
    let grid = read_grid_from_file(format!("./data/problem_{}.txt", problem_number).as_str());
    // Create list of numbers for horizontal, vertical, diagonal and anti-diagonal products.
    // Put a zero in between lines to force the product to zero for invalid products.
    let pl = if product_length <= 0 {
        return 0;
    } else {
        product_length as usize
    };

    let row_numbers = flatten_rows_with_delimiter(grid.clone(), 0_i64);
    let col_numbers = flatten_cols_with_delimiter(grid.clone(), 0_i64);
    let diag_numbers = flatten_diag_with_delimiter(grid.clone(), 0_i64);
    let anti_diag_numbers = flatten_anti_diag_with_delimiter(grid, 0_i64);

    let max_products = [
        get_max_product(&row_numbers, pl),
        get_max_product(&col_numbers, pl),
        get_max_product(&diag_numbers, pl),
        get_max_product(&anti_diag_numbers, pl),
    ];

    max_products.into_iter().max().unwrap_or(0)
}

/// Problem-11 solution; signature matches SolutionFn (fn(i64) -> i64).
pub fn largest_product_in_grid_11(product_length: i64) -> i64 {
    largest_product_in_grid(11, product_length)
}
