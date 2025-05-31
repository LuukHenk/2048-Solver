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
import { Pause } from "@mui/icons-material"

export default function GamesVisualzer({ games }: { games: [CurrentMove[]] }) {
    const [iCurrentGame, setICurrentGame] = useState(0)
    const [iCurrentMove, setICurrentMove] = useState(0)
    const CURRENT_MOVE = games[iCurrentGame][iCurrentMove]
    const FIRST_MOVE = iCurrentMove === 0
    const LAST_MOVE = iCurrentMove === games[iCurrentGame].length - 1
    const FIRST_GAME = iCurrentGame === 0
    const LAST_GAME = iCurrentGame === games.length

    const [intervalID, setIntervalID] = useState<undefined | NodeJS.Timeout>(undefined)

    function autoPlay() {
        if (LAST_MOVE) { stopAutoPlay() }
        console.debug("Perfoming next move")
        setICurrentMove(iCurrentMove => iCurrentMove + 1);
    }
    function startAutoPlay() {
        console.debug("Starting autoplay")
        const interval = setInterval(autoPlay, 500)
        setIntervalID(interval)
    }
    function stopAutoPlay() {
        console.debug("Stopping autoplay")
        clearInterval(intervalID)
        setIntervalID(undefined)
    }

    function onPreviousMove() {
        if (FIRST_MOVE) { console.debug("Already at first move"); return }
        setICurrentMove(iCurrentMove - 1)
        console.debug(`current move lowered: ${iCurrentMove}`)
    }
    function onNextMove() {
        if (LAST_MOVE) { console.debug("Already at last move"); return }
        setICurrentMove(iCurrentMove + 1)
        console.debug(`current move upped: ${iCurrentMove}`)
    }
    function onPreviousGame() {
        if (FIRST_GAME) { console.debug("Already at first game"); return }
        setICurrentGame(iCurrentGame - 1)
        console.debug(`current game lowered: ${iCurrentGame}`)
    }
    function onNextGame() {
        if (LAST_GAME) { console.debug("Already at last game"); return }
        setICurrentGame(iCurrentGame + 1)
        console.debug(`current game upped: ${iCurrentGame}`)
    }

    return <Stack sx={{ width: "100%", height: "100vh", alignItems: "center", justifyContent: "center", display: "flex" }}>
        <GameGrid board={CURRENT_MOVE.board} />
        <Stack direction="column" sx={{ marginTop: 2, height: 50 }}>
            <ButtonGroup variant="outlined">
                <Button aria-label="Previous game" onClick={onPreviousGame} disabled={FIRST_GAME || intervalID !== undefined}><SkipPrevious /></Button>
                <Button aria-label="Previous move" onClick={onPreviousMove} disabled={FIRST_MOVE || intervalID !== undefined}><ChevronLeft /></Button>

                <Button aria-label="Next move" onClick={onNextMove} disabled={LAST_MOVE || intervalID !== undefined}><ChevronRight /></Button>
                <Button aria-label="Next game" onClick={onNextGame} disabled={LAST_GAME || intervalID !== undefined}><SkipNext /></Button>
            </ButtonGroup>
            {intervalID === undefined && <Button variant="outlined" aria-label="Autoplay" endIcon={<PlayArrow />} onClick={startAutoPlay}>AutoPlay</Button>}

            {intervalID !== undefined && <Button variant="outlined" aria-label="StopAutoplay" endIcon={<Pause />} onClick={stopAutoPlay}>Stop autoplay</Button>}
        </Stack>
    </Stack>
}