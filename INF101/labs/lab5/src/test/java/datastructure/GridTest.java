package datastructure;

import static org.junit.jupiter.api.Assertions.assertDoesNotThrow;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import cellular.cellstate.CellState;

import java.util.Random;
import org.junit.jupiter.api.Test;

public class GridTest {

	Random random = new Random(0L);
	Grid<CellState> grid;

	@Test
	public void checkRowAndColumnEqualToConstructionParametersGiven() {
		int expectedRows = 11;
		int expectedColumns = 17;
		setGrid(expectedRows, expectedColumns);

		assertEquals(expectedRows, grid.numRows());
		assertEquals(expectedColumns, grid.numColumns());
	}

	private void setGrid(int rows, int columns) {
		grid = new Grid<>(rows, columns, CellState.DEAD);
	}

	/**
	 * Tests that trying to access outside of the dimensions of the grid throws
	 * an IndexOutOfBoundsException.
	 */
	@Test
	public void outOfBoundsTest() {

		int expectedRows = 11;
		int expectedColumns = 17;

		setGrid(expectedRows, expectedColumns);

		assertDoesNotThrow(() -> grid.checkLocation(new Location(0, 0)),
				"Throw exception when given row is equal to or greater than expectedRows: " + expectedRows);

		assertDoesNotThrow(() -> grid.checkLocation(new Location(expectedRows - 1, expectedColumns - 1)),
				"Throw exception when given row is equal to or greater than expectedRows: " + expectedRows);

		assertThrows(IndexOutOfBoundsException.class, () -> grid.checkLocation(new Location(expectedRows, 0)),
				"Throw exception when given row is equal to or greater than expectedRows: " + expectedRows);

		assertThrows(IndexOutOfBoundsException.class, () -> grid.checkLocation(new Location(0, expectedColumns)),
				"Throw exception when given column is equal to or greater than expectedColumns: " + expectedColumns);

		assertThrows(IndexOutOfBoundsException.class, () -> grid.checkLocation(new Location(-1, 0)), "Throw exception when given row is negative");

		assertThrows(IndexOutOfBoundsException.class, () -> grid.checkLocation(new Location(0, -1)), "Throw exception when given column is negative");
	}

	@Test
	public void checkStateSetIsEqualBeforeSettingAnyOtherCell() {
		setGrid(100, 101);

		for (Location loc : grid.locations()) {
			CellState cs = CellState.random(random);
			grid.set(loc, cs);
			assertEquals(cs, grid.get(loc), "Failed to update cell (row " + loc.row + ", col " + loc.col + ")");
		}
	}

	@Test
	public void checkStateCanBeSetMultipleTimes() {
		setGrid(101, 100);

		for (Location loc : grid.locations()) {
			grid.set(loc, CellState.random(random));
		}

		for (Location loc : grid.locations()) {
			CellState cs = CellState.random(random);
			grid.set(loc, cs);
			assertEquals(cs, grid.get(loc), "Failed to update cell (row " + loc.row + ", col " + loc.col + ")");
		}
	}

	@Test
	public void copyTest() {
		setGrid(100, 105);

		for (Location loc : grid.locations()) {
			grid.set(loc, CellState.random(random));
		}

		IGrid newGrid = grid.copy();
		for (Location loc : grid.locations()) {
			assertEquals(grid.get(loc), newGrid.get(loc), "Cell at (row " + loc.row + ", col " + loc.col + ") is not properly copied over to the new grid");
		}
	}
}
