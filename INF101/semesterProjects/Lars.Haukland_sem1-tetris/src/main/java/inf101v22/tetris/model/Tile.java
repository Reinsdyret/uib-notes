package inf101v22.tetris.model;

import java.awt.Color;

public class Tile {
	public final Color color;
	public final char character;

	/**
	 * Create a new Tile with given color and char for debugging
	 * @param color
	 * @param character
	 */
	public Tile(Color color, char character) {
		this.color = color;
		this.character = character;
	}
}
