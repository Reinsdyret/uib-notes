package labyrinth;

import java.awt.Color;

import datastructure.GridDirection;
import datastructure.IGrid;
import datastructure.Location;

public class Labyrinth implements ILabyrinth {
	private final IGrid<LabyrinthTile> tiles;
	//private int playerX = -1;
	//private int playerY = -1;
	private Location playerLoc;

	boolean playerSet;

	public Labyrinth(IGrid<LabyrinthTile> tiles) throws LabyrinthParseException {
		if(tiles == null) {
			throw new IllegalArgumentException();
		}

		this.tiles = tiles;

		int numPlayers = 0;
		for(Location loc : tiles.locations()) {
			if(tiles.get(loc) == LabyrinthTile.PLAYER) {
				numPlayers++;
				playerLoc = loc;
				playerSet = true;
			}
		}
		if(numPlayers != 1) {
			throw new LabyrinthParseException("Labyrinth created with " + numPlayers + " number of players!");
		}

		checkState(this);
	}

	public static void checkState(Labyrinth labyrinth) {
		boolean ok = !labyrinth.playerSet || labyrinth.isValidPos(labyrinth.playerLoc);
		int numPlayers = 0;
		for(Location loc : labyrinth.tiles.locations()) {
			if(labyrinth.tiles.get(loc) == LabyrinthTile.PLAYER) {
				numPlayers++;
			}
		}
		if(labyrinth.playerSet) {
			ok &= numPlayers == 1;
		} else {
			ok &= numPlayers == 0;
		}
		if(!ok) {
			throw new IllegalStateException("bad object");
		}
	}

	@Override
	public LabyrinthTile getCell(Location loc) {
		checkPosition(loc);

		return tiles.get(loc);
	}

	@Override
	public Color getColor(Location loc) {
		if(!isValidPos(loc)) {
			throw new IllegalArgumentException("Location invalid");
		}

		return tiles.get(loc).getColor();
	}

	@Override
	public int numberOfRows() {
		return tiles.numRows();
	}

	@Override
	public int getPlayerGold() {
		return 0;
	}

	@Override
	public int getPlayerHitPoints() {
		return 0;
	}

	@Override
	public int numberOfColumns() {
		return tiles.numColumns();
	}

	@Override
	public boolean isPlaying() {
		return playerSet;
	}

	private boolean isValidPos(Location loc) {
		return loc.row >= 0 && loc.row < tiles.numRows() //
				&& loc.col >= 0 && loc.col < tiles.numColumns();
	}

	private void checkPosition(Location loc) {
		if(!isValidPos(loc)) {
			throw new IndexOutOfBoundsException("Row and column indices must be within bounds");
		}
	}

	@Override
	public void movePlayer(GridDirection d) throws MovePlayerException {
		//TODO: check pre-conditions
		Location newLoc = playerLoc.getNeighbor(d);
		
		if((newLoc.row >= this.tiles.numRows()) || (newLoc.col >= this.tiles.numColumns()) || (!playerCanGo(d))) {
			throw new MovePlayerException("Bad");
		}
		
		tiles.set(playerLoc, LabyrinthTile.OPEN);
		playerLoc = newLoc;
		tiles.set(newLoc, LabyrinthTile.PLAYER);

		checkState(this);
	}

	@Override
	public boolean playerCanGo(GridDirection d) {
		if(d == null) {
			throw new IllegalArgumentException();
		}

		return playerCanGoTo(playerLoc.getNeighbor(d));
	}

	/**
	 * This method checks if a player can move to a given location
	 * A player can not go to the location if there is a wall or
	 * if the location is outside the bounds of the grid.
	 * 
	 * @param loc
	 * @return
	 */
	private boolean playerCanGoTo(Location loc) {
		if(!isValidPos(loc)) {
			return false;
		}

		return tiles.get(loc) != LabyrinthTile.WALL;
	}

	@Override
	public String toString() {
		StringBuilder sb = new StringBuilder();

		for(int y = tiles.numColumns() - 1; y >= 0; y--) {
			for(int x = 0; x < tiles.numRows(); x++) {
				sb.append(getSymbol(new Location(x, y)));
			}
			sb.append("\n");
		}
		return sb.toString();
	}

	/**
	 * No bounds checking will be done for the given {@code loc}.
	 */
	private String getSymbol(Location loc) {
		return String.valueOf(tiles.get(loc).getSymbol());
	}

	@Override
	public Iterable<Location> locations() {
		return tiles.locations();
	}
}
