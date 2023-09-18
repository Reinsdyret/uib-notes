package labyrinth;

import java.awt.Color;
import java.util.HashMap;
import java.util.Map;

public enum LabyrinthTile {
	WALL(Color.BLACK, '*'),
	OPEN(Color.WHITE, ' '),
	PLAYER(Color.BLUE, 's'),
	MONSTER(Color.RED, 'm'),
	GOLD(Color.YELLOW, 'g');


	private final Color color;
	private final char symbol;

	private static final Map<Character, LabyrinthTile> SYMBOL_TO_TILE_MAP = new HashMap<>();

	/**
	 * @param color Color of the tile
	 * @param symbol Unique symbol of the tile
	 */
	LabyrinthTile(Color color, char symbol) {
		this.color = color;
		this.symbol = symbol;
	}

	static {
		for(LabyrinthTile labyrinthTile : values()) {
			char tileSymbol = labyrinthTile.getSymbol();
			if(SYMBOL_TO_TILE_MAP.containsKey(tileSymbol)) {
				throw new IllegalStateException("Duplicate symbols found. Both " + SYMBOL_TO_TILE_MAP.get(tileSymbol) + " and " + labyrinthTile + " use the symbol '" + tileSymbol + "'");
			}
			SYMBOL_TO_TILE_MAP.put(tileSymbol, labyrinthTile);
		}
	}

	/**
	 * @return The color to paint the cell with this tile
	 */
	public Color getColor() {
		return color;
	}

	/**
	 * @return The symbol to represent this tile
	 */
	public char getSymbol() {
		return symbol;
	}

	/**
	 * @param symbol to parse into a {@code LabyrinthTile}
	 * @return The {@code LabyrinthTile} which is represented by this symbol, or {@code null} if no tile have
	 * this symbol
	 */
	public static LabyrinthTile fromSymbol(char symbol) {
		return SYMBOL_TO_TILE_MAP.get(symbol);
	}
}
