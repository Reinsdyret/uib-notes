package inf101.sem2.player.human;

import static inf101.sem2.terminal.TerminalInput.readInt;
import static inf101.sem2.terminal.TerminalInput.readName;

import java.util.Scanner;

import inf101.sem2.terminal.ConsolePlayerInput;

/**
 * This Player enables the user to play the game by using the keyboard.
 * There is in reality two different players here,
 * one for TicTacToe and one for FourInARow
 * If you check out the makeMove method it is only an if else choosing between
 * the two types
 * <p>
 * I thought still it was good to keep all this code in one class as then
 * all players can be used in both games.
 *
 * @author Martin Vatshelle - martin.vatshelle@uib.no
 */
public class ConsolePlayer extends HumanPlayer {

	/**
	 * The Scanner used to read from the terminal.
	 * We use one single static Scanner for all players
	 * rather than starting a new Scanner as all ConsolePlayers
	 * on the same computer will be typing on the same keyboard.
	 * If this game were to be played over network this might need to change.
	 */
	static Scanner sc = new Scanner(System.in);

	/**
	 * Creates new console player
	 *
	 * @param symbol
	 * @param name
	 */
	public ConsolePlayer(char symbol, String name, ConsolePlayerInput input) {
		super(symbol, name, input);
		input.setPlayer(this);
	}

	/**
	 * Creates new Console player by reading name from Console
	 *
	 * @param symbol
	 */
	public ConsolePlayer(char symbol) {
		this(symbol, readPlayerName(symbol), new ConsolePlayerInput());
	}

	/**
	 * Reads in a player name, only requirement is that name is not empty
	 *
	 * @param sc
	 */
	public static String readPlayerName(char symbol) {
		System.out.println("Player " + symbol + " type your name:");
		return readName(sc, "Name");
	}
}
