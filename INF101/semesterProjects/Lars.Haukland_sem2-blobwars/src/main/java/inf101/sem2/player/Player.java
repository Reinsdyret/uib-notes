package inf101.sem2.player;

import inf101.sem2.game.Game;

/**
 * Players has only one method that is specific to the game they play.
 * namely getMove(), this method contains the logic of the player.
 * There are normally differences in how a player chooses a move, e.g.
 * in TicTacToe the player chooses x and y while in FourInARow the player
 * chooses a column. Therefore one could imagine that different classes
 * are needed for each type of game.
 * However when possible one should make implement general
 * playing strategies that can be reused in several games.
 * That is done in this solution.
 *
 * @author Martin Vatshelle - martin.vatshelle@uib.no
 */
public interface Player {

	/**
	 * The char symbol unique to each Player to use when printing the GameBoard.
	 * The return type is String to allow for EMOJIs but should only be 1 character.
	 * TODO: this only works for certain font types so make sure monospace font is
	 * used.
	 *
	 * @return a 1 character symbol unique to each player
	 */
	char getSymbol();

	/**
	 * Gets a move from a player. The game will decide if the returned move is a
	 * legal move.
	 * If the player returns an illegal move the rules of the game decides if the
	 * player will loose his turn or get to retry
	 *
	 * @param game - The grid based game in which the move is to be made.
	 * @return a move describing what locations the player should traverse
	 */
	<T> T getMove(Game<T> board);

}
