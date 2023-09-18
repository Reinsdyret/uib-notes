package cellular;

import java.util.Random;

import datastructure.CellGrid;
import datastructure.IGrid;

/**
 * 
 * A CellAutomata that implements Conways game of life.
 * 
 * @see CellAutomaton
 * 
 *      Every cell has two states: Alive or Dead. Each step the state of each
 *      cell is decided from its neighbors (diagonal, horizontal and lateral).
 *      If the cell has less than two alive Neighbors or more than three
 *      neighbors the cell dies. If a dead cell has exactly three neighbors it
 *      will become alive.
 * 
 * @author eivind
 * @author Martin Vatshelle - martin.vatshelle@uib.no
 * @author Sondre Bolland - sondre.bolland@uib.no
 */
public class GameOfLife implements CellAutomaton {

	/**
	 * The grid of cells
	 */
	IGrid currentGeneration;

	/**
	 * 
	 * Construct a Game Of Life Cell Automaton that holds cells in a grid of the
	 * provided size
	 * 
	 * @param height The height of the grid of cells
	 * @param width  The width of the grid of cells
	 */
	public GameOfLife(int rows, int columns) {
		currentGeneration = new CellGrid(rows, columns, CellState.DEAD);
		initializeCells();
	}

	@Override
	public void initializeCells() {
		Random random = new Random();
		for (int row = 0; row < currentGeneration.numRows(); row++) {
			for (int col = 0; col < currentGeneration.numColumns(); col++) {
				if (random.nextBoolean()) {
					currentGeneration.set(row, col, CellState.ALIVE);
				} else {
					currentGeneration.set(row, col, CellState.DEAD);
				}
			}
		}
	}

	@Override
	public int numberOfRows() {
		// TODO
		return this.currentGeneration.numRows();
	}

	@Override
	public int numberOfColumns() {
		// TODO
		return this.currentGeneration.numColumns();
	}

	@Override
	public CellState getCellState(int row, int col) {
		// TODO
		return this.currentGeneration.get(row, col);
	}

	@Override
	public void step() {
		IGrid nextGeneration = currentGeneration.copy();
		// TODO
		for(int row = 0; row < nextGeneration.numRows(); row++) {
			for (int col = 0; col < nextGeneration.numColumns(); col++) {
				nextGeneration.set(row, col, this.getNextCell(row, col));
			}
		}
	}

	@Override
	public CellState getNextCell(int row, int col) {
		// TODO
		CellState currentState = this.currentGeneration.get(row, col);
		
		int aliveNeighbours = this.countNeighbors(row, col, CellState.ALIVE);
		
		if (currentState == CellState.ALIVE) {
			if(aliveNeighbours < 2 || aliveNeighbours > 3) {
				return CellState.DEAD;
			}else if (aliveNeighbours == 2 || aliveNeighbours == 3) {
				return CellState.ALIVE;
			}
		}else{
			if(aliveNeighbours == 3) return CellState.ALIVE;
		}
		return CellState.DEAD;
	}

	/**
	 * Calculates the number of neighbors having a given CellState of a cell on
	 * position (row, col) on the board
	 * 
	 * Note that a cell has 8 neighbors in total, of which any number between 0 and
	 * 8 can be the given CellState. The exception are cells along the boarders of
	 * the board: these cells have anywhere between 3 neighbors (in the case of a
	 * corner-cell) and 5 neighbors in total.
	 * 
	 * @param x     the x-position of the cell
	 * @param y     the y-position of the cell
	 * @param state the Cellstate we want to count occurences of.
	 * @return the number of neighbors with given state
	 */
	private int countNeighbors(int row, int col, CellState state) {
		// TODO
		int stateCount = 0;
		int rows = this.currentGeneration.numRows();
		int cols = this.currentGeneration.numColumns();
		
		/*int lowerBoundRow = -1;
		int lowerBoundCol = -1;
		int upperBoundRow = 1;
		int upperBoundCol = 1;
		
		
		if(row == 0 || row == rows - 1) {
			
			// If at top change upper bound, else nothing
			upperBoundRow = row == 0 ? upperBoundRow++ : upperBoundRow;
			// If at bottom change lower bound, else nothing 
			lowerBoundRow = row == rows - 1 ? lowerBoundRow++ : lowerBoundRow;
			
			if(col == 0 || col == cols - 1) {
				
				// If at the far left change lower bound, else nothing
				lowerBoundCol = col == 0 ? lowerBoundCol++ : lowerBoundCol;
				// If at the far right change upper bound, else nothing
				upperBoundCol = col == cols - 1 ? upperBoundCol-- : upperBoundCol;
			}
		}
		else if(col == 0 || col == cols - 1) {
			
			// If at the far left change lower bound, else nothing
			lowerBoundCol = col == 0 ? lowerBoundCol++ : lowerBoundCol;
			// If at the far right change upper bound, else nothing
			upperBoundCol = col == cols - 1 ? upperBoundCol-- : upperBoundCol;
		}
		
		for(int i = lowerBoundRow; i<= upperBoundRow; i++) {
			for(int j = lowerBoundCol; j<=upperBoundCol; j++) {
				// Continues when its looping through own cell
				if(i == 0 && j == 0) {
					continue;
				}
				
				CellState cell = this.currentGeneration.get(i + row, j + col);
				
				if (cell.equals(state)) stateCount++;
			}
		}*/
		
		
		
		
				
		// Defining the block of which the given cell is in the middle NOT IN USE CAUSE LAZY OPPGAV
		
		CellState[][] block = {
				{
				this.currentGeneration.get((row +  rows - 1) % rows, (col - 1 + cols) % cols),
				this.currentGeneration.get((row +  rows - 1) % rows, cols),
				this.currentGeneration.get((row +  rows - 1) % rows, (col + 1) % cols)
				},
				{
				this.currentGeneration.get((row ), (col - 1 + cols) % cols),
				this.currentGeneration.get(row,col),
				this.currentGeneration.get((row), (col + 1) % cols)
				},
				{
				this.currentGeneration.get((row +  rows + 1) % rows, (col - 1 + cols) % cols),
				this.currentGeneration.get((row +  rows + 1) % rows, (col)),
				this.currentGeneration.get((row +  rows - 1) % rows, (col + 1) % cols),
				}
		};
		
		/*for(int i = 0; i<block.length; i++) {
			for(int j = 0; j<block[i].length; j++) {
				if(i == 1 && j == 1) {
					continue;
				}
				
				if(block[i][j] == state) {
					stateCount++;
				}
			}
		}*/
		
		return stateCount;
	}

	@Override
	public IGrid getGrid() {
		return this.currentGeneration;
	}
}
