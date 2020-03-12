# board = "0000_0000_0000_0000"
# board = board.split("_")
# for row in board:
#     print(int(row, 16))

# board = "0001000100010001"
# b = bytes(board, 'utf-64')
# print(b)

import base64
board = "000f000f000f000f"
board_64 = base64.b64decode(board)
print(board_64)
new_board = base64.standard_b64encode(board_64)
print(new_board)
board_64 = base64.b64decode(board)
print(board_64)

for tile in new_board:
    print(int(str(tile), 4))



