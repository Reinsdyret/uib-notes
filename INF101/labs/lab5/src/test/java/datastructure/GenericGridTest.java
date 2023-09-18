package datastructure;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.ArrayList;
import java.util.List;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import cellular.cellstate.ICellState;
import labyrinth.LabyrinthTile;

public class GenericGridTest {

    List<IGrid> gridInstances;
    int nRows = 100;
    int nColumns = 100;

    @BeforeEach
    public void setup() {
        gridInstances = new ArrayList<>();
        gridInstances.add(new Grid<Integer>(nRows, nColumns, null));
        gridInstances.add(new Grid<String>(nRows, nColumns, null));
        gridInstances.add(new Grid<ICellState>(nRows, nColumns, null));
        gridInstances.add(new Grid<LabyrinthTile>(nRows, nColumns, null));
    }

    @Test
    public void genericNumColumns() {
        for (IGrid grid : gridInstances) {
            assertEquals(grid.numColumns(), nColumns);
        }
    }

    @Test
    public void genericNumRows() {
        for (IGrid grid : gridInstances) {
            assertEquals(grid.numRows(), nRows);
        }
    }

    @Test
    public void canCallGetGeneric() {
        for (IGrid grid : gridInstances) {
            grid.get(new Location(0, 0));
        }
    }
}
