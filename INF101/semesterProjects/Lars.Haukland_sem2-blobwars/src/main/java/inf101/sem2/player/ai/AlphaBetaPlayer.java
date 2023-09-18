package inf101.sem2.player.ai;

import inf101.sem2.game.Game;
import inf101.sem2.player.AbstractPlayer;
import inf101.sem2.player.Player;

/**
 * This AI is based on an algorithm that is not curriculum for INF101, but maybe
 * for INF102.
 * Since many of the students had tried implementing such an AI I have included
 * one in this solution.
 * But such a level of AI is not expected at the level of INF101.
 * <p>
 * This player will work both for TicTacToe, Connect4, Othello
 * and also for other 2 player games.
 * <p>
 * This player uses a minimax strategy to search for the best move, with
 * AlphaBeta pruning.
 * That means that it tries all possible moves and gives a score to each move.
 * The score is given by calling the method recursively for the other player to
 * make the move that is best for him.
 * We assume that what is best for you is worst for your opponent,
 * otherwise this strategy will not work.
 * One can define their own score method in the respective games to better
 * evaluate
 * a game state whether a player wants to select those moves or not.
 * <p>
 * 
 * Note that the sum of the scores for both players always is 0.
 *
 *
 * @author Sondre Bolland - sondre.bolland@uib.no
 * @author Martin Vatshelle - martin.vatshelle@uib.no
 * 
 */
public class AlphaBetaPlayer extends AbstractPlayer {
	/**
	 * Defines how many steps ahead the search should continue
	 */
	int depth;
	/**
	 * Player variable to keep track of who wants to
	 * maximize and who wants to minimize the score at a given move.
	 * Needed to update <code>alpha</code> and <code>beta<\code>
	 */
	Player currentPlayer;

	public AlphaBetaPlayer(char piece, int level) {
		super(piece, "AlphaBeta");
		depth = level;
	}

	@Override
	public <T> T getMove(Game<T> game) {
		game.displayMessage(name + " is thinking...");
		currentPlayer = game.getCurrentPlayer();
		Strategy<T> best = bestMove(game, depth, Integer.MIN_VALUE, Integer.MAX_VALUE);
		return best.move;
	}

	/**
	 * Chooses the move that maximizes the players score
	 *
	 * @param game
	 * @param depth
	 * @return
	 */
	private <T> Strategy<T> bestMove(Game<T> game, int depth, int alpha, int beta) {
		Strategy<T> best = null;
		int bestScore = 0;

		// try each possible strategy
		for (T move : game.getPossibleMoves()) {
			// make a copy of the game and try the move
			Game<T> newGame = game.copyGameWithoutGraphics();
			newGame.makeMove(move); // note that this changes the current player in the copy but not the real game
			newGame.nextPlayer();

			Strategy<T> current = null;
			if (newGame.gameOver() || depth == 1 || newGame.getPossibleMoves().isEmpty()) { // No more moves can be made
				current = new Strategy<T>(move, newGame);
			} else {
				// call recursively such that the opponent makes the move that is best for him
				// change the sign since this is the score of the best move for opponent
				current = bestMove(newGame, depth - 1, alpha, beta);
				current.move = move;
			}

			int score = current.game.score(game.getCurrentPlayer());
			// keep the best Strategy
			if (best == null || score > bestScore) {
				best = current;
				bestScore = score;
			}
			// Update alpha and beta based on if the player wants
			// to maximize or minimize move choice
			if (game.getCurrentPlayer().equals(currentPlayer)) {
				alpha = Math.max(alpha, bestScore);
			} else {
				beta = Math.min(beta, -bestScore);
			}
			if (beta <= alpha)
				break;

		}
		if (best == null) {
			System.err.println("This should not happen! No moves possible?");
		}
		return best;

	}
}

/**
 * A inner class only to be used by MiniMax player
 * This class keeps track of a move and a score associated with that move
 *
 * @author Martin Vatshelle - martin.vatshelle@uib.no
 */
class Strategy<MoveType> {
	MoveType move;
	Game<MoveType> game;

	public Strategy(MoveType move, Game<MoveType> game) {
		this.move = move;
		this.game = game;
	}
}
