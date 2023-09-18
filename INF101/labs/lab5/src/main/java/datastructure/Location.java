package datastructure;

/**
 * This class represents a Location on a grid.
 * That means indices for row and column.
 * 
 * Location is Immutable, this means the only way to make a new
 * Location is to call the constructor.
 * 
 * @author Martin Vatshelle - martin.vatshelle@uib.no
 *
 */
public class Location {

	public final int row;
	public final int col;

	/**
	 * Constructs a new Location
	 * @param row the row index
	 * @param col the column index
	 */
	public Location(int row, int col) {
		this.row = row;
		this.col = col;
	}

	/**
	 * This method is just for convenience.
	 * @see GridDirection#getNeighbor()
	 * @param dir
	 * @return
	 */
	public Location getNeighbor(GridDirection dir) {
		return dir.getNeighbor(this);
	}
}
