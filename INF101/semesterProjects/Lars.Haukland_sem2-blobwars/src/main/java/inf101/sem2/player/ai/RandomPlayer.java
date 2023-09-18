package inf101.sem2.player.ai;

import java.util.Collections;
import java.util.List;

import inf101.sem2.game.Game;
import inf101.sem2.player.AbstractPlayer;

/**
 * This Player chooses a random move among all the possible moves.
 * This player can play any game which implements the possibleMoves() method.
 * <p>
 * If a game where no possible moves exist is given, the player will throw an
 * Exception
 *
 * @author Martin Vatshelle - martin.vatshelle@uib.no
 */
public class RandomPlayer extends AbstractPlayer {

	public RandomPlayer(char piece, String name) {
		super(piece, name);
	}

	public RandomPlayer(char piece) {
		super(piece, "Random player");
	}

	@Override
	public <T> T getMove(Game<T> game) {
		List<T> moves = game.getPossibleMoves();
		if (moves.isEmpty()) {
			throw new IllegalStateException("No possible moves to choose, game should have ended!");
		}
		Collections.shuffle(moves);
		return moves.get(0);
	}
}
