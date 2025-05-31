"use server"
import { promises } from "fs";
import { CurrentMove } from "./components/types";
import GamesVisualzer from "./components/GamesVisualizer";


async function loadGames(): Promise<[CurrentMove[]]> {
    let file = undefined
    try {
        file = await promises.readFile(process.cwd() + '/../data/results.json', 'utf8');
    } catch (error) {
        console.error(`${error}.\nPlease supply data before rendering. Loading dummy data instead.`)
    }
    if (file === undefined) {
        file = await promises.readFile(process.cwd() + "/dummyGame.json", 'utf8');
    }

    return JSON.parse(file);
}

export default async function Home() {
    const GAMES = await loadGames()
    return <GamesVisualzer games={GAMES}/>
}
