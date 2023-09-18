package cellular;

import static org.junit.jupiter.api.Assertions.assertNotEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;

import cellular.cellstate.CellState;
import java.awt.Color;
import org.junit.jupiter.api.Test;

class CellStateTest {

	@Test
	void testGetColor() {
		Color on = CellState.ALIVE.getColor();
		Color off = CellState.DEAD.getColor();
		assertNotNull(on, "Cell Color can not be null");
		assertNotNull(off, "Cell Color can not be null");
		assertNotEquals(off, on, "CellStates must have different colors");
	}

}
