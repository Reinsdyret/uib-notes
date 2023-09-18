package inf101.sem2.game;

import java.util.List;
import java.util.Objects;

import inf101.grid.Grid;
import inf101.grid.GridDirection;
import inf101.grid.GridLocationIterator;
import inf101.grid.Location;
import inf101.sem2.player.Player;

public class GameBoard {

	private Grid<Player> grid;

	public GameBoard(int rows, int cols) {
		grid = new Grid<>(rows, cols);
	}

	public void set(Location loc, Player elem) {
		if (canPlace(loc)) {
			grid.set(loc, elem);
		} else {
			System.err.println("Can not place at " + loc + ".");
		}
	}

	/**
	 * Place new piece at location even if there is already another piece there.
	 * 
	 * @param loc
	 * @param elem
	 */
	public void swap(Location loc, Player elem) {
		grid.set(loc, elem);
	}

	public Player get(Location loc) {
		return grid.get(loc);
	}

	public int numColumns() {
		return grid.numColumns();
	}

	public int numRows() {
		return grid.numRows();
	}

	/**
	 * This method determines whether or not it is allowed to place on a given
	 * location.
	 * This method does not consider the rules of the game just whether the space on
	 * the gameBoard
	 * is available.
	 * <p>
	 * This implementation only allows for one piece in each square of the board.
	 *
	 * @return If the given location is empty
	 */
	public boolean canPlace(Location loc) {
		try {
			return grid.get(loc) == null;
		} catch (Exception e) {
			return false;
		}
	}

	/**
	 * Move piece at location <code>from</code> to location <code>to</code>.
	 * 
	 * @param from
	 * @param to
	 */
	public void movePiece(Location from, Location to) {
		Player piece = grid.get(from);
		if (piece == null) {
			System.err.println("No piece at location: " + from + ".");
		} else if (canPlace(to)) {
			grid.set(from, null);
			grid.set(to, piece);
		} else {
			System.err.println("Can not place at " + to + ".");
		}
	}

	@Override
	public String toString() {
		String s = "";

		for (int row = 0; row < grid.numRows(); row++) {
			// print row
			for (int col = 0; col < grid.numColumns(); col++) {
				Player p = grid.get(new Location(row, col));
				if (p == null) {
					s += ' ';
				} else {
					s += p.getSymbol();
				}
				if (col < grid.numColumns() - 1) {
					s += '|';
				}
			}
			// print newline
			s += "\n";

			// print horizontal separator
			if (row < grid.numRows() - 1) {
				for (int col = 0; col < grid.numColumns(); col++) {
					s += '-';
					if (col < grid.numColumns() - 1) {
						s += '+';
					}
				}
				s += "\n";
			}

		}

		return s;
	}

	/**
	 * @return The maximum number of pieces in a row that the given player has.
	 */
	public int countNumInRow(Player p) {
		int max = 0;
		for (Location loc : locations()) {
			for (GridDirection dir : GridDirection.EIGHT_DIRECTIONS) {
				max = Math.max(max, countNumInRow(p, loc, dir));
			}
		}
		return max;
	}

	private int countNumInRow(Player p, Location start, GridDirection dir) {
		int count = 0;
		while (grid.isOnGrid(start) && grid.get(start) == p) {
			count++;
			start = start.getNeighbor(dir);
		}
		return count;
	}

	/**
	 * This method checks whether all locations of the board is occupied
	 * or there are any empty places on the board.
	 *
	 * @return If the board is full
	 */
	public boolean isFull() {
		for (Location loc : locations()) {
			if (canPlace(loc)) {
				return false;
			}
		}
		return true;
	}

	/**
	 * Makes a shallow copy of the board
	 */
	public GameBoard copy() {
		GameBoard board = new GameBoard(grid.numRows(), grid.numColumns());
		grid.fillCopy(board.grid);
		return board;
	}

	public GridLocationIterator locations() {
		return grid.locations();
	}

	public void clear() {
		grid.clear();
	}

	/**
	 * Set the contents of all cells to <code>element</code>
	 * <p>
	 * For example:
	 *
	 * <pre>
	 * // clear the grid
	 * grid.setAll(null);
	 * </pre>
	 *
	 * @param initialiser
	 */
	public void fill(Player element) {
		grid.fill(element);
	}

	/**
	 * Count the number of piece of player <code>player</code>
	 * @param player
	 * @return number of pieces of player on the board
	 */
	public int countPieces(Player player) {
		int total = 0;
		for (Location loc : locations()) {
			if (player == get(loc)) {
				total++;
			}
		}
		return total;
	}

	public boolean isOnBoard(Location loc) {
		return this.grid.isOnGrid(loc);
	}

	@Override
	public int hashCode() {
		return Objects.hash(grid);
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj)
			return true;
		if (obj == null)
			return false;
		if (getClass() != obj.getClass())
			return false;
		GameBoard other = (GameBoard) obj;
		return Objects.equals(grid, other.grid);
	}
	
}
