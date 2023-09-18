package inf101.sem2.terminal;

import inf101.sem2.GUI.Graphics;
import inf101.sem2.game.GameBoard;

/**
 * Simple Graphics that prints to the screen
 * 
 * @author Martin Vatshelle - martin.vatshelle@uib.no
 *
 */
public class TerminalGraphics implements Graphics {

	@Override
	public void displayMessage(String string) {
		System.out.println(string);
	}

	@Override
	public void display(GameBoard board) {
		System.out.println(board.toString());
	}
}
