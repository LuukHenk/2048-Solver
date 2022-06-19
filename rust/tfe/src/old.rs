fn old() {
    // 0x == hexadecimal
    // 1 hexadecimal can be saved in 4 bits
    // a board with 16 tiles can be saved in 1 64 bit int (4x16)
    let mut board: u64 = 0x0004_0003_0000_1111_u64;

    // let mut horMask1: u64 = 0xffff_0000_0000_0000_u64;

    // operatie om alleen horizontale rij 1 te bewaren is: board <bitwise and> horMask1
    // println!("{:016x}", board);
    // board |= horMask1;
    // board &= horMask1;
    // println!("{:016x}", board);

    // Count empty positions on the board
    // for i in  0 .. 16 {
    //     if board
    //     println!("{}", i)

    // Choose a random position from the empty positions (idx)
    // Add tile to position 'idx'
    let mut idx = 0;
    let mut tmp_board = board; // Use tmp_board to find the location of 0's
    let mut tile: u64 = 0x1_u64; // Tile that is added to the board
    loop {
        // Check if first value (0xF) is not 0. if so, shift to next tile
        while (tmp_board & 0xF) != 0 {
            tmp_board >>= 4;
            tile <<= 4;
        }

        // If the first value was 0, remove 1 from the index before shifting to next tile
        if idx == 0 { break } else {idx -=1}
        tile <<= 4;
        tmp_board >>= 4;
    }

    board |= tile;



    // Generated 64 bit string
    // println!("{:016x}", board);

    // Convert string to 2D board and print the real numbers
    //
    let mut board_cp = board;
    let mut idx = 15;
    let mut board_string = String::new();

    loop {
        let tile = board_cp & 0xF;
        let mut tile_int: u32 = tile.to_string().parse().unwrap();

        if tile_int != 0 {
            tile_int = 2u32.pow(tile_int);
        }

        let tile_str = &tile_int.to_string();

        if idx % 4 == 0 {
            board_string = board_string + tile_str + "\n";
        } else {

            board_string = board_string + tile_str + " - ";
        }

        if idx == 0 { break } else { idx -= 1 }
        // TODO what does this stand for?
        board_cp >>= 4;
    }

    println!("{}", board_string)

}
