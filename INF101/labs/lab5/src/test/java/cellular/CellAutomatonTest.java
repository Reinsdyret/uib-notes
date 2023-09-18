package cellular;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;

import java.util.ArrayList;
import java.util.List;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import datastructure.Location;

class CellAutomatonTest {

	List<ICellAutomaton> instances;

	@BeforeEach
	void setUp() {
		setInstances(100, 100);
	}

	void setInstances(int rows, int cols) {
		instances = new ArrayList<>();
		instances.add(new GameOfLife(rows, cols));
		instances.add(new SeedsAutomaton(rows, cols));
		instances.add(new LangtonsAnt(rows, cols, "RRLLLRLLLRRR"));
	}

	@Test
	void testInitializeCells() {
		for (ICellAutomaton ca : instances) {
			for(Location loc : ca.locations()) {
				assertNotNull(ca.getCellState(loc));
			}
		}
	}

	@Test
	void testNumberOfRowsAndColumns() {
		for (int rows = 1; rows < 200; rows += 10) {
			for (int cols = 1; cols < 200; cols += 10) {
				setInstances(rows, cols);
				for (ICellAutomaton ca : instances) {
					assertEquals(rows, ca.numberOfRows(), "Number of Rows wrong for " + ca.getClass());
					assertEquals(cols, ca.numberOfColumns(), "Number of Columns wrong for " + ca.getClass());
				}
			}
		}
	}
}
