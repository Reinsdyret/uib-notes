package datastructure;

import cellular.cellstate.ICellState;

/**
 * ICellStateGrid is a grid of ICellStates
 * @author Anna Eilertsen - anna.eilertsen@uib.no
 */
public interface IGrid<T> {

	/**
	 * @return The number of columns in the grid.
	 */
	int numColumns();

	/**
	 * @return The number of rows in the grid.
	 */
	int numRows();

	/**
	 * Set the contents of the cell in the given location.
	 * <p>
	 * The Location loc must be within the bounds of the grid, i.e.
	 * The row index must be greater than or equal to 0 and less than numRows()
	 * The column index must be greater than or equal to 0 and less than numColumns()
	 * 
	 * @param x The column of the cell to change the contents of.
	 * @param y The row of the cell to change the contents of.
	 * @param element The contents the cell is to have.
	 */
	void set(Location loc, T element);

	/**
	 * Get the contents of the cell in the given x,y location.
	 * <p>
	 * y must be greater than or equal to 0 and less than getHeight().
	 * x must be greater than or equal to 0 and less than getWidth().
	 * @param x The column of the cell to get the contents of.
	 * @param y The row of the cell to get contents of.
	 */
	T get(Location loc);

	/**
	 * Makes it possible to iterate over all locations of this grid
	 * Iteration happens row-wise i.e. First row 0, then row 1 and so on.
	 */
	Iterable<Location> locations();

	/**
	 * Checks if a given Location is within the bounds of the grid. 
	 * @param loc the location to check
	 * @return true if loc is within bounds of the grid, false otherwise.
	 */
	boolean isOnGrid(Location loc);

	/**
	 * Make a copy
	 * @return A shallow copy of the grid, with the same elements
	 */
	IGrid<T> copy();
}
