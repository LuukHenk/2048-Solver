"""The game board displayer"""

from PySide6.QtWidgets import (  # pylint: disable=E0611
    QGridLayout,
    QWidget,
    QHBoxLayout,
    QLabel,
)
from PySide6.QtCore import Qt, Slot  # pylint: disable=E0611
from src.data_layer.game_data_formats import MoveResult
from src.presentation_layer.colors import COLORSET_2, contrasting_text_color


class Board(QWidget):  # pylint: disable=R0903
    """The board widge"""

    def __init__(self, board_size: int = 4) -> None:
        """
        Creates a 2048 board
        Args:
            board (Table[Table[int]]) = The 2048 game board

        Returns (QtWidgets.QWidget) = The game board as qt widget
        """
        super().__init__()
        self.board = QGridLayout(self)
        self.board_size = 100
        self._font_size = round(self.board_size / 5)
        self._default_bg_color = COLORSET_2[0]
        self._default_color = "#" + contrasting_text_color(self._default_bg_color[1:])
        self.__create_cells(board_size)

    @Slot(MoveResult)
    def update_cells(self, move_result: MoveResult) -> None:
        """update the cells with the new board data"""
        for y_position, row in enumerate(move_result["board"]):
            for x_position, value in enumerate(row):
                cell = self.board.itemAtPosition(y_position, x_position).widget()
                self._update_cell(cell, value)

    def _update_cell(self, cell: QWidget, value: int):
        cell_label = cell.findChild(QLabel)
        cell_text = str(value).strip("0")
        cell_label.setText(cell_text)
        bg_color = COLORSET_2[value] if value in COLORSET_2 else self._default_bg_color
        font_color = "#" + contrasting_text_color(bg_color[1:])
        cell.setStyleSheet(f"background-color:{bg_color}; color:{font_color};")

    def __create_cells(self, board_size: int):
        size_range = range(board_size)
        for y_position in size_range:
            for x_position in size_range:
                cell = self.__create_cell()
                self.board.addWidget(cell, y_position, x_position, Qt.AlignCenter)

    def __create_cell(self) -> QWidget:
        cell_label = QLabel()
        cell_label.setAlignment(Qt.AlignCenter)
        cell_label.setStyleSheet("QLabel {font-size: " + str(self._font_size) + "px;}")
        cell_content = QHBoxLayout()
        cell_content.addWidget(cell_label)
        cell = QWidget()
        cell.setLayout(cell_content)
        cell.setStyleSheet(
            f"background-color:{self._default_bg_color}; color:{self._default_color};"
        )
        cell.setFixedSize(self.board_size, self.board_size)
        return cell
