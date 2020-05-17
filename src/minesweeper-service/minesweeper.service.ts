import { Inject } from "@angular/core";
import { Cell } from 'src/models/Cell';

@Inject({
    providedIn: 'root'
})
export class MinesweeperService {
    constructor() {
        
    }

    getNeighbors = (row, column) => {
        return [
          {
            row: row - 1,
            column: column
          },
          {
            row: row - 1,
            column: column -1
          },
          {
            row: row - 1,
            column: column + 1
          },
          {
            row: row + 1,
            column
          },
          {
            row: row + 1,
            column: column - 1
          },
          {
            row: row + 1,
            column: column + 1
          },
          {
            row,
            column: column - 1
          },
          {
            row,
            column: column + 1
          }
        ]
      }
      
      initializeBoard() {
        return new Promise<Cell[]>((resolve, reject) => {
          const rows = 10;
          const columns = 10;
        
          let board = Array(rows * columns).fill(0).map((pr, index) => {
            
            return {
              id: index,
              row: Math.floor(index / rows),
              column: index % columns,
              hasBomb: Boolean(Math.floor(Math.random() * 1.04)),
              isFlagged: false,
              isRevealed: false,
              bombCount: 0
            }
          });
        
          board = board.map(pr => {
            const computedNeighbors = this.getNeighbors(pr.row, pr.column);
            let bombCount = null;
        
            if (!pr.hasBomb) {
              bombCount = board.filter(cell => computedNeighbors
                .some(pr => pr.row === cell.row && pr.column === cell.column))
                  .reduce((acc, cell) => acc + Number(cell.hasBomb), 0)
            }      
        
            return {
              ...pr,
              bombCount
            }
          })
        
          resolve(board)
        })
      }
      
      computeNeighbors = (board: Cell[], cell: Cell) => {
        const relativeNeighbors = [{
          row: cell.row,
          column: cell.column + 1
        },
        {
          row: cell.row,
          column: cell.column - 1
        },
        {
          row: cell.row + 1,
          column: cell.column
        },
        {
          row: cell.row - 1,
          column: cell.column
        }];
      
        const neighbors = board.filter(cell => relativeNeighbors.some(pr => pr.column === cell.column && pr.row === cell.row));
      
        return neighbors.map(pr => board
          .find(cell => cell.row === pr.row && cell.column === pr.column));
      }
      
      uncoverNeighbors = (board: Cell[], cell: Cell) => {
        let neighbors = this.computeNeighbors(board, cell);
        let checkedNeighbors: Cell[] = [];
        let result = [];
      
        while(neighbors.length) {
          const firstCell = neighbors.pop();
          const isChecked = checkedNeighbors.find(cell => cell.id === firstCell.id);
      
          if(isChecked || firstCell.hasBomb) {
            continue;
          }
      
          if(!firstCell.bombCount) {
            neighbors = [...neighbors, ...this.computeNeighbors(board, firstCell)];
          }
      
          checkedNeighbors = [...checkedNeighbors, firstCell];
          result = [...result, firstCell];
        }
        return result;
      }
}