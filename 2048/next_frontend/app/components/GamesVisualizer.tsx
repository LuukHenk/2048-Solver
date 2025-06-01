"use client"

import Stack from "@mui/material/Stack"
import GameGrid from "./gameGrid/GameGrid"
import { CurrentMove } from "./types"
import { useState } from "react"
import ControlPanel from "./ControlPanel"
import Box from "@mui/material/Box"
import theme from "../theme"
import useMediaQuery from "@mui/material/useMediaQuery"
import { Button, ButtonGroup, Chip, Typography } from "@mui/material"

export default function GamesVisualzer({ games }: { games: [CurrentMove[]] }) {
    const [iCurrentGame, setICurrentGame] = useState(0)
    const [iCurrentMove, setICurrentMove] = useState(0)
    const CURRENT_MOVE = games[iCurrentGame][iCurrentMove]
    const GAME_MOVE_COUNT = games[iCurrentGame].length - 1
    const GAME_COUNT = games.length - 1

    const SMALL_SCREEN = useMediaQuery(theme.breakpoints.down('md'))
    const CURRENT_GAME_TEXT = `Current Game: ${iCurrentGame + 1}`
    const CURRENT_MOVE_COUNT_TEXT = `Current Move: ${iCurrentMove + 1}`
    const SCORE_TEXT = `Score: ${CURRENT_MOVE.score}`
    const PERFORMED_MOVE_TEXT = `Performed Move: ${CURRENT_MOVE.performedMove}`
    return (
        <Stack direction={SMALL_SCREEN ? "column" : "row"} sx={{ width: "100%", height: "100vh", display: "flex", alignItems: "center" }}>
            <Stack direction="column" sx={{ height: SMALL_SCREEN ? "25%" : "100%", width: SMALL_SCREEN ? "100%" : "45%", padding: SMALL_SCREEN ? 0 : "5vh", paddingTop: SMALL_SCREEN ? 1 : 2, justifyContent: "start", alignItems: "center" }}>
                <ButtonGroup variant="contained" disabled sx={{width: "100%"}}>
                    <Button sx={{ width: "25%" }} >{CURRENT_GAME_TEXT}</Button>
                    <Button sx={{ width: "25%" }} >{CURRENT_MOVE_COUNT_TEXT}</Button>
                    <Button sx={{ width: "25%" }} >{SCORE_TEXT}</Button>
                    <Button sx={{ width: "25%" }} >{PERFORMED_MOVE_TEXT}</Button>
                </ButtonGroup>
                <Box sx={{width: "100%"}}>
                <ControlPanel
                    iCurrentGame={iCurrentGame}
                    iCurrentMove={iCurrentMove}
                    setICurrentGame={setICurrentGame}
                    setICurrentMove={setICurrentMove}
                    gameCount={GAME_COUNT}
                    gameMoveCount={GAME_MOVE_COUNT}
                />
                </Box>
            </Stack>
            <Box sx={{ width: SMALL_SCREEN ? "80%" : "50%", paddingTop: SMALL_SCREEN ? 5 : 0 }}>
                <GameGrid board={CURRENT_MOVE.board} />
            </Box>
        </Stack>
    )
}