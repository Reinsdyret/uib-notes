package labyrinth;


import static org.junit.jupiter.api.Assertions.assertDoesNotThrow;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import datastructure.GridDirection;
import datastructure.IGrid;
//import datastructure.ILabyrinthTileGrid;
import datastructure.Location;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

public class LabyrinthTest {

	private ILabyrinth labyrinth;

	@BeforeEach
	public void setup() {
		labyrinth = new Labyrinth(testGrid());
	}

	private IGrid<LabyrinthTile> testGrid() {
		return LabyrinthHelper.loadGrid(new char[][]{ //
				{'*', '*', '*', '*'}, //
				{'*', ' ', ' ', '*'}, //
				{'*', ' ', '*', '*'}, //
				{'*', 's', '*', '*'}, //
				{'*', '*', '*', '*'},});
	}

	@Test
	public void goodMoveTestN() {
		assertTrue(labyrinth.playerCanGo(GridDirection.NORTH));
		assertDoesNotThrow(() -> labyrinth.movePlayer(GridDirection.NORTH));
		assertEquals(LabyrinthTile.PLAYER, labyrinth.getCell(new Location(2, 1)));
	}

	@Test
	public void goodMoveTestNNE() {
		assertEquals(LabyrinthTile.PLAYER, labyrinth.getCell(new Location(3, 1)));
		assertDoesNotThrow(() -> labyrinth.movePlayer(GridDirection.NORTH));
		assertEquals(LabyrinthTile.PLAYER, labyrinth.getCell(new Location(2, 1)));
		assertDoesNotThrow(() -> labyrinth.movePlayer(GridDirection.NORTH));
		assertEquals(LabyrinthTile.PLAYER, labyrinth.getCell(new Location(1, 1)));
		assertDoesNotThrow(() -> labyrinth.movePlayer(GridDirection.EAST));
		assertEquals(LabyrinthTile.PLAYER, labyrinth.getCell(new Location(1, 2)));
	}

	@Test
	public void badMoveTestNWN() {
		assertEquals(LabyrinthTile.PLAYER, labyrinth.getCell(new Location(3, 1)));
		assertDoesNotThrow(() -> labyrinth.movePlayer(GridDirection.NORTH));
		assertEquals(LabyrinthTile.PLAYER, labyrinth.getCell(new Location(2, 1)));
		try {
			labyrinth.movePlayer(GridDirection.WEST);			
		} catch (Exception e) {
			//This test should pass whether or not exception is thrown
		}
		assertEquals(LabyrinthTile.PLAYER, labyrinth.getCell(new Location(2, 1)));
		assertDoesNotThrow(() -> labyrinth.movePlayer(GridDirection.NORTH));
		assertEquals(LabyrinthTile.PLAYER, labyrinth.getCell(new Location(1, 1)));
	}

	@Test
	public void badMoveTestNWNThrowsException() {
		assertEquals(LabyrinthTile.PLAYER, labyrinth.getCell(new Location(3, 1)));
		assertDoesNotThrow(() -> labyrinth.movePlayer(GridDirection.NORTH));
		assertEquals(LabyrinthTile.PLAYER, labyrinth.getCell(new Location(2, 1)));
		assertThrows(MovePlayerException.class,() -> labyrinth.movePlayer(GridDirection.WEST));
		assertEquals(LabyrinthTile.PLAYER, labyrinth.getCell(new Location(2, 1)));
		assertDoesNotThrow(() -> labyrinth.movePlayer(GridDirection.NORTH));
		assertEquals(LabyrinthTile.PLAYER, labyrinth.getCell(new Location(1, 1)));
	}
	
	@Test
	void throwWhenMultiplePlayersAreDefined() {

		assertThrows(LabyrinthParseException.class, () -> new Labyrinth(LabyrinthHelper.loadGrid(new char[][]{ //
				{'*', '*', '*', '*'}, //
				{'*', ' ', 's', '*'}, //
				{'*', ' ', '*', '*'}, //
				{'*', 's', '*', '*'}, //
				{'*', '*', '*', '*'},})), "Should throw exception if board has more than one player");
	}

	@Test
	void gridGet() {
		assertEquals(LabyrinthTile.WALL, labyrinth.getCell(new Location(0, 0)));
		assertEquals(LabyrinthTile.WALL, labyrinth.getCell(new Location(4, 2)));
		assertEquals(LabyrinthTile.WALL, labyrinth.getCell(new Location(1, 3)));
		assertEquals(LabyrinthTile.PLAYER, labyrinth.getCell(new Location(3, 1)));
		assertEquals(LabyrinthTile.OPEN, labyrinth.getCell(new Location(2, 1)));
	}

	@Test
	void throwWhenNoPlayersAreGiven() {
		assertThrows(LabyrinthParseException.class, () -> new Labyrinth(LabyrinthHelper.loadGrid(new char[][]{ //
				{'*', '*', '*', '*'}, //
				{'*', ' ', ' ', '*'}, //
				{'*', ' ', '*', '*'}, //
				{'*', ' ', '*', '*'}, //
				{'*', '*', '*', '*'},})), "Should throw exception if board have no players");
	}

}
