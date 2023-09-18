package inf101v22.grid;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.junit.jupiter.api.Assertions.fail;

import java.util.ArrayList;
import java.util.List;

import org.junit.jupiter.api.Test;

/**
 * Testing the class Grid.java 
 * @author Lars Moeen Haukland (Yral)
 */
public class GridTest {
	// TODO: Uncomment these tests and make them pass by creating a class Grid which implements IGrid

	@Test
	void gridTestGetRowsAndCols() {
		IGrid<Integer> grid = new Grid<>(3, 2);
		assertEquals(3, grid.getRows());
		assertEquals(2, grid.getCols());
	}

	@Test
	void gridSanityTest() {
		String defaultValue = "x";
		IGrid<String> grid = new Grid<>(3, 2, defaultValue);

		assertEquals(3, grid.getRows());
		assertEquals(2, grid.getCols());

		assertEquals("x", grid.get(new Coordinate(0, 0)));
		assertEquals("x", grid.get(new Coordinate(2, 1)));

		grid.set(new Coordinate(1, 1), "y");

		assertEquals("y", grid.get(new Coordinate(1, 1)));
		assertEquals("x", grid.get(new Coordinate(0, 1)));
		assertEquals("x", grid.get(new Coordinate(1, 0)));
		assertEquals("x", grid.get(new Coordinate(2, 1)));
	}

	@Test
	void gridCanHoldNull() {
		String defaultValue = "x";
		IGrid<String> grid = new Grid<>(3, 2, defaultValue);

		assertEquals("x", grid.get(new Coordinate(0, 0)));
		assertEquals("x", grid.get(new Coordinate(2, 1)));

		grid.set(new Coordinate(1, 1), null);

		assertEquals(null, grid.get(new Coordinate(1, 1)));
		assertEquals("x", grid.get(new Coordinate(0, 1)));
		assertEquals("x", grid.get(new Coordinate(1, 0)));
		assertEquals("x", grid.get(new Coordinate(2, 1)));
	}

	@Test
	void gridNullsInDefaultConstructor() {
		IGrid<String> grid = new Grid<>(3, 2);

		assertEquals(null, grid.get(new Coordinate(0, 0)));
		assertEquals(null, grid.get(new Coordinate(2, 1)));

		grid.set(new Coordinate(1, 1), "y");

		assertEquals("y", grid.get(new Coordinate(1, 1)));
		assertEquals(null, grid.get(new Coordinate(0, 1)));
		assertEquals(null, grid.get(new Coordinate(1, 0)));
		assertEquals(null, grid.get(new Coordinate(2, 1)));
	}

	@Test
	void coordinateIsOnGridTest() {
		IGrid<Double> grid = new Grid<>(3, 2, 0.9);

		assertTrue(grid.coordinateIsOnGrid(new Coordinate(2, 1)));
		assertFalse(grid.coordinateIsOnGrid(new Coordinate(3, 1)));
		assertFalse(grid.coordinateIsOnGrid(new Coordinate(2, 2)));

		assertTrue(grid.coordinateIsOnGrid(new Coordinate(0, 0)));
		assertFalse(grid.coordinateIsOnGrid(new Coordinate(-1, 0)));
		assertFalse(grid.coordinateIsOnGrid(new Coordinate(0, -1)));
	}

	@Test
	void throwsExceptionWheenCoordinateOffGrid() {
		IGrid<String> grid = new Grid<>(3, 2, "x");

		try {
			grid.get(new Coordinate(3, 1));
			fail();
		} catch (IndexOutOfBoundsException e) {
			// Test passed
		}
	}

	@Test
	void testIterator() {
		IGrid<String> grid = new Grid<>(3, 2, "x");
		grid.set(new Coordinate(0, 0), "a");
		grid.set(new Coordinate(1, 1), "b");
		grid.set(new Coordinate(2, 1), "c");

		List<CoordinateItem<String>> items = new ArrayList<>();
		for (CoordinateItem<String> coordinateItem : grid) {
			items.add(coordinateItem);
		}

		assertEquals(3 * 2, items.size());
		assertTrue(items.contains(new CoordinateItem<>(new Coordinate(0, 0), "a")));
		assertTrue(items.contains(new CoordinateItem<>(new Coordinate(1, 1), "b")));
		assertTrue(items.contains(new CoordinateItem<>(new Coordinate(2, 1), "c")));
		assertTrue(items.contains(new CoordinateItem<>(new Coordinate(0, 1), "x")));
	}
}
