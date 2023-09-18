package inf101v22.tetris.model;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.awt.Color;

import org.junit.jupiter.api.Test;

/**
 * Testing the Tile.java class
 * @author Lars Moeen Haukland (Yral)
 *
 */
public class TileTest {

	@Test
	void TileConstructorTest(){
		Tile tile = new Tile(Color.black, 'b');

		assertEquals(tile.character, 'b');
		assertEquals(tile.color, Color.black);
	}
}
