package inf101.grid;

import java.util.Comparator;

public class LocationComparator implements Comparator<Location> {

	private Location loc;

	public LocationComparator(Location loc) {
		this.loc = loc;
	}

	@Override
	public int compare(Location loc1, Location loc2) {
		int dist1 = loc.gridDistanceTo(loc1);
		int dist2 = loc.gridDistanceTo(loc2);
		return Integer.compare(dist1, dist2);
	}

}
