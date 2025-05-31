import { Typography } from "@mui/material";
import Box from "@mui/material/Box";
import { BASE_TILE_COLOR } from "./constants";


const TILE_COLOR_MAPPING: { [value: number]: string } = {
    // https://martin-ueding.de/posts/matplotlib-colors-scales-as-hex-codes/#tab20c
    0: BASE_TILE_COLOR,
    2: "#3182bd",
    4: "#6baed6",
    8: "#9ecae1",
    16: "#e6550d",
    32: "#fd8d3c",
    64: "#fdae6b",
    128: "#31a354",
    256: "#74c476",
    512: "#a1d99b",
    1024: "#756bb1",
    2048: "#9e9ac8",
    4096: "#bcbddc",
    8192: "#636363",
    16384: "#969696",
    32768: "#ffffff",
    65536: "#000000",
}

function getTileColor(value: number) {
    let tileColor = BASE_TILE_COLOR
    try {
        tileColor = TILE_COLOR_MAPPING[value]
    } catch (error) {
        console.warn(`No tile color found for value ${value}: ${error}`)
    }
    if (tileColor === undefined) { tileColor = BASE_TILE_COLOR }
    return tileColor
}

function getContrastingTextColor(hex_str: string) {
    const RED_HEX = hex_str.substring(0, 2)
    const GREEN_HEX = hex_str.substring(2, 4)
    const BLUE_HEX = hex_str.substring(2, 4)

    const RED = parseInt(RED_HEX, 16)
    const GREEN = parseInt(GREEN_HEX, 16)
    const BLUE = parseInt(BLUE_HEX, 16)
    if (
        1
        - (RED * 0.299 + GREEN * 0.587 + BLUE * 0.114) / 255
        < 0.5
    ) { return "#000" }
    return "#fff"
}

export default function GameTile({ value }: { value: number }) {

    const TILE_COLOR = getTileColor(value)
    const TEXT_COLOR = getContrastingTextColor(TILE_COLOR.substring(1))
    return (
        <Box sx={{ textAlign: "center", alignContent: "center", backgroundColor: TILE_COLOR, height: "25%", border: `4px solid ${BASE_TILE_COLOR}`, borderRadius: 2 }}>
            <Typography sx={{ color: TEXT_COLOR, fontSize: { xs: 20, md: 26 } }}>{value}</Typography>
        </Box>
    )
}