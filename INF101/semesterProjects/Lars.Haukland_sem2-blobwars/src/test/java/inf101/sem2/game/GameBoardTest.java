package inf101.sem2.game;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import inf101.GetStarted;
import inf101.grid.Location;
import inf101.sem2.player.Player;
import inf101.sem2.player.ai.DumbPlayer;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

class GameBoardTest {
	@BeforeEach
	void testReadConditions() {
		assertTrue(GetStarted.hasRead);
	}

	GameBoard board;
	Player p1;
	Player p2;

	@BeforeEach
	void setUp() throws Exception {
		board = new GameBoard(7, 4);
		p1 = new DumbPlayer('X');
		p2 = new DumbPlayer('O');
	}

	@Test
	void testGameBoard() {
		assertEquals(7, board.numRows());
		assertEquals(4, board.numColumns());
	}

	@Test
	void testSetLocationPlayer() {
		Location loc = new Location(2, 3);
		board.set(loc, p1);
		assertEquals(p1, board.get(loc));
		board.set(loc, p2);
		assertEquals(p1, board.get(loc));
	}

	@Test
	void testCanPlace() {
		Location loc = new Location(2, 3);
		assertTrue(board.canPlace(loc));
		board.set(loc, p1);
		assertFalse(board.canPlace(loc));
	}

	@Test
	void testCountNumInRow() {
		assertEquals(0, board.countNumInRow(p1));
		board.set(new Location(1, 1), p1);
		assertEquals(1, board.countNumInRow(p1));
		board.set(new Location(2, 2), p1);
		assertEquals(2, board.countNumInRow(p1));
		board.set(new Location(3, 1), p1);
		assertEquals(2, board.countNumInRow(p1));
		board.set(new Location(2, 1), p2);
		assertEquals(2, board.countNumInRow(p1));
		assertEquals(1, board.countNumInRow(p2));
	}

	@Test
	void testIsFull() {
		assertFalse(board.isFull());
		board.set(new Location(2, 2), p1);
		assertFalse(board.isFull());
		board.fill(p1);
		assertTrue(board.isFull());
	}

	@Test
	void testCopy() {
		GameBoard newBoard = board.copy();
		for (Location loc : board.locations()) {
			assertEquals(board.get(loc), newBoard.get(loc));
		}
		assertEquals(board.numRows(), newBoard.numRows());
		assertEquals(board.numColumns(), newBoard.numColumns());
	}

	@Test
	void testToString() {
		int numP1 = 0;
		for (Location loc : board.locations()) {
			int r = (int) (3 * Math.random());

			switch (r) {
				case 1 -> {
					board.set(loc, p1);
					numP1++;
				}
				case 2 -> board.set(loc, p2);
			}
		}
		String s = board.toString();
		int count = 0;
		for (char c : s.toCharArray()) {
			if (c == p1.getSymbol()) {
				count++;
			}
		}
		assertEquals(numP1, count);
	}
}
