package inf101.grid;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.Test;

public class MoveTest {

	Location start = new Location(1,5);
	Location end = new Location(2,6);
	Move move = new Move(start, end);
	
	@Test
	public void constructTest() {
		assertEquals(move.start, this.start);
		assertEquals(move.end, this.end);
	}
	
	@Test
	public void validMoveTest() {
		assertTrue(move.isValid());
	}
	
	@Test
	public void invalidMoveTest() {
		Location invalidEnd = new Location(0,2);
		Move invalidMove = new Move(this.start, invalidEnd);
		assertFalse(invalidMove.isValid());
	}
	
	@Test
	public void moveTypeTest() {
		assertEquals(false, move.moveType);
		
		Location newEnd = new Location(3,5);
		Move newMove = new Move(this.start, newEnd);
		
		assertEquals(true, newMove.moveType);
	}
}
