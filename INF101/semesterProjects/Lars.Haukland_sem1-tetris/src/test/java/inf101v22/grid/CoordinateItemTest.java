package inf101v22.grid;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.Objects;

import org.junit.jupiter.api.Test;

/**
 * Testing the class CoordinateItem.java
 * @author Lars Moeen Haukland (Yral)
 */
public class CoordinateItemTest {
	// TODO: Uncomment these tests and make them pass by implementing CoordinateItem and Coordinate

	@Test
	void coordinateItemSanityTest() {
		String item = "Test";
		Coordinate coordinate = new Coordinate(4, 2);
		CoordinateItem<String> coordinateItem = new CoordinateItem<>(coordinate, item);

		assertEquals(coordinate, coordinateItem.coordinate);
		assertEquals(item, coordinateItem.item);
	}

	@Test
	void coordinateItemToStringTest() {
		String item = "Test";
		Coordinate coordinate = new Coordinate(4, 2);
		CoordinateItem<String> coordinateItem = new CoordinateItem<>(coordinate, item);

		assertEquals("{ coordinate='{ row='4', col='2' }', item='Test' }", coordinateItem.toString());
	}

	@Test
	void coordinateItemEqualityAndHashCodeTest() {
		String item = "Test";
		Coordinate coordinate = new Coordinate(4, 2);
		CoordinateItem<String> coordinateItem = new CoordinateItem<>(coordinate, item);

		String item2 = "Test";
		Coordinate coordinate2 = new Coordinate(4, 2);
		CoordinateItem<String> coordinateItem2 = new CoordinateItem<>(coordinate2, item2);

		assertTrue(coordinateItem2.equals(coordinateItem));
		assertTrue(coordinateItem.equals(coordinateItem2));
		assertTrue(Objects.equals(coordinateItem, coordinateItem2));
		assertTrue(coordinateItem.hashCode() == coordinateItem2.hashCode());
	}

	@Test
	void coordinateItemInequalityTest() {
		String item = "Test";
		Coordinate coordinate = new Coordinate(4, 2);
		CoordinateItem<String> coordinateItem = new CoordinateItem<>(coordinate, item);

		String item2 = "Test2";
		Coordinate coordinate2 = new Coordinate(2, 4);

		CoordinateItem<String> coordinateItem2 = new CoordinateItem<>(coordinate2, item);
		CoordinateItem<String> coordinateItem3 = new CoordinateItem<>(coordinate, item2);

		assertFalse(coordinateItem2.equals(coordinateItem));
		assertFalse(coordinateItem.equals(coordinateItem2));
		assertFalse(coordinateItem.equals(coordinateItem3));
		assertFalse(coordinateItem2.equals(coordinateItem3));
		assertFalse(Objects.equals(coordinateItem, coordinateItem2));
		assertFalse(Objects.equals(coordinateItem, coordinateItem3));
		assertFalse(Objects.equals(coordinateItem2, coordinateItem3));
	}
}
