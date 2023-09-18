package inf101v22.grid;

public interface IGrid<E> extends Iterable<CoordinateItem<E>> {

	/** @return number of rows in grid */
	int getRows();

	/** @return number of columns in grid */
	int getCols();

	/**
	 * Sets the value of a postion in the grid. A subsequent call to {@link #get}
	 * with an equal coordinate as argument will return the value which was set. The
	 * method will overwrite any previous value that was stored at the location.
	 *
	 * @param coordinate the location in which to store the value
	 * @param value      the new value
	 * @throws IndexOutOfBoundsException if the coordinate is not within bounds of
	 *                                   the grid
	 */
	void set(Coordinate coordinate, E value);

	/**
	 * Gets the current value at the given coordinate.
	 *
	 * @param coordinate the location to get
	 * @return the value stored at the coordinate
	 * @throws IndexOutOfBoundsException if the coordinate is not within bounds of
	 *                                   the grid
	 */
	E get(Coordinate coordinate);

	/**
	 * Reports whether the coordinate is within bounds for this grid
	 *
	 * @param coordinate coordinate to check
	 * @return true if the coordinate is within bounds, false otherwise
	 */
	boolean coordinateIsOnGrid(Coordinate coordinate);
}
