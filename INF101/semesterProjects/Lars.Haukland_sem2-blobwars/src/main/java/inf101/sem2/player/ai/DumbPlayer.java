package inf101.sem2.player.ai;

import inf101.sem2.game.Game;
import inf101.sem2.player.AbstractPlayer;

public class DumbPlayer extends AbstractPlayer {

	static int counter = 1;

	public DumbPlayer(char symbol) {
		super(symbol, "DumbPlayer " + counter++);
	}

	@Override
	public <T> T getMove(Game<T> game) {
		for (T move : game.getPossibleMoves()) {
			return move;
		}
		return null;
	}

}
