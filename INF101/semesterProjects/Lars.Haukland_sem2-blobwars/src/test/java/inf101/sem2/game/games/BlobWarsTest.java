package inf101.sem2.game.games;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.Arrays;
import java.util.List;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import inf101.grid.Location;
import inf101.grid.Move;
import inf101.sem2.GUI.DummyGraphics;
import inf101.sem2.game.Game;
import inf101.sem2.game.TestGame;

public class BlobWarsTest extends TestGame{
	BlobWars game;

	@Override
	@BeforeEach
	protected void init() {
		super.init();
		game = new BlobWars(new DummyGraphics(), player1, player2);
		assertEquals(player1,game.getCurrentPlayer());
	}

	@Test
	void testCopy() {
		TestGame.testCopy(game);
		assertTrue(Arrays.equals("test".getBytes(),"test".getBytes()));
	}

	@Test
	void testGetPossibleMoves() {
		// Test moves for player X
		// List of expected moves
		Location start = new Location(0,0);
		Move[] expected = {new Move(start, new Location(0,1)),new Move(start, new Location(0,2)),new Move(start, new Location(1,0)),new Move(start, new Location(1,1)),new Move(start, new Location(1,2)),new Move(start, new Location(2,0)),new Move(start, new Location(2,1)),new Move(start, new Location(2,2))};
		List<Move> expectedList = Arrays.asList(expected);

		List<Move> real = this.game.getPossibleMoves();

		assertEquals(expectedList.size(), real.size());

		for(int i = 0; i < real.size(); i++) {
			assertTrue(expectedList.get(i).equals(real.get(i)));
		}
	}

	/**
	 * Tests the making of a move
	 * First the move of only one cell (start blob stays)
	 * Then test move of two cells (start blob is removed)
	 */
	@Test
	void testMakeMove() {
		Location start = new Location(0,0);
		Location end = new Location(1,1);
		// Test original pos
		assertTrue(this.game.hasCurrentPlayer(start));
		// Double check that positioned to be moved to does not have player
		assertFalse(this.game.hasCurrentPlayer(end));

		// Move the blob
		this.game.makeMove(new Move(start,end));

		// Check that new blob has been created
		assertTrue(this.game.hasCurrentPlayer(end));
		// Check that previous blob is still there
		assertTrue(this.game.hasCurrentPlayer(start));

		start = end;
		end = new Location(3,3);

		// Move
		this.game.makeMove(new Move(start, end));

		// Check that new blob is there
		assertTrue(this.game.hasCurrentPlayer(end));
		// Check that previous is gone
		assertFalse(this.game.hasCurrentPlayer(start));
	}

	/**
	 * First test that gameOver returns True when board is full
	 * Then test that gameOver returns true when board only has one player
	 */
	@Test
	void testGameOver() {
		// Reset board because of previous testing
		this.game.restart();

		// Fill the board
		// First make line of blobs one the left so that filling is easier
		for(int row = 0; row<7; row++) {
			this.game.makeMove(new Move(new Location(row, 0), new Location(row + 1, 0)));
		}
		for(int row = 0; row<8; row++) {
			for(int col = 0; col<7; col++) { // Col cannot be 7 as then I would attempt to make a move outside the board
				// Skip the last cell as the enemy is there
				if(row == 7 && col == 6) continue;
				this.game.makeMove(new Move(new Location(row, col), new Location(row, col+1)));
			}
		}
		// Check
		assertTrue(this.game.gameOver());

		// Reset the board
		this.game.restart();

		// Eliminate player O
		this.game.makeMove(new Move(new Location(0,0), new Location(2,2)));
		this.game.makeMove(new Move(new Location(2,2), new Location(4,4)));
		this.game.makeMove(new Move(new Location(4,4), new Location(6,6)));

		// The gameover only checks if the player with no blobs is currentPlayer
		this.game.nextPlayer();

		// Check
		assertTrue(this.game.gameOver());
	}

	@Override
	public Game<?> getGame() {
		// TODO Auto-generated method stub
		return game;
	}
}
