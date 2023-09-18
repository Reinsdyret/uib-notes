package inf101.sem2.game;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.junit.jupiter.api.Assertions.fail;

import org.junit.jupiter.api.BeforeEach;

import inf101.GetStarted;
import inf101.sem2.player.AbstractPlayer;
import inf101.sem2.player.Player;
import inf101.sem2.player.ai.DumbPlayer;

import org.junit.jupiter.api.Test;

class PlayerTest {

	@BeforeEach
	void testReadConditions() {
		assertTrue(GetStarted.hasRead);
	}

	public static void testConstructor(String symbol, String name, Player p) {
		assertEquals(name, p.toString());
		assertEquals(symbol, p.getSymbol());
	}

	@Test
	void testisValidName() {
		assertTrue(AbstractPlayer.isValidName("Martin"));
		assertFalse(AbstractPlayer.isValidName(" "));
		assertFalse(AbstractPlayer.isValidName("\n"));
		assertFalse(AbstractPlayer.isValidName("\t"));
	}

	@Test
	void testValidateName() {
		testValidName("Martin");
		try {
			AbstractPlayer.validateName("");
		} catch (IllegalArgumentException e) {
			return;
		} catch (Exception e) {
			fail("Should throw an IllegalArgumentException");
		}
	}

	private void testValidName(String name) {
		try {
			assertEquals(name, AbstractPlayer.validateName(name));
		} catch (Exception e) {
			fail(name + " is not a vailld name.");
		}
	}

	@Test
	void testConstruct() {
		testConstruct(new DumbPlayer('X'));
	}

	private void testConstruct(Player player) {
		testValidName(player.toString());

	}

}
