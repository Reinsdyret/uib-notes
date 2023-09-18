package cellular;

import cellular.cellstate.ICellState;
import datastructure.Location;

/**
 * An CellAutomaton represents a Cellular Automaton. The
 * automaton contains a cell generation organized in rows
 * and columns.
 * @author Anna Eilertsen - anna.eilertsen@uib.no
 */
public interface ICellAutomaton {

	/**
	 * Get the state of the cell in the provided row and column
	 * @param row The row of the cell, 0-indexed
	 * @param column The column of the cell, 0-indexed
	 * @return The state of the cell in the given row and column.
	 */
	ICellState getCellState(Location loc);

	/**
	 * Sets the start-state for each cell
	 */
	void initializeCells();

	/**
	 * Updates the state of the cell according to the rules of the automaton
	 */
	void step();

	/**
	 * @return The number of rows in this automaton
	 */
	int numberOfRows();

	/**
	 * @return The number of columns in this automaton
	 */
	int numberOfColumns();

	/**
	 * Iterable over all Locations in this CellAutomaton
	 * @see IGrid#locations()
	 */
	Iterable<Location> locations();
}
