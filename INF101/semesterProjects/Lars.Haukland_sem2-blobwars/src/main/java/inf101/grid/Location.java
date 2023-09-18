package inf101.grid;

import java.util.ArrayList;
import java.util.Collection;

/**
 * This class represents a Location on a grid.
 * That means indices for row and column.
 *
 * @author Martin Vatshelle - martin.vatshelle@uib.no
 */
public class Location {

	public final int row;
	public final int col;

	/**
	 * Constructor for location, note that a Location is independent of the Grid
	 * implementation,
	 * hence can have values not corresponding to a cell in the grid.
	 *
	 * @param row
	 * @param col
	 */
	public Location(int row, int col) {
		this.row = row;
		this.col = col;
	}

	/**
	 * Gets the Location of the adjacent cell in the given direction.
	 * {@link GridDirection#getNeighbor}
	 *
	 * @param dir The direction to go in
	 * @return The adjacent location
	 */
	public Location getNeighbor(GridDirection dir) {
		return dir.getNeighbor(this);
	}

	/**
	 * Returns the Manhattan distance between 2 locations.
	 * That is the shortest distance distance between two points
	 * if you can only go East,West,North and South (not diagonally)
	 *
	 * @param loc
	 * @return
	 */
	public int gridDistanceTo(Location loc) {
		return Math.abs(row - loc.row) + Math.abs(col - loc.col);
	}

	/**
	 * Returns a list of the eight neighbors around this location
	 *
	 * @return
	 */
	public Collection<Location> allNeighbors() {
		ArrayList<Location> neighbours = new ArrayList<>();
		for (GridDirection dir : GridDirection.EIGHT_DIRECTIONS) {
			neighbours.add(this.getNeighbor(dir));
		}
		return neighbours;
	}

	@Override
	public boolean equals(Object obj) {
		if (obj instanceof Location) {
			Location loc = (Location) obj;
			return this.row == loc.row && this.col == loc.col;
		}
		return false;
	}

	@Override
	public int hashCode() {
		final int prime = 31;
		int result = 1;
		result = prime * result + col;
		result = prime * result + row;
		return result;
	}

	@Override
	public String toString() {
		return "(" + row + "," + col + ")";
	}

	/**
	 * Finds one GridDirection such that the distance to loc decreases if one go in
	 * that direction.
	 *
	 * @param loc The location one wants to go in.
	 * @return The direction to go in.
	 */
	public GridDirection directionTo(Location loc) {
		GridDirection bestDir = GridDirection.CENTER;
		int bestDist = this.gridDistanceTo(loc);
		for (GridDirection dir : GridDirection.EIGHT_DIRECTIONS) {
			Location neighbor = this.getNeighbor(dir);
			int curDist = neighbor.gridDistanceTo(loc);
			if (curDist < bestDist) {
				bestDist = curDist;
				bestDir = dir;
			}
		}
		return bestDir;
	}
}
