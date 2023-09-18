package datastructure;

import java.util.ArrayList;
import java.util.List;

/**
 * A Grid contains a set of cell states
 * @author Martin Vatshelle - martin.vatshelle@uib.no
 */
public class Grid<T> implements IGrid<T> {
	private final List<T> cells;
	private final int columns;
	private final int rows;

	/**
	 * Construct a grid with the given dimensions.
	 * @param rows
	 * @param columns
	 * @param initElement What the cells should initially hold (possibly null)
	 */
	public Grid(int rows, int columns, T initElement) {
		if(rows <= 0 || columns <= 0) {
			throw new IllegalArgumentException();
		}

		this.columns = columns;
		this.rows = rows;
		cells = new ArrayList<T>(columns * rows);
		for(int i = 0; i < columns * rows; ++i) {
			cells.add(initElement);
		}
	}

	@Override
	public int numColumns() {
		return columns;
	}

	@Override
	public int numRows() {
		return rows;
	}

	@Override
	public void set(Location loc, T elem) {
		checkLocation(loc);

		cells.set(locationToIndex(loc), elem);
	}

	/**
	 * This method checks if a given Location is within the bounds of this grid.
	 * If it is not, an IndexOutOfBoundsException is thrown.
	 * @param loc the location to check
	 */
	public void checkLocation(Location loc) {
		if(!isOnGrid(loc)) {
			throw new IndexOutOfBoundsException("Row and column indices must be within bounds");
		}
	}

	@Override
	public boolean isOnGrid(Location loc) {
		if(loc.row < 0 || loc.row >= rows) {
			return false;
		}

		return loc.col >= 0 && loc.col < columns;
	}

	/**
	 * This method computes which index in the list belongs to a given Location
	 */
	private int locationToIndex(Location loc) {
		return loc.row + loc.col * rows;
	}

	@Override
	public T get(Location loc) {
		checkLocation(loc);

		return cells.get(locationToIndex(loc));
	}

	@Override
	public Iterable<Location> locations() {
		return new GridLocationIterator(numRows(), numColumns());
	}

	@Override
	public IGrid<T> copy() {
		Grid<T> newGrid = new Grid<T>(numRows(), numColumns(), null);

		for(Location loc : this.locations()) {
			newGrid.set(loc, this.get(loc));
		}
		return newGrid;
	}
}
