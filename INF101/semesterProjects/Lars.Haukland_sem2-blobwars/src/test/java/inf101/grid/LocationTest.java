package inf101.grid;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.HashSet;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import inf101.GetStarted;

public class LocationTest {
	@BeforeEach
	void testReadConditions() {
		assertTrue(GetStarted.hasRead);
	}

	Location loc = new Location(3, 4);

	@Test
	void testConstruct() {
		assertEquals(3, loc.row);
		assertEquals(4, loc.col);
	}

	@Test
	void testGridDist() {
		for (GridDirection dir : GridDirection.FOUR_DIRECTIONS) {
			Location neighbor = loc.getNeighbor(dir);
			assertEquals(1, loc.gridDistanceTo(neighbor));
			assertEquals(1, neighbor.gridDistanceTo(loc));
		}
	}

	@Test
	void testAllNeighbors() {
		HashSet<Location> locs = new HashSet<>();
		for (Location neighbor : loc.allNeighbors()) {
			assertTrue(locs.add(neighbor));
		}
		assertEquals(8, locs.size());
	}

	@Test
	void testEquals() {
		assertEquals(loc, new Location(3, 4));
		assertNotEquals(loc, new Location(4, 3));
		assertNotEquals(loc, "(3,4)");
	}

	@Test
	void testDirectionTo() {
		for (GridDirection dir : GridDirection.values()) {
			Location neighbor = loc.getNeighbor(dir);
			assertEquals(dir, loc.directionTo(neighbor));
		}
	}
}
