export type Board = [
    [string, string, string, string],
    [string, string, string, string],
    [string, string, string, string],
    [string, string, string, string]
]

export interface CurrentMove {
    board: Board
    performedMove: string
    score: string
}

