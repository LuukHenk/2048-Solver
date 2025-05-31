"use client"

import Stack from "@mui/material/Stack"
import GameGrid from "./gameGrid/GameGrid"
import ButtonGroup from "@mui/material/ButtonGroup"
import Button from "@mui/material/Button"
import SkipPrevious from "@mui/icons-material/SkipPrevious"
import ChevronLeft from "@mui/icons-material/ChevronLeft"
import PlayArrow from "@mui/icons-material/PlayArrow"
import ChevronRight from "@mui/icons-material/ChevronRight"
import SkipNext from "@mui/icons-material/SkipNext"
import { CurrentMove } from "./types"
import { useState } from "react"

export default function GamesVisualzer({ games }: { games: [CurrentMove[]] }) {
    const [iCurrentGame, setICurrentGame] = useState(0)
    const [iCurrentMove, setICurrentMove] = useState(0)
    let currentMove = games[iCurrentGame][iCurrentMove]
    let firstMove = iCurrentMove === 0
    let lastMove = iCurrentMove === games[iCurrentGame].length - 1
    let firstGame = iCurrentGame === 0
    let lastGame = iCurrentGame === games.length - 1

    function onPreviousMove() {
        setICurrentMove(iCurrentMove - 1)
    }
    function onNextMove() {
        setICurrentMove(iCurrentMove + 1)
    }
    function onPreviousGame() {
        setICurrentGame(iCurrentGame - 1)
    }
    function onNextGame() {
        setICurrentGame(iCurrentGame + 1)
    }

    return <Stack sx={{ width: "100%", height: "100vh", alignItems: "center", justifyContent: "center", display: "flex" }}>
        <GameGrid board={currentMove.board} />
        <Stack direction="column" sx={{ marginTop: 2, height: 50 }}>
            <ButtonGroup variant="outlined">
                <Button aria-label="Previous game" onClick={onPreviousGame} disabled={firstGame}><SkipPrevious /></Button>
                <Button aria-label="Previous move" onClick={onPreviousMove} disabled={firstMove}><ChevronLeft /></Button>

                <Button aria-label="Next move" onClick={onNextMove} disabled={lastMove}><ChevronRight /></Button>
                <Button aria-label="Next game" onClick={onNextGame} disabled={lastGame}><SkipNext /></Button>
            </ButtonGroup>
            {/* <Button variant="outlined" aria-label="Autoplay">Autoplay<PlayArrow /></Button> */}
        </Stack>
    </Stack>
}