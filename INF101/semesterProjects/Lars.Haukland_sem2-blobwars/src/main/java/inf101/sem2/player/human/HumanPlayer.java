package inf101.sem2.player.human;

import inf101.sem2.GUI.Input;
import inf101.sem2.game.Game;
import inf101.sem2.player.AbstractPlayer;

/**
 * Class for players which reciveve human input, 
 * for instance from GUI or console.
 */
public class HumanPlayer extends AbstractPlayer {

	Input input;

	public HumanPlayer(char symbol, String name, Input input) {
		super(symbol, name);
		this.input = input;
	}

	@Override
	public <T> T getMove(Game<T> game) {
		game.displayBoard();
		return input.getMove(game);
	}
}
