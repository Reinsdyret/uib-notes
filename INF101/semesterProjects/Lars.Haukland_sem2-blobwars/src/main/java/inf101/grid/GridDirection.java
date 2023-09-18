package inf101.grid;

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
	public static final List<GridDirection> FOUR_DIRECTIONS = Arrays.asList(EAST, NORTH, WEST, SOUTH);
	/**
	 * The eight cardinal and intercardinal directions: {@link #NORTH},
	 * {@link #SOUTH}, {@link #EAST}, {@link #WEST}, {@link #NORTHWEST},
	 * {@link #NORTHEAST}, {@link #SOUTHWEST}, {@link #SOUTHEAST}.
	 */
	public static final List<GridDirection> EIGHT_DIRECTIONS = Arrays.asList(EAST, NORTHEAST, NORTH, NORTHWEST, WEST,
			SOUTHWEST, SOUTH, SOUTHEAST);
	/**
	 * The eight cardinal and intercardinal directions ({@link #EIGHT_DIRECTIONS}),
	 * plus {@link #CENTER}.
	 */
	public static final List<GridDirection> NINE_DIRECTIONS = Arrays.asList(EAST, NORTHEAST, NORTH, NORTHWEST, WEST,
			SOUTHWEST, SOUTH, SOUTHEAST, CENTER);

	private final int dx;
	private final int dy;

	GridDirection(int dx, int dy) {
		this.dx = dx;
		this.dy = dy;
	}

	/**
	 * Finds the Location adjacent in the given direction
	 *
	 * @param loc
	 * @return
	 */
	public Location getNeighbor(Location loc) {
		return new Location(loc.row + dy, loc.col + dx);
	}

	/**
	 * Returns the direction one get by rotating this direction
	 * 90 degrees to the right
	 *
	 * @return
	 */
	public GridDirection turnRight() {
		return this.turnRight45().turnRight45();
	}

	public GridDirection turnRight45() {
		return switch (this) {
			case NORTH -> GridDirection.NORTHEAST;
			case NORTHEAST -> GridDirection.EAST;
			case EAST -> GridDirection.SOUTHEAST;
			case SOUTHEAST -> GridDirection.SOUTH;
			case SOUTH -> GridDirection.SOUTHWEST;
			case SOUTHWEST -> GridDirection.WEST;
			case WEST -> GridDirection.NORTHWEST;
			case NORTHWEST -> GridDirection.NORTH;
			case CENTER -> GridDirection.CENTER;
		};
	}

	/**
	 * Returns the direction one get by rotating this direction
	 * 90 degrees to the left
	 *
	 * @return
	 */
	public GridDirection turnLeft() {
		return this.turnRight().turnRight().turnRight();
	}

	/**
	 * Returns the direction one get by rotating this direction
	 * 45 degrees to the left
	 *
	 * @return
	 */
	public GridDirection turnLeft45() {
		return this.turnLeft().turnRight45();
	}
}
