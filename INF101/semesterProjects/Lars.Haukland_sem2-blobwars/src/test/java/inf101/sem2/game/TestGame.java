package inf101.sem2.game;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.junit.jupiter.api.Assertions.fail;

import java.util.Collection;
import java.util.Collections;
import java.util.List;

import org.junit.jupiter.api.BeforeEach;

import inf101.GetStarted;
import inf101.grid.Location;
import inf101.sem2.game.games.ConnectFour;
import inf101.sem2.game.games.TicTacToe;
import inf101.sem2.player.Player;
import inf101.sem2.player.ai.DumbPlayer;
import inf101.sem2.terminal.TerminalGraphics;
import org.junit.jupiter.api.Test;


public abstract class TestGame {

	protected Player player1;
	protected Player player2;
	
	@BeforeEach
	protected void init() {
		testReadConditions();
		player1 = new DumbPlayer('X');
		player2 = new DumbPlayer('O');
	}
	
	void testReadConditions() {
		assertTrue(GetStarted.hasRead);
	}

	public abstract Game<?> getGame();
	
	@Test
	void testDumbPlayerCanPlay() {
		Game<?> game = new TicTacToe(new TerminalGraphics(), player1, player2);
		game.run();
		assertTrue(game.gameOver());

		game = new ConnectFour(new TerminalGraphics(), player1, player2);
		game.run();
		assertTrue(game.gameOver());
	}

	@Test
	public void testPlayers() {
		Game<?> game = getGame();
		assertEquals(player1, game.getCurrentPlayer());
		game.nextPlayer();
		assertEquals(player2, game.getCurrentPlayer());
		game.nextPlayer();
		assertEquals(player1, game.getCurrentPlayer());
	}
	
	public static <T> void testSameContent(Collection<T> expected, Collection<T> actual) {
		if(expected==null || actual==null)
			if(expected!=actual)
				fail();
			else
				return;
		if(expected.size()!=actual.size())
			fail("Collections does not have the same number of elements");
		for(T t : expected) {
			int expectedCount = Collections.frequency(expected, t);
			int actualCount = Collections.frequency(actual, t);
			assertEquals(expectedCount,actualCount,"Element "+t+" appears "+actualCount+" times in "+actual+", but was expected "+expectedCount+" times.");
		}
	}
	
	protected static void testCopy(Game<?> game) {
		Game<?> newGame = game.copy();
		for(Location loc:game.getGameBoard().locations()) {
			assertEquals(game.getGameBoard().get(loc), newGame.getGameBoard().get(loc));
		}
		assertEquals(game.getCurrentPlayer(),newGame.getCurrentPlayer());
	}
}
