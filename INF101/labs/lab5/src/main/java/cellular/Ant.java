package cellular;

import datastructure.GridDirection;
import datastructure.Location;

/**
 * An Ant can walk straight, turn right or turn left
 * Otherwise they don't do much.
 * Each Ant is in a position on a Grid and walks in a direction
 * 
 * @author Martin Vatshelle - martin.vatshelle@uib.no
 */
public class Ant {

	private Location loc;
	private GridDirection dir;

	/**
	 * Constructs a new Ant in a position walking in a direction.
	 * Note that using GridDirection.CENTER produces an Ant that does not move.
	 * 
	 * @param loc the location the Ant starts in.
	 * @param dir the direction the Ant is walking in.
	 */
	public Ant(Location loc, GridDirection dir) {
		this.loc = loc;
		this.dir = dir;
	}

	/**
	 * Changes the direction 90 degrees counter clockwise
	 */
	public void turnLeft() {
		dir = dir.turnLeft();
	}

	/**
	 * Changes the direction 90 degrees clockwise
	 */
	public void turnRight() {
		dir = dir.turnRight();
	}

	/**
	 * Moves this Ant one step straight forward in the direction
	 * this Ant is facing.
	 */
	public void move() {
		loc = loc.getNeighbor(dir);
	}

	public Location getLocation() {
		return loc;
	}

	public void setRow(int x) {
		setLocation(new Location(x, loc.col));
	}

	public void setCol(int y) {
		setLocation(new Location(loc.row, y));
	}

	public void setLocation(Location loc) {
		this.loc = loc;
	}

	public Ant copy() {
		return new Ant(loc, dir);
	}

	public int getRow() {
		return loc.row;
	}

	public int getCol() {
		return loc.col;
	}

}
