package inf101.grid;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.ArrayList;
import java.util.function.Function;

import inf101.GetStarted;
import inf101.util.IGenerator;
import inf101.util.generators.GridGenerator;
import inf101.util.generators.LocationGenerator;
import inf101.util.generators.StringGenerator;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

public class GridTest {

    @BeforeEach
    void testReadConditions() {
        assertTrue(GetStarted.hasRead);
    }

    private static final int N = 10000;

    private final IGenerator<String> strGen = new StringGenerator();
    private final IGenerator<IGrid<String>> gridGen = new GridGenerator<>(strGen);

    @Test
    public void canConstruct() {
        Grid<Integer> grid = new Grid<>(3, 4);
        for (Location loc : grid.locations()) {
            assertEquals(null, grid.get(loc));
        }
        Grid<String> grid2 = new Grid<>(3, 4, "Value");
        for (Location loc : grid2.locations()) {
            assertEquals("Value", grid2.get(loc));
        }
    }

    public <T> void fillProperty1(IGrid<T> grid, T val) {
        grid.fill(val);
        for (Location l : grid.locations()) {
            assertEquals(val, grid.get(l));
        }
    }

    public <T> void fillProperty2(IGrid<T> grid, Function<Location, T> fun) {
        grid.fill(fun);
        for (Location l : grid.locations()) {
            assertEquals(fun.apply(l), grid.get(l));
        }
    }

    @Test
    public void fillTest1() {

        for (int i = 0; i < N / 10; i++) {
            IGrid<String> grid = gridGen.generate();

            String s = strGen.generate();
            fillProperty1(grid, s);
        }
    }

    @Test
    public void fillTest2() {
        for (int i = 0; i < N / 10; i++) {
            IGrid<String> grid = gridGen.generate();

            fillProperty2(grid, (l) -> l.toString());
        }
    }

    /**
     * A set on (x1,y1) doesn't affect a get on a different (x2,y2)
     */
    public <T> void setGetIndependentProperty(IGrid<T> grid, Location l1, Location l2, T val) {
        if (!l1.equals(l2)) {
            T s = grid.get(l2);
            grid.set(l1, val);
            assertEquals(s, grid.get(l2));
        }
    }

    @Test
    public void setGetIndependentTest() {
        for (int j = 0; j < 10; j++) {
            IGrid<String> grid = gridGen.generate();
            IGenerator<Location> lGen = new LocationGenerator(grid);

            for (int i = 0; i < N; i++) {
                Location l1 = lGen.generate();
                Location l2 = lGen.generate();
                String s = strGen.generate();

                setGetIndependentProperty(grid, l1, l2, s);
            }
        }
    }

    /**
     * get(x,y) is val after set(x, y, val)
     */
    public <T> void setGetProperty(IGrid<T> grid, Location l, T val) {
        grid.set(l, val);
        assertEquals(val, grid.get(l));
    }

    /**
     * Test that get gives back the same value after set.
     */
    @Test
    public void setGetTest() {
        for (int j = 0; j < 10; j++) {
            IGrid<String> grid = gridGen.generate();
            IGenerator<Location> lGen = new LocationGenerator(grid);

            for (int i = 0; i < N; i++) {
                Location l = lGen.generate();
                String s = strGen.generate();

                setGetProperty(grid, l, s);
            }
        }
    }

    @Test
    public void uniqueLocations() {
        for (int j = 0; j < 10; j++) {
            IGrid<String> grid = gridGen.generate();
            ArrayList<Location> found = new ArrayList<>();
            for (Location loc : grid.locations()) {
                assertFalse(found.contains(loc), "The location " + loc + " appeared twice in locations()");
                found.add(loc);
            }
        }
    }

    @Test
    public void testInvalidLocations() {
        for (int j = 0; j < 10; j++) {
            Grid<String> grid = (Grid<String>) gridGen.generate();
            assertFalse(grid.isOnGrid(new Location(-1, 3)));
            assertFalse(grid.isOnGrid(new Location(2, -2)));
            assertFalse(grid.isOnGrid(new Location(grid.numRows(), 0)));
            assertFalse(grid.isOnGrid(new Location(0, grid.numColumns())));
            assertFalse(grid.isOnGrid(null));
            assertThrows(IndexOutOfBoundsException.class, () -> grid.checkLocation(new Location(-1, 2)));
        }
    }

    @Test
    public void testCopy() {
        for (int j = 0; j < 10; j++) {
            Grid<String> grid = (Grid<String>) gridGen.generate();
            IGrid<String> newGrid = grid.copy();
            for (Location loc : grid.locations()) {
                assertEquals(grid.get(loc), newGrid.get(loc));
            }
        }

    }
}
