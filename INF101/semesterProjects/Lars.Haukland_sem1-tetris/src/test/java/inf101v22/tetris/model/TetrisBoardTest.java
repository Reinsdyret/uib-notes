package inf101v22.tetris.model;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.awt.Color;

import org.junit.jupiter.api.Test;

import inf101v22.grid.Coordinate;

/**
 * Testing for TetrisBoard.java
 * @author Lars Moeen Haukland (Yral)
 *
 */
public class TetrisBoardTest {

	@Test
	void charArray2dToStringTest() {
		TetrisBoard board = new TetrisBoard(5,5);
		board.set(new Coordinate(0,0), new Tile(Color.yellow, 'y'));
		board.set(new Coordinate(4,0), new Tile(Color.red, 'r'));
		board.set(new Coordinate(0,4), new Tile(Color.blue, 'b'));
		board.set(new Coordinate(4,4), new Tile(Color.green, 'g'));
		String goalString = "y---b\n-----\n-----\n-----\nr---g\n";

		String string2d = board.charArray2dToString(board.toCharArray2d());
		assertEquals(goalString, string2d);
	}

	@Test
	void rowFilledWithNullsTest() {
		TetrisBoard board = new TetrisBoard(3,3);
		TetrisBoard board1 = new TetrisBoard(3,3);
		board.set(new Coordinate(0,0), new Tile(Color.yellow, 'y'));

		assertFalse(board.rowFilledWithNulls(0));

		assertTrue(board1.rowFilledWithNulls(0));
	}

	@Test
	void fillRowWithNullsTest() {
		TetrisBoard board = new TetrisBoard(3,3);
		board.set(new Coordinate(0,0), new Tile(Color.yellow, 'y'));
		board.fillRowWithNulls(0);

		assertTrue(board.rowFilledWithNulls(0));
	}
}
