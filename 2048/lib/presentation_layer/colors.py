"""Color for each tile number in a 4x4 board"""

COLORSET_2 = {
    0: "#d0d0d0",
    2: "#ffe7e0",
    4: "#80554a",
    8: "#ffa994",
    16: "#ba716b",
    32: "#cc8776",
    64: "#493C2B",
    128: "#000000",
    256: "#1b55b3",
    512: "#ffc040",
    1024: "#633221",
    2048: "#e5c76a",
    4096: "#78d12f",
    8192: "#2e9551",
    16384: "#0b4443",
    32768: "#ffffff",
    65536: "#000000",
}


def contrasting_text_color(hex_str):
    """Input a string without hash sign of RGB hex digits to compute
    complementary contrasting color such as for fonts
    """
    (r, g, b) = (hex_str[:2], hex_str[2:4], hex_str[4:])  # pylint: disable=c0103
    if 1 - (int(r, 16) * 0.299 + int(g, 16) * 0.587 + int(b, 16) * 0.114) / 255 < 0.5:
        return "000"
    return "fff"
