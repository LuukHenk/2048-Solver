export type Board = [
    [string, string, string, string],
    [string, string, string, string],
    [string, string, string, string],
    [string, string, string, string]
]

export interface CurrentMove {
    board: Board
    latestMove: string
    score: string
}

