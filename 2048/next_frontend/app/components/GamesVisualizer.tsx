"use client"

import Stack from "@mui/material/Stack"
import GameGrid from "./gameGrid/GameGrid"
import { CurrentMove } from "./types"
import { useState } from "react"
import ControlPanel from "./ControlPanel"

export default function GamesVisualzer({ games }: { games: [CurrentMove[]] }) {
    const [iCurrentGame, setICurrentGame] = useState(0)
    const [iCurrentMove, setICurrentMove] = useState(0)
    const CURRENT_MOVE = games[iCurrentGame][iCurrentMove]
    const GAME_MOVE_COUNT = games[iCurrentGame].length - 1
    const GAME_COUNT = games.length - 1

    return <Stack sx={{ width: "100%", height: "100vh", alignItems: "center", justifyContent: "center", display: "flex" }}>
        <GameGrid board={CURRENT_MOVE.board} />
        <ControlPanel
            iCurrentGame={iCurrentGame}
            iCurrentMove={iCurrentMove}
            setICurrentGame={setICurrentGame}
            setICurrentMove={setICurrentMove}
            gameCount={GAME_COUNT}
            gameMoveCount={GAME_MOVE_COUNT}
        />
    </Stack>
}