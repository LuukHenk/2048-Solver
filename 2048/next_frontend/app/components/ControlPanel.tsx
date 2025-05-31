import ChevronLeft from "@mui/icons-material/ChevronLeft";
import ChevronRight from "@mui/icons-material/ChevronRight";
import Pause from "@mui/icons-material/Pause";
import PlayArrow from "@mui/icons-material/PlayArrow";
import SkipNext from "@mui/icons-material/SkipNext";
import SkipPrevious from "@mui/icons-material/SkipPrevious";
import Button from "@mui/material/Button";
import ButtonGroup from "@mui/material/ButtonGroup";
import Stack from "@mui/material/Stack";
import { Dispatch, SetStateAction, useState } from "react";


export default function ControlPanel({ iCurrentMove, setICurrentMove, iCurrentGame, setICurrentGame, gameCount, gameMoveCount }: { iCurrentMove: number, setICurrentMove: Dispatch<SetStateAction<number>>, iCurrentGame: number, setICurrentGame: Dispatch<SetStateAction<number>>, gameCount: number, gameMoveCount: number }) {
    const FIRST_MOVE = iCurrentMove === 0
    const LAST_MOVE = iCurrentMove === gameMoveCount
    const FIRST_GAME = iCurrentGame === 0
    const LAST_GAME = iCurrentGame === gameCount

    const [intervalID, setIntervalID] = useState<undefined | NodeJS.Timeout>(undefined)
    function autoPlay() {
        if (LAST_MOVE) { stopAutoPlay() }
        onNextMove();
    }
    function startAutoPlay() {
        console.debug("Starting autoplay")
        const interval = setInterval(autoPlay, 200

        )
        setIntervalID(interval)
    }
    function stopAutoPlay() {
        console.debug("Stopping autoplay")
        clearInterval(intervalID)
        setIntervalID(undefined)
    }

    function onPreviousMove() {
        if (FIRST_MOVE) { console.debug("Already at first move"); return }
        setICurrentMove(iCurrentMove => iCurrentMove - 1)
        console.debug(`current move lowered: ${iCurrentMove}`)
    }
    function onNextMove() {
        if (LAST_MOVE) { console.debug("Already at last move"); return }
        setICurrentMove(iCurrentMove => iCurrentMove + 1)
        console.debug(`current move upped: ${iCurrentMove}`)
    }
    function onPreviousGame() {
        if (FIRST_GAME) { console.debug("Already at first game"); return }
        setICurrentGame(iCurrentGame => iCurrentGame - 1)
        setICurrentMove(0)
        console.debug(`current game lowered: ${iCurrentGame}`)
    }
    function onNextGame() {
        if (LAST_GAME) { console.debug("Already at last game"); return }
        setICurrentGame(iCurrentGame => iCurrentGame + 1)
        setICurrentMove(0)
        console.debug(`current game upped: ${iCurrentGame}`)
    }
    return (
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
    )
}