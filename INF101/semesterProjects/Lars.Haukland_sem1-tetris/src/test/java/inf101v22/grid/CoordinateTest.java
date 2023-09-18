package inf101v22.grid;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.Objects;

import org.junit.jupiter.api.Test;

/**
 * Testing the class Coordinate.java
 * @author Lars Moeen Haukland (Yral)
 */
public class CoordinateTest {
	// TODO: Uncomment these tests and make them pass by implementing Coordinate

	@Test
	void coordinateSanityTest() {
		Coordinate c = new Coordinate(4, 3);
		assertEquals(4, c.row);
		assertEquals(3, c.col);
	}
	@Test
	void coordinateToStringTest() {
		Coordinate c1 = new Coordinate(4, 3);
		assertEquals("{ row='4', col='3' }", c1.toString());

		Coordinate c2 = new Coordinate(6, 1);
		assertEquals("{ row='6', col='1' }", c2.toString());
	}

	@Test
	void coordinateEqualityTest() {
		Coordinate a = new Coordinate(2, 3);
		Coordinate b = new Coordinate(2, 3);

		assertFalse(a == b);
		assertTrue(a.equals(b));
		assertTrue(b.equals(a));
		assertTrue(Objects.equals(a, b));
	}

	@Test
	void coordinateInequalityTest() {
		Coordinate a = new Coordinate(2, 3);
		Coordinate b = new Coordinate(3, 2);

		assertFalse(a == b);
		assertFalse(a.equals(b));
		assertFalse(b.equals(a));
		assertFalse(Objects.equals(a, b));
	}

	@Test
	void coordinateHashcodeTest() {
		Coordinate a = new Coordinate(2, 3);
		Coordinate b = new Coordinate(2, 3);
		assertTrue(a.hashCode() == b.hashCode());

		Coordinate c = new Coordinate(100, 100);
		Coordinate d = new Coordinate(100, 100);
		assertTrue(c.hashCode() == d.hashCode());
	}
}
