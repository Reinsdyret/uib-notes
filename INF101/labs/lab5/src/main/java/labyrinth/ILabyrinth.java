package labyrinth;

import java.awt.Color;

import datastructure.GridDirection;
import datastructure.Location;

public interface ILabyrinth {

	/**
	 * Get labyrinth cell contents at a location
	 * @param loc
	 * @return The tile at loc
	 * @throws IllegalArgumentException unless 0 <= x < {@link #numberOfColumns()} and 0 <= y <
	 * {@link #numberOfRows()}
	 */
	LabyrinthTile getCell(Location loc);

	/**
	 * Get the color of the cell in a given row,col location.
	 * @param x
	 * @param y
	 * @return The color of the cell in the given row and column.
	 */
	Color getColor(Location loc);

	/**
	 * @return The number of rows.
	 */
	int numberOfRows();

	/**
	 * @return Current amount of player gold
	 */
	int getPlayerGold();

	/**
	 * @return Current number of player hitpoints
	 */
	int getPlayerHitPoints();

	/**
	 * @return The number of columns.
	 */
	int numberOfColumns();

	/**
	 * @return True if the game is active
	 */
	boolean isPlaying();

	/**
	 * Move player one step in direction dir
	 * If player can not move in that direction leave the player in same position.
	 * @param d The direction the player should move in
	 * @throws MovePlayerException Player cant move in that direction d
	 */
	void movePlayer(GridDirection dir) throws MovePlayerException;

	/**
	 * Check if a move is valid.
	 * A move is considered invalid if there is a wall in the new position
	 * or if the move brings the player outside the bounds of the grid.
	 * 
	 * @param d The direction the play should move in
	 * @return True if movePlayer(d) is a valid move
	 */
	boolean playerCanGo(GridDirection d);

	/**
	 * Iterable over all Locations in this CellAutomaton
	 * @see IGrid#locations()
	 */
	Iterable<Location> locations();

}
