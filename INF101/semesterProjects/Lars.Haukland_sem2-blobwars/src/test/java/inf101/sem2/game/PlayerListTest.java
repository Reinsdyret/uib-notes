package inf101.sem2.game;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.BeforeEach;

import inf101.GetStarted;
import inf101.sem2.player.Player;
import inf101.sem2.player.ai.DumbPlayer;

import org.junit.jupiter.api.Test;

class PlayerListTest {

	@BeforeEach
	void testReadConditions() {
		assertTrue(GetStarted.hasRead);
	}

	@Test
	void testAlternatingPlayers() {
		PlayerList players = new PlayerList();
		Player p1 = new DumbPlayer('X');
		Player p2 = new DumbPlayer('O');
		players.add(p1);
		players.add(p2);
		assertEquals(p1, players.getCurrentPlayer());
		assertEquals(p2, players.nextPlayer());
		assertEquals(p2, players.getCurrentPlayer());
		assertEquals(p1, players.nextPlayer());
		assertEquals(p1, players.getCurrentPlayer());
		assertEquals(p2, players.nextPlayer());
		assertEquals(p2, players.getCurrentPlayer());
	}

	@Test
	void testAddPlayers() {
		PlayerList players = new PlayerList();
		Player p1 = new DumbPlayer('X');
		Player p2 = new DumbPlayer('O');
		players.add(p1);
		players.add(p2);

		assertThrows(IllegalArgumentException.class, () -> players.add(p1));

		Player p3 = new DumbPlayer('O');
		assertThrows(IllegalArgumentException.class, () -> players.add(p3));
	}

	@Test
	void testRemovePlayers() {
		PlayerList players = new PlayerList();
		Player p1 = new DumbPlayer('X');
		Player p2 = new DumbPlayer('O');
		Player p3 = new DumbPlayer('T');
		players.add(p1);

		assertThrows(IllegalArgumentException.class, () -> players.remove(p2));
		players.add(p2);
		players.add(p3);

		players.setCurrentPlayer(p1);
		players.remove(p3);
		assertEquals(p1, players.getCurrentPlayer());
		players.add(p3);

		players.setCurrentPlayer(p2);
		players.remove(p1);
		assertEquals(p2, players.getCurrentPlayer());
		players.add(p1);

		players.setCurrentPlayer(p1);
		players.remove(p1);
		assertEquals(p2, players.getCurrentPlayer());
		players.remove(p2);
		players.remove(p3);
	}

	@Test
	void testSetCurrentPlayer() {
		PlayerList players = new PlayerList();
		Player p1 = new DumbPlayer('X');
		Player p2 = new DumbPlayer('O');
		Player p3 = new DumbPlayer('T');
		players.add(p1);
		assertThrows(IllegalArgumentException.class, () -> players.setCurrentPlayer(p2));

		players.add(p2);
		players.add(p3);

		players.setCurrentPlayer(p2);
		assertEquals(p2, players.getCurrentPlayer());
	}
}
