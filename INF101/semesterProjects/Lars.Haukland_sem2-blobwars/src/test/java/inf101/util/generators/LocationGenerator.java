package inf101.util.generators;

import java.util.Random;

import inf101.grid.IGrid;
import inf101.grid.Location;

public class LocationGenerator extends AbstractGenerator<Location> {
	private final IGrid<?> area;

	/**
	 * New LocationGenerator, will generate locations within the area
	 */
	public LocationGenerator(IGrid<?> area) {
		this.area = area;
	}

	@Override
	public Location generate(Random r) {
		return new Location(r.nextInt(area.numRows()), r.nextInt(area.numColumns()));
	}

}
