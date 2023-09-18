package datastructure;

import java.util.Arrays;
import java.util.List;

/**
 * This class represents the 8 different directions
 * (plus the direction of no movement) in in a grid
 * which one can move by changing either x or y coordinate by at most 1
 */
public enum GridDirection {
	EAST(1, 0),
	NORTH(0, -1),
	WEST(-1, 0),
	SOUTH(0, 1), //
	NORTHEAST(1, -1),
	NORTHWEST(-1, -1),
	SOUTHWEST(-1, 1),
	SOUTHEAST(1, 1), //
	CENTER(0, 0);

	/**
	 * The four cardinal directions: {@link #NORTH}, {@link #SOUTH}, {@link #EAST},
	 * {@link #WEST}.
	 */
	public static final List<GridDirection> FOUR_DIRECTIONS = Arrays.asList(
			EAST,
			NORTH,
			WEST,
			SOUTH);
	/**
	 * The eight cardinal and intercardinal directions: {@link #NORTH},
	 * {@link #SOUTH}, {@link #EAST}, {@link #WEST}, {@link #NORTHWEST},
	 * {@link #NORTHEAST}, {@link #SOUTHWEST}, {@link #SOUTHEAST}.
	 */
	public static final List<GridDirection> EIGHT_DIRECTIONS = Arrays.asList(
			EAST,
			NORTHEAST,
			NORTH,
			NORTHWEST,
			WEST,
			SOUTHWEST,
			SOUTH,
			SOUTHEAST);
	/**
	 * The eight cardinal and intercardinal directions ({@link #EIGHT_DIRECTIONS}),
	 * plus {@link #CENTER}.
	 */
	public static final List<GridDirection> NINE_DIRECTIONS = Arrays.asList(
			EAST,
			NORTHEAST,
			NORTH,
			NORTHWEST,
			WEST,
			SOUTHWEST,
			SOUTH,
			SOUTHEAST,
			CENTER);

	private final int dx;
	private final int dy;

	GridDirection(int dx, int dy) {
		this.dx = dx;
		this.dy = dy;
	}

	/**
	 * Finds the Location adjacent in the given direction
	 * @param loc
	 * @return
	 */
	public Location getNeighbor(Location loc) {
		return new Location(loc.row + dy, loc.col + dx);
	}

	/**
	 * Returns the direction one get by rotatint this direction
	 * 90 degrees to the left
	 * @return
	 */
	public GridDirection turnLeft() {
		return turnLeft45().turnLeft45();
	}

	public GridDirection turnLeft45() {
		switch(this) {
			case NORTH:
				return GridDirection.NORTHWEST;
			case NORTHWEST:
				return GridDirection.WEST;
			case WEST:
				return GridDirection.SOUTHWEST;
			case SOUTHWEST:
				return GridDirection.SOUTH;
			case SOUTH:
				return GridDirection.SOUTHEAST;
			case SOUTHEAST:
				return GridDirection.EAST;
			case EAST:
				return GridDirection.NORTHEAST;
			case NORTHEAST:
				return GridDirection.NORTH;
			case CENTER:
				return GridDirection.CENTER;
			default:
				throw new IllegalStateException("Unknown state of Direction");
		}
	}


	public GridDirection turnRight() {
		return turnRight45().turnRight45();
	}

	public GridDirection turnRight45() {
		switch(this) {
			case NORTH:
				return GridDirection.NORTHEAST;
			case NORTHEAST:
				return GridDirection.EAST;
			case EAST:
				return GridDirection.SOUTHEAST;
			case SOUTHEAST:
				return GridDirection.SOUTH;
			case SOUTH:
				return GridDirection.SOUTHWEST;
			case SOUTHWEST:
				return GridDirection.WEST;
			case WEST:
				return GridDirection.NORTHWEST;
			case NORTHWEST:
				return GridDirection.NORTH;
			case CENTER:
				return GridDirection.CENTER;
			default:
				throw new IllegalStateException("Unknown state of Direction");
		}
	}
}
