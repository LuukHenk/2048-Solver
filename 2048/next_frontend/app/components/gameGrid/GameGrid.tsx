"use client"

import Grid from '@mui/material/Grid';
import GameTile from './GameTile';
import { BASE_TILE_COLOR } from './constants';


export default function GameGrid() {
    return (

        <Grid container sx={{ width: "auto", height: {xs: "50%", md: "80%"}, aspectRatio: "1/1", backgroundColor: BASE_TILE_COLOR, padding: 1}}>
            <Grid size={3}>
                <GameTile value={2} />
                <GameTile value={4} />
                <GameTile value={8} />
                <GameTile value={16} />
            </Grid>
            <Grid size={3}>
                <GameTile value={32} />
                <GameTile value={64} />
                <GameTile value={128} />
                <GameTile value={256} />
            </Grid>
            <Grid size={3}>
                <GameTile value={512} />
                <GameTile value={1024} />
                <GameTile value={2048} />
                <GameTile value={4096} />
            </Grid>
            <Grid size={3}>
                <GameTile value={8192} />
                <GameTile value={16384} />
                <GameTile value={32768} />
                <GameTile value={65536} />
            </Grid>
        </Grid>
    );
}