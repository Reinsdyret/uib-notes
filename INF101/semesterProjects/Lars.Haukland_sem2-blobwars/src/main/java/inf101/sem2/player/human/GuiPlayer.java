package inf101.sem2.player.human;

import javax.swing.JOptionPane;

import inf101.sem2.GUI.Input;
import inf101.sem2.game.Game;
import inf101.sem2.player.AbstractPlayer;

/**
 * This Player should be used if one wants input from GUI.
 * The game loop will stop when reaching an instance of GuiPlayer
 * and when a mouse click is detected the game loop will resume.
 *
 * @author Martin Vatshelle - martin.vatshelle@uib.no
 */
public class GuiPlayer extends HumanPlayer {

	public GuiPlayer(char piece, String name, Input input) {
		super(piece, name, input);
	}

	public GuiPlayer(char piece, Input input) {
		super(piece, readPlayerName(piece), input);
	}

	@Override
	public <T> T getMove(Game<T> game) {
		game.displayBoard();
		return super.getMove(game);
	}

	/**
	 * Asks player to type in name in a GUI pop up
	 *
	 * @param symbol - The symbol representing this player
	 * @return the name chosen by the player
	 */
	public static String readPlayerName(char symbol) {
		String name = null;
		while (!AbstractPlayer.isValidName(name)) {
			name = JOptionPane.showInputDialog("Player " + symbol + ". Type in your name.");
		}
		return name;
	}

}
