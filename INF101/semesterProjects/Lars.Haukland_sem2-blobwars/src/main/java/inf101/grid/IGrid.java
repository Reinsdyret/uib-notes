package inf101.grid;

import java.util.List;
import java.util.function.Function;

public interface IGrid<T> extends Iterable<T> {

	/**
	 * Get the contents of the cell in the given x,y location.
	 * <p>
	 * y must be greater than or equal to 0 and less than getHeight(). x must be
	 * greater than or equal to 0 and less than getWidth().
	 *
	 * @param loc The (x,y) position of the grid cell to get the contents of.
	 * @throws IndexOutOfBoundsException if !isOnGrid(loc)
	 */
	T get(Location loc);

	/**
	 * Get the contents of the cell in the given x,y location.
	 * <p>
	 * y must be greater than or equal to 0 and less than getHeight(). x must be
	 * greater than or equal to 0 and less than getWidth().
	 *
	 * @param pos           The (x,y) position of the grid cell to get the contents
	 *                      of.
	 * @param defaultResult A default value to be substituted if the (x,y) is out of
	 *                      bounds or contents == null.
	 */
	default T getOrDefault(Location pos, T defaultResult) {
		return (isOnGrid(pos) && get(pos) != null) ? get(pos) : defaultResult;
	}

	/**
	 * Set the contents of the cell in the given x,y location.
	 * <p>
	 * y must be greater than or equal to 0 and less than getHeight(). x must be
	 * greater than or equal to 0 and less than getWidth().
	 *
	 * @param pos     The (x,y) position of the grid cell to get the contents of.
	 * @param element The contents the cell is to have.
	 * @throws IndexOutOfBoundsException if !isOnGrid(pos)
	 */
	void set(Location pos, T element);

	/**
	 * @return The height of the grid.
	 */
	int numRows();

	/**
	 * @return The width of the grid.
	 */
	int numColumns();

	/**
	 * Initialise the contents of all cells using an initialiser function.
	 * <p>
	 * The function will be called with the (x,y) position of an element, and is
	 * expected to return the element to place at that position. For example:
	 *
	 * <pre>
	 * // fill all cells with the position as a string (e.g., "(2,2)")
	 * grid.setAll((x, y) -> String.format("(%d,%d)", x, y));
	 * </pre>
	 *
	 * @param initialiser The initialiser function
	 */
	void fill(Function<Location, T> initialiser);

	/**
	 * Set the contents of all cells to <code>element</code>
	 * <p>
	 * For example:
	 *
	 * <pre>
	 * // clear the grid
	 * grid.setAll(null);
	 * </pre>
	 *
	 * @param initialiser
	 */
	void fill(T element);

	/**
	 * Make a copy
	 *
	 * @return A fresh copy of the grid, with the same elements
	 */
	IGrid<T> copy();

	/**
	 * Check if coordinates are valid.
	 * <p>
	 * Valid coordinates are 0 <= pos.getX() < getWidth(), 0 <= pos.getY() <
	 * getHeight().
	 *
	 * @param pos A position
	 * @return true if the position is within the grid
	 */
	boolean isOnGrid(Location pos);

	/**
	 * Iterate over all grid locations
	 * <p>
	 * See also {@link #iterator()} – using the grid directly in a for-loop will
	 * iterate over the elements.
	 * <p>
	 * All locations obtained through the iterator are guaranteed to be valid
	 * according to {@link #isValid(ILocation)}.
	 *
	 * @return An iterable for iterating over all the locations in the grid
	 */
	Iterable<Location> locations();

	boolean canGo(Location from, GridDirection dir);
	
}
