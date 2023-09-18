 package inf101.sem2.game.games;

import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import inf101.grid.Location;
import inf101.sem2.GUI.DummyGraphics;
import inf101.sem2.game.Game;
import inf101.sem2.game.TestGame;
import inf101.sem2.player.Player;

class ConnectFourTest extends TestGame {

	ConnectFour game;
	
	@BeforeEach
	protected void init() {
		super.init();
		game = new ConnectFour(new DummyGraphics(), player1, player2);
	}
	
	@Test
	void testConstruct() {
		assertEquals(6,game.getGameBoard().numRows());
		assertEquals(7,game.getGameBoard().numColumns());
		assertEquals("Connect Four", game.getName());
	}

	@Override
	public Game<?> getGame() {
		return game;
	}

	@Test
	void testMakeValidMove(){
		game.makeMove(3);
		assertEquals(player1, game.getGameBoard().get(new Location(5, 3)));
		game.nextPlayer();
		game.makeMove(2);
		assertEquals(player2, game.getGameBoard().get(new Location(5, 2)));
		game.nextPlayer();
		game.makeMove(2);
		assertEquals(player1, game.getGameBoard().get(new Location(4, 2)));
		game.nextPlayer();
		game.makeMove(3);
		assertEquals(player2, game.getGameBoard().get(new Location(4, 3)));
	}
	
	@Test
	void testInvalidMove() {
		game.makeMove(3);
		game.nextPlayer();
		game.makeMove(3);
		game.nextPlayer();
		game.makeMove(3);
		game.nextPlayer();
		game.makeMove(3);
		game.nextPlayer();
		game.makeMove(3);
		game.nextPlayer();
		game.makeMove(3);
		game.nextPlayer();
		
		checkInvalidMove(3);
		checkInvalidMove(-1);		
		checkInvalidMove(7);		
		checkInvalidMove(null);
	}
	
	private void checkInvalidMove(Integer col) {
		assertFalse(game.validMove(col));
		assertThrows(IllegalArgumentException.class, ()-> game.makeMove(col));
	}
}
