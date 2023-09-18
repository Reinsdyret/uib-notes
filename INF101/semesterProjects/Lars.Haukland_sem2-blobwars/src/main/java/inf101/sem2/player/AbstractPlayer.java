package inf101.sem2.player;

/**
 * This class takes care of the name and symbol handling in Player classes.
 * Using an abstract class here means we avoid duplicating this code in each
 * class of Players.
 * <p>
 * Each player must have a symbol to be displayed on the board
 * and a name to be displayed in game messages.
 *
 * @author Martin Vatshelle - martin.vatshelle@uib.no
 */
public abstract class AbstractPlayer implements Player {

	protected char symbol;
	protected String name;

	public AbstractPlayer(char symbol) {
		this(symbol, "Player " + symbol);
	}

	public AbstractPlayer(char symbol, String name) {
		this.symbol = symbol;
		this.name = name;
	}

	@Override
	public char getSymbol() {
		return symbol;
	}

	@Override
	public boolean equals(Object obj) {
		if (obj instanceof AbstractPlayer) {
			AbstractPlayer player = (AbstractPlayer) obj;
			return this.symbol == player.symbol;
		}
		return false;
	}

	@Override
	public String toString() {
		return name;
	}

	/**
	 * Checks if a given String is a valid player name
	 *
	 * @param name - the name to check
	 * @return the name if it is valid, otherwise an Exception will be thrown
	 */
	public static String validateName(String name) {
		if (!isValidName(name)) {
			throw new IllegalArgumentException("Name can not be blank");
		}
		return name;
	}

	/**
	 * Checks if a given string is a valid player name
	 *
	 * @param name - the name to check
	 * @return true if the name is valid, false otherwise.
	 */
	public static boolean isValidName(String name) {
		return name != null && !name.isBlank();
	}
}
