package inf101.sem2.GUI;

import inf101.sem2.game.GameBoard;

/**
 * This interface describes how to display game information for human players.
 * A player needs to know the current state of the GameBoard in order to make a
 * move.
 * And a player needs to be informed of events in the game such as when someone
 * wins the game.
 * 
 * @author Martin Vatshelle - martin.vatshelle@uib.no
 *
 */
public interface Graphics {

	/**
	 * Displays a text message
	 * 
	 * @param string - the message to display
	 */
	void displayMessage(String string);

	/**
	 * Displays the current state of a given board
	 * In order to display a board there needs to be a mapping from
	 * Player to symbol defined within the graphics object, if the players
	 * appearing on the board does not match the players in the mapping the
	 * board might not be displayed correctly.
	 * 
	 * @param board - the board to display.
	 */
	void display(GameBoard board);

	default void sleep(int delay) {

	}

}
