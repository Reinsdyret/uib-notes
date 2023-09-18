package inf101v22.tetris.controller;

import java.util.Iterator;

import inf101v22.grid.CoordinateItem;
import inf101v22.tetris.model.GameScreen;
import inf101v22.tetris.model.Tile;

public interface TetrisControllable {

	/**
	 * Will be used for moving the falling piece by deltaRows and deltaCol
	 * @param deltaRow
	 * @param deltaCol
	 * @return A boolean representing if the moving was done or not
	 */
	public boolean moveFallingPiece(int deltaRow, int deltaCol);

	/**
	 * Get number of rows
	 * @return rows
	 */
	public int getRows();

	/**
	 * Rotates the falling piece
	 * @return a boolean representing whether this movement was possible or not
	 */
	public boolean rotatePiece();

	/**
	 * Handle the dropping of a tetris piece. And handles game over.
	 */
	public void handleDropPiece();

	/**
	 * Gets the current gameScreen
	 * @return the gameScreen
	 */
	public GameScreen getGameScreen();

	/**
	 * Sets the gameScreen to ACTIVE_GAME
	 */
	public void setActiveGame();

	/**
	 * Get the amount in ms of time between each fall step for falling piece.
	 * @return time in ms
	 */
	public int getFallTime();

	/**
	 * If possible, moves the falling piece one row further down
	 * If not, locks the piece to the board and creates a new piece
	 */
	public void clockTick();

	public Iterator<CoordinateItem<Tile>> getPiece();

}
