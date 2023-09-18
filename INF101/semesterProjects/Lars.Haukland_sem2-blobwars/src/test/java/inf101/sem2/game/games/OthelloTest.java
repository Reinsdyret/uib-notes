package inf101.sem2.game.games;

import static org.junit.jupiter.api.Assertions.*;

import java.util.Arrays;
import java.util.List;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import inf101.grid.Location;
import inf101.sem2.GUI.DummyGraphics;
import inf101.sem2.game.Game;
import inf101.sem2.game.TestGame;
import inf101.sem2.player.Player;

class OthelloTest extends TestGame{
	Othello game;

	@BeforeEach
	protected void init() {
		super.init();
		game = new Othello(new DummyGraphics(), player1, player2);
		assertEquals(player1,game.getCurrentPlayer());
	}

	@Override
	public Game<?> getGame() {
		return game;
	}

	@Test
	void canConstruct() {
		assertEquals("Othello", game.getName());
		Othello game2 = new Othello(game.getGraphics(), game.players());
		assertEquals("Othello", game2.getName());
		boolean lik = game.getGameBoard().equals(game2.getGameBoard());
		assertTrue(lik);
	}
	
	@Test
	void startsWith4pieces() {
		assertEquals(player1,game.getGameBoard().get(new Location(3, 3)));
		assertEquals(player2,game.getGameBoard().get(new Location(3, 4)));
		assertEquals(player2,game.getGameBoard().get(new Location(4, 3)));
		assertEquals(player1,game.getGameBoard().get(new Location(4, 4)));
	}
	
	@Test
	void validMoveTest() {
		assertFalse(game.validMove(new Location(3, 2)));
		assertTrue(game.validMove(new Location(3, 5)));
		assertTrue(game.validMove(new Location(5, 3)));
		assertFalse(game.validMove(new Location(0, 0)));
		assertFalse(game.validMove(new Location(-1, -1)));
		}
	
	@Test
	void testMakeMove() {
		Location move = new Location(5,3);
		assertTrue(game.validMove(move));
		game.makeMove(move);
		game.nextPlayer();
		assertEquals(player1, game.getGameBoard().get(move));
		assertEquals(player1, game.getGameBoard().get(new Location(4,3)));
		assertEquals(player2, game.getCurrentPlayer());
	}
	
	@Test
	void testGetPossibleMoves() {
		List<Location> moves = game.getPossibleMoves();
		for(Location loc : game.getGameBoard().locations()) {
			assertEquals(game.validMove(loc), moves.contains(loc));
		}
	}
	
	@Test
	void testCopy() {
		TestGame.testCopy(game);
		assertTrue(Arrays.equals("test".getBytes(),"test".getBytes()));
	}
	
	@Test
	void testGameOver() {
		//D3 -> 3,2
		//C5 -> 2,4
		//D6 -> 3,5
		//E3 -> 4,2
		//B4 -> 1,3
		//C3 -> 2,2
		//D2 -> 3,1
		//C4 -> 2,3
		//F4 -> 5,3
		
		assertEquals(player1,game.getCurrentPlayer());
		game.makeMove(new Location(4, 2));
		game.nextPlayer();
		assertEquals(player2,game.getCurrentPlayer());
		game.makeMove(new Location(5, 4));
		game.nextPlayer();
		assertEquals(player1,game.getCurrentPlayer());
		game.makeMove(new Location(4, 5));
		game.nextPlayer();
		assertEquals(player2,game.getCurrentPlayer());
		game.makeMove(new Location(3, 2));
		game.nextPlayer();
		assertEquals(player1,game.getCurrentPlayer());
		game.makeMove(new Location(6, 3));
		game.nextPlayer();
		assertEquals(player2,game.getCurrentPlayer());
		game.makeMove(new Location(5, 2));
		game.nextPlayer();
		assertEquals(player1,game.getCurrentPlayer());
		game.makeMove(new Location(4, 1));
		game.nextPlayer();
		assertEquals(player2,game.getCurrentPlayer());
		game.makeMove(new Location(5, 3));
		assertFalse(game.gameOver());
		game.nextPlayer();
		assertEquals(player1,game.getCurrentPlayer());
		game.makeMove(new Location(2, 3));
		game.nextPlayer();
		assertTrue(game.gameOver());
		assertEquals(13, game.score(player1));
		assertEquals(-13, game.score(player2));
	}
	
	@Test
	void testRestart() {
		game.makeMove(new Location(4,2));
		assertEquals(player1,getPlayer(4,2));
	}
	
	Player getPlayer(int row, int col) {
		return game.getGameBoard().get(new Location(4,2));
	}
	
	@Test
	void testFirstMoveNumPieces() {
		Location loc = player1.getMove(game);
		game.makeMove(loc);
		game.nextPlayer();
		assertEquals(4,game.getGameBoard().countPieces(player1));
		assertEquals(1,game.getGameBoard().countPieces(player2));
		loc = player2.getMove(game);
		game.makeMove(loc);
		assertEquals(3,game.getGameBoard().countPieces(player1));
		assertEquals(3,game.getGameBoard().countPieces(player2));
	}
}
