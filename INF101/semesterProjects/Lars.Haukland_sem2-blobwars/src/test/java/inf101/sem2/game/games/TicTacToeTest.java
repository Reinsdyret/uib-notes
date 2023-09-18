package inf101.sem2.game.games;

import static org.junit.jupiter.api.Assertions.*;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import inf101.grid.Location;
import inf101.sem2.GUI.DummyGraphics;
import inf101.sem2.game.Game;
import inf101.sem2.game.TestGame;
import inf101.sem2.player.Player;

class TicTacToeTest extends TestGame{

	TicTacToe game;
	
	@BeforeEach
	protected void init() {
		super.init();
		game = new TicTacToe(new DummyGraphics(), player1, player2);
	}

	public Game<?> getGame(){
		return game;
	}
	
	@Test
	void testConstruct() {
		assertEquals(3, game.getGameBoard().numRows());
		assertEquals(3, game.getGameBoard().numColumns());
		assertEquals("Tic Tac Toe", game.getName());
	}
	
	@Test
	void testValidMove() {
		checkValidMove(new Location(0, 0));
		checkValidMove(new Location(1, 1));
		checkValidMove(new Location(1, 2));
		checkValidMove(new Location(2, 1));
		checkValidMove(new Location(0, 1));
	}

	private void checkValidMove(Location loc) {
		Player p = game.getCurrentPlayer();
		assertTrue(game.validMove(loc));
		game.makeMove(loc);
		assertEquals(p, game.getGameBoard().get(loc));
	}
	
	@Test
	void testInvalidMove() {
		checkValidMove(new Location(0, 0));
		checkInvalidMove(new Location(0, 0));
		checkValidMove(new Location(1, 1));
		checkInvalidMove(new Location(1, 1));
		checkNotAMove(new Location(3, 1));
		checkNotAMove(new Location(2, -1));		
		checkNotAMove(null);
	}
	private void checkInvalidMove(Location loc) {
		assertFalse(game.validMove(loc));
		Player p = game.getGameBoard().get(loc);
		assertThrows(IllegalArgumentException.class, ()-> game.makeMove(loc));
		assertEquals(p, game.getGameBoard().get(loc));
	}

	private void checkNotAMove(Location loc) {
		assertFalse(game.validMove(loc));
		assertThrows(IllegalArgumentException.class, ()-> game.makeMove(loc));
	}

	@Test
	void testGameOverTie() {
		game.makeMove(new Location(1, 1));
		game.nextPlayer();
		assertFalse(game.gameOver());
		game.makeMove(new Location(0, 2));
		game.nextPlayer();
		assertFalse(game.gameOver());
		game.makeMove(new Location(0, 1));
		game.nextPlayer();
		assertFalse(game.gameOver());
		game.makeMove(new Location(2, 1));
		game.nextPlayer();
		assertFalse(game.gameOver());
		game.makeMove(new Location(1, 0));
		game.nextPlayer();
		assertFalse(game.gameOver());
		game.makeMove(new Location(1, 2));
		game.nextPlayer();
		assertFalse(game.gameOver());
		game.makeMove(new Location(2, 2));
		game.nextPlayer();
		assertFalse(game.gameOver());
		game.makeMove(new Location(0, 0));
		game.nextPlayer();
		assertFalse(game.gameOver());
		game.makeMove(new Location(2, 0));
		game.nextPlayer();
		assertTrue(game.gameOver());
	}
	
	@Test
	void testGameOverWinner() {
		game.makeMove(new Location(1, 1));//X
		game.nextPlayer();
		assertFalse(game.gameOver());
		game.makeMove(new Location(0, 2));//O
		game.nextPlayer();
		assertFalse(game.gameOver());
		game.makeMove(new Location(0, 1));//X
		game.nextPlayer();
		assertFalse(game.gameOver());
		game.makeMove(new Location(2, 1));//O
		game.nextPlayer();
		assertFalse(game.gameOver());
		game.makeMove(new Location(1, 0));//X
		game.nextPlayer();
		assertFalse(game.gameOver());
		game.makeMove(new Location(2, 2));//O
		game.nextPlayer();
		assertFalse(game.gameOver());
		game.makeMove(new Location(1, 2));//X
		assertTrue(game.gameOver());
		assertTrue(game.isWinner(player1));
		assertFalse(game.isWinner(player2));
	}
	
	@Test
	void testPossibleMoves() {
		ArrayList<Location> moves = new ArrayList<Location>();
		for(Location loc : game.getGameBoard().locations()) {
			moves.add(loc);
		}
		testCurrentMoves(moves);
		makeAndTrackMove(moves,new Location(0, 0));
		makeAndTrackMove(moves,new Location(2, 0));
		makeAndTrackMove(moves,new Location(1, 1));
		makeAndTrackMove(moves,new Location(2, 2));
	}

	private void makeAndTrackMove(ArrayList<Location> moves,Location loc) {
		game.makeMove(loc);
		moves.remove(loc);
		testCurrentMoves(moves);
		game.nextPlayer();
	}

	private void testCurrentMoves(List<Location> expected) {
		List<Location> actual = game.getPossibleMoves();
		TestGame.testSameContent(expected,actual);
	}
}
