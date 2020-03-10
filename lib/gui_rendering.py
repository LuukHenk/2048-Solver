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


class Render:
    def __init__(self):
        # Load tk.Tk()
        self.root = tk.Tk()
        #TODO why do we use gameframe
        self.gameframe = tk.Frame(self.root)
        self.gameframe.pack()
        # Set the colorset (int: "hex")
        self.colorset = COLORSET_1
        # Set size of the tiles on the board
        self.tile_size = 10
        # Build a gui board using tk
        self.gui_board = self.generate_tk_board()

    def generate_tk_board(self, board_size=4, board=None):
        "Generate an empty tk grid in the size of the game board"
        # Generate an empty board of board_size if there is no input board given.
        # Otherwise use the board from the input
        board = [[0]*board_size]*board_size if board is None else board

        # Build the tk board using the tkinter import
        tk_board = {}
        for i, row in enumerate(board):
            for j, column in enumerate(row):
                value = "  " if column == 0 else str(column)
                value += " " if len(value) % 2 == 1 else ""

                value_size = len(value)
                empty_space = round(self.tile_size/2)- round(value_size/2)
                # print("|" + " "*empty_space+value+" "*empty_space + "|")
                tile_value = " " * empty_space + value + " " * empty_space

                tk_board_label = str(i)+", "+str(j)
                tk_board[tk_board_label] = tk.Label(
                    self.gameframe,
                    text=tile_value,
                    font=("Ariel", self.tile_size*2),
                    bg=self.colorset[column] if column in self.colorset else "#f0f0f0",
                )

                tk_board[tk_board_label].grid(
                    row=i,
                    column=j,
                    ipadx=self.tile_size*3,
                    ipady=self.tile_size*6,
                    sticky="WENS"
                )

        return tk_board

    def draw_gui_board(self, board):
        """ Draw the board on an external screen """
        for i, row in enumerate(board):
            for j, column in enumerate(row):
                value = "  " if column == 0 else str(column)
                value += " " if len(value) % 2 == 1 else ""

                value_size = len(value)
                empty_space = round(self.tile_size/2)- round(value_size/2)
                # print("|" + " "*empty_space+value+" "*empty_space + "|")
                tile_value = " " * empty_space + value + " " * empty_space

                gui_board_label = str(i)+", "+str(j)
                self.gui_board[gui_board_label].configure(
                    text=tile_value,
                    bg=self.colorset[column] if column in self.colorset else "#f0f0f0"
                )

        #TODO these must be at the end?
        self.root.update_idletasks()
        self.root.update()

    # def mainloop(self):
    #     self.root.mainloop()

# https://stackoverflow.com/questions/29158220/tkinter-understanding-mainloop
# https://stackoverflow.com/questions/8829751/python-tkinter-how-to-update-information-in-grid





