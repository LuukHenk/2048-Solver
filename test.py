import tkinter as tk

COLORSET_1 = {
    0: "floral white",
    2: "#baaed2",
    4: "#f0ebb9",
    8: "#a8bfd0",
    16: "#b2cebc",
    32: "#eec8c8",
    64: "#f4ebdc",
    128: "#d2b27d",
    256: "#947144",
    512: "#456730",
    1024: "#2f3831",
    2048: "#e5c76a",
    4096: "#570926",
    8192: "#7d4157",
    16384: "#bd90aa",
    32768: "#ffffff",
    65536: "#000000"
}

COLORSET_2 = {
    0: "#f0f0f0",
    2: "#ffe7e0",
    4: "#80554a",
    8: "#ffa994",
    16: "#ba716b",
    32: "#cc8776",
    64: "#4089ff",
    128: "#b37b09",
    256: "#1b55b3",
    512: "#ffc040",
    1024: "#633221",
    2048: "#e5c76a",
    4096: "#78d12f",
    8192: "#2e9551",
    16384: "#0b4443",
    32768: "#ffffff",
    65536: "#000000"
}

ROOT = tk.Tk()

def generate_gui_board(board, colors, tile_size):
    gui_board = {}

    global gameframe
    gameframe = tk.Frame(ROOT)
    gameframe.pack()

    for i, row in enumerate(board):
        for j, column in enumerate(row):
            value = "  " if column == 0 else str(column)
            value += " " if len(value) % 2 == 1 else ""

            value_size = len(value)
            empty_space = round(tile_size/2)- round(value_size/2)
            # print("|" + " "*empty_space+value+" "*empty_space + "|")
            tile_value = " " * empty_space + value + " " * empty_space

            gui_board_label = str(i)+", "+str(j)
            gui_board[gui_board_label] = tk.Label(
                gameframe,
                text=tile_value,
                font=("Ariel", tile_size*2),
                bg=colors[column] if column in colors else "#f0f0f0",
            )

            gui_board[gui_board_label].grid(
                row=i,
                column=j,
                ipadx=tile_size*3,
                ipady=tile_size*6,
                sticky="WENS"
            )
    return gui_board

def draw(board, gui_board, colors, tile_size):
    """ Draw the board on an external screen """
    ROOT.update_idletasks()
    ROOT.update()

    for i, row in enumerate(board):
        for j, column in enumerate(row):
            value = "  " if column == 0 else str(column)
            value += " " if len(value) % 2 == 1 else ""

            value_size = len(value)
            empty_space = round(tile_size/2)- round(value_size/2)
            # print("|" + " "*empty_space+value+" "*empty_space + "|")
            tile_value = " " * empty_space + value + " " * empty_space

            gui_board_label = str(i)+", "+str(j)
            gui_board[gui_board_label].configure(
                text=tile_value,
                bg=colors[column] if column in colors else "#f0f0f0"
            )
#TODO test on the real game ^^

from lib.game import Game
import time
game = Game()
# board = [[2, 4, 8, 16], [32, 64, 128, 256], [512, 1024, 2048, 4096], [8192, 16384, 32768, 65536]]
COLORS = COLORSET_1
# Default TILE_SIZE for a 4x4 board is 10
TILE_SIZE = 10

#
gui_board = generate_gui_board(game.board.board, COLORS, TILE_SIZE)
while True:
    direction = input("---")
    if direction == "a":
        direction = "left"
    elif direction == "s":
        direction = "down"


    if direction in game.possible_directions(game.board.board):
        print("hello0")
        game.perform_movement(direction)
        print("hello1")
        draw(game.board.board, gui_board, COLORS, TILE_SIZE)
    print("hello2")



ROOT.mainloop()

# https://stackoverflow.com/questions/29158220/tkinter-understanding-mainloop
# https://stackoverflow.com/questions/8829751/python-tkinter-how-to-update-information-in-grid





