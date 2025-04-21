pub fn annotate(minefield: &[&str]) -> Vec<String> {

    let mut annotated_field: Vec<String> = Vec::new();
    let mut proto_annotated_field: Vec<Vec<u32>>;
    let row_dim;
    let col_dim;

    if !is_valid_field(minefield) {
        return annotated_field;
    } else {
        row_dim = minefield.len();
        col_dim = minefield[0].len();
        proto_annotated_field = vec![vec![u32::MIN; col_dim]; row_dim];
    }

    for (row_idx, row) in minefield.iter().enumerate() {
        for (col_idx, val) in row.chars().enumerate(){
            if val == '*' {
                increment_neighbours(&mut proto_annotated_field, row_idx as isize, col_idx as isize, row_dim as isize, col_dim as isize);       //assumption: field is not so big that the size doesn't fit into isize
                proto_annotated_field[row_idx][col_idx] = u32::MAX;
            }
        }
    }
    translate_minefield(proto_annotated_field, &mut annotated_field);
    annotated_field
}


/*fn increment_neighbours(minefield_proto: &mut Vec<Vec<u32>>, row: isize, col: isize, row_dim: isize, col_dim: isize) -> () {
    //if a field is u32::MAX, don't increment -> saturating add
    if(is_in_bounds(row+1, col, row_dim, col_dim)){
        minefield_proto[row+1][col] = minefield_proto[row+1][col].saturating_add(1);
    }
    if(is_in_bounds(row, col+1, row_dim, col_dim)){
        minefield_proto[row][col+1] = minefield_proto[row][col+1].saturating_add(1);
    }
    if(is_in_bounds(row+1, col+1, row_dim, col_dim)){
        minefield_proto[row+1][col+1] = minefield_proto[row+1][col+1].saturating_add(1);
    }
    if(is_in_bounds(row-1, col, row_dim, col_dim)){
        minefield_proto[row-1][col] = minefield_proto[row-1][col].saturating_add(1);
    }
    if(is_in_bounds(row, col-1, row_dim, col_dim)){
        minefield_proto[row][col-1] = minefield_proto[row][col-1].saturating_add(1);
    }
    if(is_in_bounds(row-1, col-1, row_dim, col_dim)){
        minefield_proto[row-1][col-1] = minefield_proto[row-1][col-1].saturating_add(1);
    }
    if(is_in_bounds(row-1, col+1, row_dim, col_dim)){
        minefield_proto[row-1][col+1] = minefield_proto[row-1][col+1].saturating_add(1);
    }
    if(is_in_bounds(row+1, col-1, row_dim, col_dim)){
        minefield_proto[row+1][col-1] = minefield_proto[row+1][col-1].saturating_add(1);
    }
}*/
fn increment_neighbours(
    minefield: &mut Vec<Vec<u32>>,
    row: isize,
    col: isize,
    row_dim: isize,
    col_dim: isize,
) {
    fn is_in_bounds(row: isize, col: isize, row_dim: isize, col_dim: isize) -> bool {
        if(row < row_dim && col < col_dim) && (row >= 0 && col >= 0){
            return true;
        }
        false
    }

    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    for (dr, dc) in directions {
        let new_row = row + dr;
        let new_col = col + dc;

        if is_in_bounds(new_row, new_col, row_dim, col_dim) {
            minefield[new_row as usize][new_col as usize] =
                minefield[new_row as usize][new_col as usize].saturating_add(1);
        }
    }
}

fn is_valid_field(minefield: &[&str]) -> bool {

    if minefield.is_empty() {
        return false;
    }

    let  first_len = minefield[0].len();
    for row in minefield {
        if row.len() != first_len {
            return false;
        }
    }
    true
}

fn translate_minefield(prototype: Vec<Vec<u32>>, target: &mut Vec<String>) -> () {
    for (i, row) in prototype.iter().enumerate() {
        target.push(String::new());
        for col in row {
            match *col {
                u32::MIN => target[i].push(' '),
                1..9 => target[i].push(char::from_digit(*col,10).unwrap()),
                _ => target[i].push('*'),           //assumption nothing is ever counted over 9, which should be impossible
            }
        }
    }
}
