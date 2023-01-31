from typing import List, Final
from PySide6.QtWidgets import (
    QGridLayout,
    QWidget,
    QHBoxLayout,
    QLabel,
)
from PySide6.QtCore import Qt, Slot
from game_displayer.board.tile_value_color_mapping import TILE_VALUE_TO_COLOR_MAPPING


class TileWidget(QWidget):

    TILE_SIZE: Final[int] = 100
    FONT_SIZE: Final[int] = round(TILE_SIZE / 5)
    DEFAULT_TILE = "0"

    def __init__(self):
        super().__init__()
        self.__layout = QHBoxLayout(self)
        self.__layout.addWidget(self.__create_label_widget())
        self.__tile_styling()

    def __create_label_widget(self) -> QLabel:
        cell_label = QLabel()
        cell_label.setAlignment(Qt.AlignCenter)
        cell_label.setStyleSheet("QLabel {font-size: " + str(self.FONT_SIZE) + "px;}")

    def __tile_styling(self):
        background_color = TILE_VALUE_TO_COLOR_MAPPING[self.DEFAULT_TILE]
        font_color = contrasting_text_color(background_color)
        self.setStyleSheet(f"background-color:{background_color}; color:{font_color};")
        self.setFixedSize(self.TILE_SIZE, self.TILE_SIZE)


def contrasting_text_color(hex_str):
    """Input a string without hash sign of RGB hex digits to compute
    complementary contrasting color such as for fonts
    """
    (red, green, blue) = (hex_str[:2], hex_str[2:4], hex_str[4:])
    if (
        1
        - (int(red, 16) * 0.299 + int(green, 16) * 0.587 + int(blue, 16) * 0.114) / 255
        < 0.5
    ):
        return "#000"
    return "#fff"
