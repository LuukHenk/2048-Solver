"use client"

import Stack from "@mui/material/Stack"
import GameGrid from "./gameGrid/GameGrid"
import { CurrentMove } from "./types"
import { useState } from "react"
import ControlPanel from "./ControlPanel"
import Grid from "@mui/material/Grid"
import Box from "@mui/material/Box"
import theme from "../theme"
import useMediaQuery from "@mui/material/useMediaQuery"

export default function GamesVisualzer({ games }: { games: [CurrentMove[]] }) {
    const [iCurrentGame, setICurrentGame] = useState(0)
    const [iCurrentMove, setICurrentMove] = useState(0)
    const CURRENT_MOVE = games[iCurrentGame][iCurrentMove]
    const GAME_MOVE_COUNT = games[iCurrentGame].length - 1
    const GAME_COUNT = games.length - 1

    const SMALL_SCREEN = useMediaQuery(theme.breakpoints.down('md'))
    return (
        <Stack direction={SMALL_SCREEN ? "column" : "row"} sx={{ width: "100%", height: "100vh", display: "flex", alignItems: "center" }}>
            <Stack direction="column" sx={{ height: SMALL_SCREEN ? "10%" : "100%", width: SMALL_SCREEN ? "50%" : "40%", padding: SMALL_SCREEN ? 0 : "5vh", paddingTop: SMALL_SCREEN ? 1 : 0, justifyContent: "start" }}>
                <ControlPanel
                    iCurrentGame={iCurrentGame}
                    iCurrentMove={iCurrentMove}
                    setICurrentGame={setICurrentGame}
                    setICurrentMove={setICurrentMove}
                    gameCount={GAME_COUNT}
                    gameMoveCount={GAME_MOVE_COUNT}
                />
            </Stack>
            <Box sx={{ width: SMALL_SCREEN ? "80%" : "50%", paddingTop: SMALL_SCREEN ? 5 : 0 }}>
                <GameGrid board={CURRENT_MOVE.board} />
            </Box>
        </Stack>
    )
}