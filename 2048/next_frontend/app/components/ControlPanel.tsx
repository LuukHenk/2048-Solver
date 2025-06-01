import ChevronLeft from "@mui/icons-material/ChevronLeft";
import ChevronRight from "@mui/icons-material/ChevronRight";
import Pause from "@mui/icons-material/Pause";
import PlayArrow from "@mui/icons-material/PlayArrow";
import SkipNext from "@mui/icons-material/SkipNext";
import SkipPrevious from "@mui/icons-material/SkipPrevious";
import Button from "@mui/material/Button";
import ButtonGroup from "@mui/material/ButtonGroup";
import Stack from "@mui/material/Stack";
import { Dispatch, SetStateAction, useEffect, useState } from "react";


export default function ControlPanel({ iCurrentMove, setICurrentMove, iCurrentGame, setICurrentGame, gameCount, gameMoveCount }: { iCurrentMove: number, setICurrentMove: Dispatch<SetStateAction<number>>, iCurrentGame: number, setICurrentGame: Dispatch<SetStateAction<number>>, gameCount: number, gameMoveCount: number }) {
    const FIRST_MOVE = iCurrentMove === 0
    const LAST_MOVE = iCurrentMove === gameMoveCount
    const FIRST_GAME = iCurrentGame === 0
    const LAST_GAME = iCurrentGame === gameCount
    const [autoPlayActive, setAutoPlayActive] = useState(false)

    useEffect(() => {
        const interval = setInterval(() => {
            if (LAST_MOVE) {
                setAutoPlayActive(false);
            } else if (autoPlayActive && !LAST_MOVE) {
                onNextMove();
            }
        }, 1)
        return () => clearInterval(interval);
    }, [autoPlayActive, LAST_MOVE, iCurrentMove]);

    function startAutoPlay() {
        console.debug("Starting autoplay")
        setAutoPlayActive(true)
    }
    function stopAutoPlay() {
        console.debug("Stopping autoplay")
        setAutoPlayActive(false)
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
        setICurrentMove(0)
        console.debug(`current game lowered: ${iCurrentGame}`)
    }
    function onNextGame() {
        if (LAST_GAME) { console.debug("Already at last game"); return }
        setICurrentGame(iCurrentGame + 1)
        setICurrentMove(0)
        console.debug(`current game upped: ${iCurrentGame}`)
    }
    return (
        <Stack direction="column" sx={{ marginTop: 2, height: 50 }}>
            <ButtonGroup variant="outlined">
                <Button sx={{ width: "25%" }} aria-label="Previous game" onClick={onPreviousGame} disabled={FIRST_GAME || autoPlayActive} startIcon={<SkipPrevious />}>Previous Game</Button>
                <Button sx={{ width: "25%" }} aria-label="Previous move" onClick={onPreviousMove} disabled={FIRST_MOVE || autoPlayActive} startIcon={<ChevronLeft />}>Previous Move</Button>

                <Button sx={{ width: "25%" }} aria-label="Next move" onClick={onNextMove} disabled={LAST_MOVE || autoPlayActive} endIcon={<ChevronRight />}>Next Move</Button>
                <Button sx={{ width: "25%" }} aria-label="Next game" onClick={onNextGame} disabled={LAST_GAME || autoPlayActive} endIcon={<SkipNext />}>Next Game</Button>
            </ButtonGroup>
            {!autoPlayActive && <Button sx={{ width: "stretch" }} variant="outlined" aria-label="Autoplay" endIcon={<PlayArrow />} onClick={startAutoPlay} disabled={LAST_MOVE}>AutoPlay</Button>}

            {autoPlayActive && <Button variant="outlined" aria-label="StopAutoplay" endIcon={<Pause />} onClick={stopAutoPlay}>Stop autoplay</Button>}
        </Stack>
    )
}