from typing import Final
from PySide6.QtWidgets import (
    QWidget,
    QHBoxLayout,
    QLabel,
)
from PySide6.QtCore import Qt
from game_displayer.board.tile_value_color_mapping import TILE_TEXT_TO_COLOR_MAPPING

TILE_SIZE: Final[int] = 120


class TileWidget(QWidget):
    """Tile widget"""

    FONT_SIZE: Final[int] = round(TILE_SIZE / 5)
    DEFAULT_TEXT: Final[str] = "0"

    def __init__(self):
        super().__init__()
        self.__label_widget = self.__create_label_widget()
        self.__set_tile_layout()

    def style_tile_based_on_text(self, text: str):
        """Formats the tile style based on the given text"""
        background_color = self.__map_tile_text_with_hex_color(text)
        font_color = "#" + self.__contrasting_text_color(background_color[1:])
        self.setStyleSheet(f"background-color:{background_color}; color:{font_color};")
        self.__label_widget.setText(self.__format_text(text))

    def __set_tile_layout(self) -> None:
        layout = QHBoxLayout(self)
        layout.addWidget(self.__label_widget)
        self.style_tile_based_on_text(self.DEFAULT_TEXT)
        self.setFixedSize(TILE_SIZE, TILE_SIZE)

    def __create_label_widget(self) -> QLabel:
        label_widget = QLabel()
        label_widget.setAlignment(Qt.AlignCenter)
        label_widget.setStyleSheet("QLabel {font-size: " + str(self.FONT_SIZE) + "px;}")
        return label_widget

    def __map_tile_text_with_hex_color(self, text: str) -> str:
        if text in TILE_TEXT_TO_COLOR_MAPPING:
            return TILE_TEXT_TO_COLOR_MAPPING[text]
        return TILE_TEXT_TO_COLOR_MAPPING[self.DEFAULT_TEXT]

    @staticmethod
    def __format_text(text: str) -> str:
        return "" if text == "0" else text

    @staticmethod
    def __contrasting_text_color(hex: str) -> str:
        """Input a string without hash sign of RGB hex digits to compute
        complementary contrasting color such as for fonts
        """
        (red, green, blue) = (hex[:2], hex[2:4], hex[4:])
        if (
            1
            - (int(red, 16) * 0.299 + int(green, 16) * 0.587 + int(blue, 16) * 0.114)
            / 255
            < 0.5
        ):
            return "000"
        return "fff"
