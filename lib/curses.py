import curses
import math

class Screen:
    curses.initscr()
    curses.start_color()
    curses.use_default_colors()
    for i in range(0, curses.COLORS):
        curses.init_pair(i + 1, 197, i)

    def __init__(self):
        self.stdscr = curses.initscr()
        curses.noecho()
        curses.cbreak()
        self.stdscr.keypad(1)

    def text_styling(self, val):
        val = (' ' if val == 0 else val)
        return (str(val).center(6, ' '))

    def set_color(self, value):
        return (8 if value == 0 else round(math.log(value, 2)) * 2 + 78)

    def render_game(self, board, score):
        for y, row in enumerate(board):
            y = y * 3
            for x, value in enumerate(row):
                self.stdscr.addstr(y, x * 6, '      ', curses.color_pair(self.set_color(value)))
                self.stdscr.addstr(y + 1, x * 6, self.text_styling(value), curses.color_pair(self.set_color(value)))
                self.stdscr.addstr(y + 2, x * 6, '      ', curses.color_pair(self.set_color(value)))
        self.stdscr.addstr(12, 0, 'Score: ')
        self.stdscr.addstr(12, 15-len(str(score)), str(score))
        self.stdscr.refresh() 

    def reset(self):
        curses.echo()
        curses.nocbreak()
        curses.endwin()

    def keypress(self):
        key = self.stdscr.getch()
        self.stdscr.refresh()
        return(key)
