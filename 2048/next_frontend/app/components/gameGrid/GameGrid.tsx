"use client"

import Grid from '@mui/material/Grid';
import GameTile from './GameTile';
import { BASE_TILE_COLOR, Board } from './constants';


export default function GameGrid({ board }: { board: Board }) {
    return (

        <Grid container sx={{ width: "auto", height: { xs: "50%", md: "80%" }, aspectRatio: "1/1", backgroundColor: BASE_TILE_COLOR, padding: 1 }}>
            <Grid size={3}>
                <GameTile value={board[0][0]} />
                <GameTile value={board[1][0]} />
                <GameTile value={board[2][0]} />
                <GameTile value={board[3][0]} />
            </Grid>
            <Grid size={3}>
                <GameTile value={board[0][1]} />
                <GameTile value={board[1][1]} />
                <GameTile value={board[2][1]} />
                <GameTile value={board[3][1]} />
            </Grid>
            <Grid size={3}>
                <GameTile value={board[0][2]} />
                <GameTile value={board[1][2]} />
                <GameTile value={board[2][2]} />
                <GameTile value={board[3][2]} />
            </Grid>
            <Grid size={3}>
                <GameTile value={board[0][3]} />
                <GameTile value={board[1][3]} />
                <GameTile value={board[2][3]} />
                <GameTile value={board[3][3]} />
            </Grid>
        </Grid>
    );
}