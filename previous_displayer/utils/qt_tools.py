"""Helper tools for qt widgets"""

from PySide6.QtWidgets import (  # pylint: disable=E0611
    QLayout,
    QWidget,
    QSpacerItem,
    QSizePolicy,
)


def get_widget_from_layout(layout: QLayout, widget_name: str) -> QWidget:
    """Find a widget in a QLayout via the widget name
    Arg - layout (QLayout): A QLayout to search for the widget name
    Arg - widget_name (str): The name of the QWidget to search for
    Returns (QWidget | None): Returns the QWidget with the given name, if found. else
        returns None
    """
    for i in range(layout.count()):
        widget = layout.itemAt(i).widget()
        if widget.objectName() == widget_name:
            return widget
    return None


HORIZONTAL_SPACER = QSpacerItem(0, 0, QSizePolicy.Expanding, QSizePolicy.Minimum)
