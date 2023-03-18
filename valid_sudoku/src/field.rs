



type field = [[number;9];9];


type number = Option<u8>;


fn valid_field(field: &field) -> bool{


    for i in 0..9 as usize {
        for j in 0..9 as usize {
            if None == field[i][j] {continue;}
            if !(check_left_right(field, i, j) && check_top_bottom(field, i, j) && check_box(field, i, j)){
                return false
            }
        }
    }

    true
}



fn check_left_right(field: &field, x :usize, y :usize) -> bool{
    for i in 0..9 as usize {
        match (field[i][y], field[x][y]) {
            (Some(a), Some(b)) if a == b  && i != x =>  return false,
            _ => ()
        }
    }
    true
}


fn check_top_bottom(field: &field, x :usize, y :usize) -> bool{
    for j in 0..9 as usize {
        match (field[x][j], field[x][y]) {
            (Some(a), Some(b)) if a == b  && j != y =>  return false,
            _ => ()
        }
    }
    true
}


fn check_box(field: &field, x :usize, y :usize) -> bool{
    
    let currentbox = (x /3, y/3);
    let box_x = currentbox.0 *3;
    let box_y= currentbox.1 *3;
    
    for i in box_x..box_x+3 {
        for j in box_y..box_y+3 {
            match (field[i][j], field[x][y]) {
                (Some(a), Some(b)) if a == b  && !(i == x && j == y)=>  return false,
                _ => ()
            }
        }
    }
    true
}



#[test]
fn empty_field () {
    let f:field = [[None;9];9];
    assert_eq!(true, valid_field(&f));
}

#[test]
fn one_number() {
    let mut f:field = [[None;9];9];
    f[0][1] = Some(5);
    assert_eq!(true, valid_field(&f));
}

#[test]
fn many_numbers_five() {
    let mut f:field = [[None;9];9];
    f[0][1] = Some(5);
    f[3][2] = Some(5);
    f[7][8] = Some(5);
    assert_eq!(true, valid_field(&f));
}


#[test]
fn many_five_eight() {
    let mut f:field = [[None;9];9];
    f[0][1] = Some(5);
    f[3][2] = Some(5);
    f[7][8] = Some(5);
    f[1][0] = Some(8);
    f[3][3] = Some(8);
    f[7][4] = Some(8);
    //f[5][5] = Some(8);
    assert_eq!(true, valid_field(&f));
}

#[test]
fn many_five_eight_with_double_eight_in_one_one () {
    let mut f:field = [[None;9];9];
    f[0][1] = Some(5);
    f[3][2] = Some(5);
    f[7][8] = Some(5);
    f[1][0] = Some(8);
    f[3][3] = Some(8);
    f[7][4] = Some(8);
    f[5][5] = Some(8);
    assert_eq!(false, valid_field(&f));
}