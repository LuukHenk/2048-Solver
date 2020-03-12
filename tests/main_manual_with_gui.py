
from lib.gui_rendering import Render
from lib.game import Game

game = Game()
print(game.board.board)

render = Render()
render.draw_gui_board(game.board.board)

while True:

    direction = input("---")
    if direction == "a":
        direction = "left"
    elif direction == "s":
        direction = "down"
    elif direction == "d":
        direction = "right"
    elif direction == "w":
        direction = "up"

    possible_directions = game.possible_directions(game.board.board)
    if direction in possible_directions:
        game.perform_movement(direction)
        render.draw_gui_board(game.board.board)
    # Break if game over
    elif len(possible_directions) == 0:
        break

render.root.mainloop()

# https://stackoverflow.com/questions/29158220/tkinter-understanding-mainloop
# https://stackoverflow.com/questions/8829751/python-tkinter-how-to-update-information-in-grid





