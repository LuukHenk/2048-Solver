fn main() {
    let mut row: u16 = 0x1112;
    row = merge_row_to_the_right(row);
    println!("{:#02X}", row);
}

fn merge_row_to_the_right(row: u16) -> u16 {
    let mut tmp_row: u16 = row;
    let mut new_row: u16 = 0x0000;
    let mut tile_to_add: u16;
    let mut current_position_on_new_row: u8 = 0;
    let mut first_tile: u16 = 0x000F & tmp_row;
    let mut second_tile: u16 = 0x0000;
    tmp_row >>= 4;
    
    for _ in 0..3 {
        second_tile = 0x000F & tmp_row;
        tmp_row >>= 4;
        if first_tile == 0 {
            first_tile = second_tile;
        } else if second_tile != 0{
            if first_tile == second_tile {
                tile_to_add = first_tile + 1;
                first_tile = 0x0000;
                second_tile = 0x0000;
            } else {
                tile_to_add = first_tile;
                first_tile = second_tile;
            }
            new_row += tile_to_add << current_position_on_new_row;
            current_position_on_new_row += 4;
        }

    }
    if second_tile != 0 {
        new_row += second_tile << current_position_on_new_row;
    }

    return new_row
}









// fn right_move(row: u16) {
//     let mut tmp_row: u16 = row;
//     let mut new_row: u16 = 0x0000;
//     let mut first_tile: u16 = 0x0000;
//     let mut second_tile: u16;
//     let mut new_row_idx: u8 = 0;
//     for _i in 0..4 {
//         println!("{}", _i);
//         second_tile = 0xF000 & tmp_row;   
//         if second_tile != 0 {
//             if second_tile == first_tile {first_tile = first_tile * 2;}
//             if first_tile != 0 {
//                 first_tile >>= 4 * new_row_idx;
//             }
//             new_row += first_tile;
//             new_row_idx += 1;
//             first_tile = 0x0000;
//         } else {
//             first_tile >>= 4 * new_row_idx;
//             new_row += first_tile;
//             first_tile = second_tile;
//         }
        
//         tmp_row <<= 4;
//         // measure here
//     }
//     println!("{}", new_row);
//     // TODO add second tile to board
// }

// i: 0 f: 1000 s: 1000 row: 2110 out: 0000
// i: 1 f: 2000 s: 2000 row: 2110 out: 1200

// i: 2 f: 1000 s: 1000 row: 1100 out: 1220
// i: 3 f: 1000 s: 1000 row: 1000 out: 1220