package inf101v22.grid;

import java.util.Objects;

public class Coordinate {
	// TODO: create this class
	public final int col;
	public final int row;

	/**
	 * Create a new coordinate object
	 * @param row
	 * @param col
	 */
	public Coordinate(int row, int col) {
		this.row = row;
		this.col = col;
	}

	@Override
	public int hashCode() {
		return Objects.hash(this.col, this.row);
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj)
			return true;
		if ((obj == null) || (this.getClass() != obj.getClass()))
			return false;
		Coordinate other = (Coordinate) obj;
		return (this.col == other.col) && (this.row == other.row);
	}

	@Override
	public String toString() {
		return "{ row='" + this.row + "', col='" + this.col + "' }";
	}
}
