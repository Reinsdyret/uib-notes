package inf101.grid;

import java.util.Iterator;

/**
 * This class is able to iterate over all locations in a grid.
 *
 * @author Martin Vatshelle - martin.vatshelle@uib.no
 */
public class GridLocationIterator implements Iterator<Location>, Iterable<Location> {

	private int numRows;
	private int numCols;
	private Location current;

	/**
	 * Constructs a GridLocationIterator
	 *
	 * @param grid - the grid which we want to iterate through
	 */
	public GridLocationIterator(IGrid<?> grid) {
		this(grid.numRows(), grid.numColumns());
	}

	/**
	 * Constructs a GridLocationIterator
	 *
	 * @param numRows - number of rows in the grid
	 * @param numCols - number of columns in the grid
	 */
	public GridLocationIterator(int numRows, int numCols) {
		this.numRows = numRows;
		this.numCols = numCols;
		current = new Location(0, 0);
	}

	@Override
	public boolean hasNext() {
		return current.row < numRows && current.col < numCols;
	}

	@Override
	public Location next() {
		Location elem = current;
		if (current.col < numCols - 1) {
			current = current.getNeighbor(GridDirection.EAST);
		} else {
			current = new Location(current.row + 1, 0);
		}
		return elem;
	}

	@Override
	public Iterator<Location> iterator() {
		return new GridLocationIterator(numRows, numCols);
	}

}
