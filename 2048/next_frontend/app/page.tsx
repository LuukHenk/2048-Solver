"use server"
import Box from "@mui/material/Box";
import GameGrid from "./components/gameGrid/GameGrid";
import { Button, ButtonGroup, Stack } from "@mui/material";
import { promises } from "fs";


async function loadData() {
    let file = undefined
    try { file = await promises.readFile(process.cwd() + '/../data/resuldssdsdts.json', 'utf8'); } catch (error) {
        console.error(`${error}.\nPlease supply data before rendering. Loading dummy data instead.`)
    }
    if (file === undefined) {
        file = await promises.readFile(process.cwd() + "/dummyGame.json", 'utf8');
    }

    return JSON.parse(file);
}

export default async function Home() {
    const gamesData = await loadData()
    return <Stack sx={{ width: "100%", height: "100vh", alignItems: "center", justifyContent: "center", display: "flex" }}>
        <GameGrid />
        <ButtonGroup variant="outlined">
            <Button></Button>
            <Button>Previous Move</Button>
            <Button>Next Move</Button>
        </ButtonGroup>
    </Stack>
}
