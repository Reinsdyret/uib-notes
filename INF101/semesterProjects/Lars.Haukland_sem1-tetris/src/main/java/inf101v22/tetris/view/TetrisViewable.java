package inf101v22.tetris.view;

import java.util.Iterator;

import inf101v22.grid.CoordinateItem;
import inf101v22.tetris.model.GameScreen;
import inf101v22.tetris.model.Tile;

public interface TetrisViewable {

	/**
	 * @return Number of rows in the tetris grid
	 */
	public int getRows();

	/**
	 * @return Number of columns in the tetris grid
	 */
	public int getCols();

	/**
	 * Get the running game score
	 * @return the gamescore
	 */
	public int getScore();


	/**
	 * Makes an iterator of all cells with their coordinates and their tiles
	 * @return Iterator<CoordinateItem<Tile>> with all cells on grid
	 */
	public Iterable<CoordinateItem<Tile>> Iterable();

	/**
	 * Gets the falling piece and returns it as an iterator
	 * @return the falling piece as an iterator with coords and tile
	 */
	public Iterator<CoordinateItem<Tile>> getPiece();

	/**
	 * Gets the current gameScreen
	 * @return the gameScreen
	 */
	public GameScreen getGameScreen();
}
