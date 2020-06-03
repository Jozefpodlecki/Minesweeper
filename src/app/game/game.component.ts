import { Component, OnInit } from '@angular/core';
import { faBomb, faFlag, faSyncAlt, faSmile, faFrown } from '@fortawesome/free-solid-svg-icons';
import { trigger, state, style, transition, animate } from '@angular/animations';
import { Cell } from 'src/models/Cell';
import { MinesweeperService } from 'src/minesweeper-service/minesweeper.service';
import { GameState } from 'src/models/GameState';
import { GameOptions } from 'src/models/GameOptions';

@Component({
  selector: 'game',
  templateUrl: './game.component.html',
  styleUrls: ['./game.component.less'],
  animations: [
    trigger('enterTrigger', [
      //transition(':leave', [style({opacity: '0'}), animate('1500ms')]),
      transition('void => *', [style({opacity: '0'}), animate('1500ms')])
    ])
  ]
})
export class GameComponent implements OnInit {
  faBomb = faBomb;
  faFlag = faFlag;
  faSyncAlt = faSyncAlt;
  faSmile = faSmile;
  faFrown = faFrown;

  board: Cell[];
  state: GameState;
  disabled: boolean;
  GameState = GameState;
  numberOfBombs: number;

  constructor(private _minesweeperService: MinesweeperService) {
    this.numberOfBombs = 10;
    this.initialize();
  }

  initialize() {
    this.disabled = true;
    this.state = GameState.loading;
    const options: GameOptions = {
      randomizer: () => Boolean(Math.floor(Math.random() * 2)),
      numberOfBombs: this.numberOfBombs
    }

    this._minesweeperService.initializeBoard(options).then(board => {
      this.board = board;
      this.state = GameState.game;
      this.disabled = false;
    });
  }

  ngOnInit(): void {
  }

  computeState(cell: any) {
    if(cell.isRevealed) {
      if(cell.hasBomb) {
        return 'bomb'
      }
      if(cell.bombCount) {
        return 'count'
      }
      return 'revealed'
    }
    if(cell.isFlagged) {
      return 'flag'
    }
    return 'unrevealed'
  }

  click(cell: Cell) {
    cell.isRevealed = true;
    
    if(cell.hasBomb) {
      this.gameOver();
      return;
    }

    if(!cell.bombCount) {
      const cells = this._minesweeperService.uncoverNeighbors(this.board, cell);

      for(let cell of cells) {  
        cell.isRevealed = true;
      }
    }

    this.checkIfGameOver();
  }

  checkIfGameOver() {
    const allRevealed = this.board.some(({isRevealed}) => isRevealed === true);
    const allFlagged = !this.board.some(({hasBomb, isFlagged}) => hasBomb !== isFlagged);

    if(allFlagged && allRevealed) {
      this.setState(GameState.won);
    }
  }

  onRightClick(cell: Cell, event: MouseEvent) {
    cell.isFlagged = !cell.isFlagged;
    this.checkIfGameOver();
    event.preventDefault();
  }

  onNumberOfBombsChange(value: string) {
    this.numberOfBombs = Number(value);
    this.initialize();
  }

  gameOver() {
    this.disabled = true;
    for(let cell of this.board) {
      cell.isRevealed = true;
    }
    this.setState(GameState.lost);
  }

  setState(state: GameState) {
    setTimeout(() => {
      this.state = state;
    }, 2000)
  }

  trackById(index: number, cell: Cell){
    return cell.id; 
  }

}
