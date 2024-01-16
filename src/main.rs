use std::time::Instant;

fn main() {
    let start = Instant::now(); // 4 9 16 25 36 49 64 81 100 121
    // u8 => max dimension length 4
    // u16 => max dimension length 9 (default)
    // u32 => max dimension length 25
    // u64 => max dimension length 49
    // u128 => max dimension length 121
    let mut sudoku: Vec<Vec<u128>> = vec![
        vec![5,3,0,0,7,0,0,0,0],
        vec![6,0,0,1,9,5,0,0,0],
        vec![0,9,8,0,0,0,0,6,0],
        vec![8,0,0,0,6,0,0,0,3],
        vec![4,0,0,8,0,3,0,0,1],
        vec![7,0,0,0,2,0,0,0,6],
        vec![0,6,0,0,0,0,2,8,0],
        vec![0,0,0,4,1,9,0,0,5],
        vec![0,0,0,0,8,0,0,7,9]
        ];
    let dimension = sudoku.len();
    for row in sudoku.iter_mut(){
        for cell in row.iter_mut(){
            if *cell != 0{
                *cell = u128::pow(2, (*cell).try_into().unwrap());
            }
            else{
                // Minus 1 to get all 1's for the binary work, and minus 1 again since we ignore 1 (which would be the representation of 0 had it been part of the sudoku)
                *cell = u128::pow(2, (dimension + 1).try_into().unwrap()) - 2
            }
        }
    }
    println!("{:?}", sudoku);
    sudoku = sudoku_loop(sudoku, dimension);
    for row in sudoku.iter_mut(){
        for cell in row.iter_mut(){
            // Pretty cool way to get the log2 of a number if it's an exponential of 2
            *cell = u128::from(cell.trailing_zeros());
        }
    }
    println!("{:?}", sudoku);
    let elapsed = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", elapsed);
}

fn sudoku_loop(mut sudoku: Vec<Vec<u128>>, dimension: usize) -> Vec<Vec<u128>>{
    let zeroed: Vec<u128> = vec![0; dimension];
    let mut rows = vec![u128::pow(2, (dimension + 1).try_into().unwrap()) - 2; dimension];
    let mut blocks = rows.clone();
    let mut columns = rows.clone();
    // Make list of all possibilities for the rows, blocks, and columns in binary by looking at already certain cells (clean exponentials of two)
    for (y, row) in &mut sudoku.iter_mut().enumerate() {
        for (x, cell) in &mut row.iter_mut().enumerate(){
            if (*cell & (*cell - 1)) == 0 {
                rows[y] = rows[y] & !*cell;
                blocks[(y/3)*3 + x/3] = blocks[(y/3)*3 + x/3] & !*cell;
                columns[x] = columns[x] & !*cell;
            }
        }
    }
    // If no possibilities left, assume sudoku is solved. Only one out of the 3 needs to be checked, because it would show up in all of them.
    if rows == zeroed{
        return sudoku;
    }
    // Set cell value as the overlapping possibility/ies of all the corresponding row, block, and column value.
    for (y, row) in &mut sudoku.iter_mut().enumerate() {
        for (x, cell) in &mut row.iter_mut().enumerate(){
            if (*cell & (*cell - 1)) != 0 {
                *cell = rows[y] & blocks[(y/3)*3 + x/3] & columns[x];
            }
        }
    }
    return sudoku_loop(sudoku, dimension);
}
