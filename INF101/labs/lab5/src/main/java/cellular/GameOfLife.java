package cellular;

import java.util.Random;

import cellular.cellstate.CellState;
import cellular.cellstate.ICellState;
import datastructure.Grid;
import datastructure.GridDirection;
import datastructure.IGrid;
import datastructure.Location;

/**
 * A Cell Automata that implements Conway's Game of Life.
 * <p>
 * Every cell has two states: Alive or Dead.
 * Each cell (except along the border) has eight neighbors: diagonal, horizontal and lateral.
 * <p>
 * On each step the cell state is updated according to the Game of Life rules:
 * <p>
 * Any live cell with fewer than two live neighbors dies, as if by underpopulation.
 * Any live cell with two or three live neighbors lives on to the next generation.
 * Any live cell with more than three live neighbors dies, as if by overpopulation.
 * Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
 * @author eivind, Anna Eilertsen - anna.eilertsen@uib.no
 */
public class GameOfLife implements ICellAutomaton {

	/**
	 * The grid of cells
	 */
	private IGrid<ICellState> currentGeneration;

	/**
	 * Construct a Game Of Life Cell Automaton that holds cells in a grid of the provided size
	 * @param columns The height of the grid of cells
	 * @param rows The width of the grid of cells
	 */
	public GameOfLife(int rows, int columns) {
		currentGeneration = new Grid<>(rows, columns, CellState.DEAD);
	}

	@Override
	public void initializeCells() {
		Random random = new Random();
		for(Location loc : currentGeneration.locations()) {
			if(random.nextBoolean()) {
				currentGeneration.set(loc, CellState.ALIVE);
			} else {
				currentGeneration.set(loc, CellState.DEAD);
			}
		}
	}

	@Override
	public int numberOfRows() {
		return currentGeneration.numRows();
	}

	@Override
	public int numberOfColumns() {
		return currentGeneration.numColumns();
	}

	@Override
	public ICellState getCellState(Location loc) {
		return currentGeneration.get(loc);
	}

	@Override
	public void step() {

		IGrid nextGeneration = new Grid(
				currentGeneration.numRows(), currentGeneration.numColumns(),
				CellState.ALIVE);

		for(Location loc : currentGeneration.locations()) {
			ICellState newState = getNextState(loc);
			nextGeneration.set(loc, newState);
		}

		currentGeneration = nextGeneration;

	}

	/**
	 * This method implements the rules of the game
	 * For a given location these rules decide what state this location
	 * will turn into next time the step() method is called.
	 * 
	 * @param loc the location
	 * @return state of loc as it will be at next step
	 */
	private ICellState getNextState(Location loc) {
		int numAlive = countNeighbors(loc, CellState.ALIVE);
		boolean isAlive = getCellState(loc) == CellState.ALIVE;
		boolean isDead = getCellState(loc) == CellState.DEAD;

		if(isAlive && numAlive < 2) {
			return CellState.DEAD;
		}
		if(isAlive && (numAlive == 2 || numAlive == 3)) {
			return CellState.ALIVE;
		}
		if(isAlive && numAlive > 3) {
			return CellState.DEAD;
		}
		if(isDead && numAlive == 3) {
			return CellState.ALIVE;
		}

		return getCellState(loc);
	}

	/**
	 * This method counts the number of neighbors that have a given state.
	 * There are 8 possible neighbors (unless the location is by the edge of the grid)
	 * @param loc the location whose neighbors to count
	 * @param state the state to count
	 * @return number of neighbors having the given state
	 */
	private int countNeighbors(Location loc, CellState state) {
		int numNeighbors = 0;
		for(GridDirection dir : GridDirection.EIGHT_DIRECTIONS) {
			Location neighbor = loc.getNeighbor(dir);

			if(currentGeneration.isOnGrid(neighbor)) {
				if(currentGeneration.get(neighbor) == state) {
					numNeighbors++;
				}
			}
		}
		return numNeighbors;
	}

	@Override
	public Iterable<Location> locations() {
		return currentGeneration.locations();
	}
}
