package inf101v22.grid;

import java.util.Objects;

public class CoordinateItem <E> {
	// TODO: create this class
	public final Coordinate coordinate;
	public final E item;

	/**
	 * Create a new coordinateItem object
	 * @param coord Coordinate to the item
	 * @param item 
	 */
	public CoordinateItem(Coordinate coord, E item) {
		this.coordinate = coord;
		this.item = item;
	}

	@Override
	public int hashCode() {
		return Objects.hash(this.coordinate, this.item);
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj)
			return true;
		if ((obj == null) || (this.getClass() != obj.getClass()))
			return false;
		CoordinateItem<E> other = (CoordinateItem<E>) obj;
		return Objects.equals(this.coordinate, other.coordinate) && Objects.equals(this.item, other.item);
	}

	@Override
	public String toString() {
		return "{ coordinate='{ row='" + this.coordinate.row + "', col='" + this.coordinate.col + "' }', item='" + this.item + "' }";
	}


}
