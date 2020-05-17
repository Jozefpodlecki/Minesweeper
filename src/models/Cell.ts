export interface Cell {
    id: number;
    row: number;
    column: number;
    hasBomb: boolean;
    isFlagged: boolean;
    isRevealed: boolean;
    bombCount: number;
}