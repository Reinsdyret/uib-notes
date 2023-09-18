package inf101v22.tetris.model;

import java.util.ArrayList;
import java.util.Iterator;

import inf101v22.grid.Coordinate;
import inf101v22.grid.CoordinateItem;
import inf101v22.grid.Grid;

public class TetrisBoard extends Grid<Tile>{

	/**
	 * Creates a new TetrisBoard and fills it with null values
	 * Supers the constructor to the Grid constructor
	 * @param rows Number of rows on the board
	 * @param cols Number of cols on the board
	 */
	public TetrisBoard(int rows, int cols) {
		super(rows, cols);
		// TODO Auto-generated constructor stub
	}

	/**
	 * Creates a new TetrisBoard and fills it with tile given
	 * @param rows Number of rows on the board
	 * @param cols Number of cols on the board
	 * @param tile The tile to be filled on the board
	 */
	public TetrisBoard(int rows, int cols,Tile tile) {
		super(rows, cols, tile);
		// TODO Auto-generated constructor stub
	}

	/**
	 * Generates a 2d array of characters representing the tetris grid
	 * a '-' represents a null value and any other character is a color
	 * @returns A char[][] representing the tetris grid
	 */
	public char[][] toCharArray2d(){
		int rows = this.getRows();
		int cols = this.getCols();
		char[][] grid = new char[rows][cols];

		Iterator<CoordinateItem<Tile>> gridIterator = this.iterator();

		while(gridIterator.hasNext()) {
			CoordinateItem<Tile> coordinateItem = gridIterator.next();
			int row = coordinateItem.coordinate.row;
			int col = coordinateItem.coordinate.col;
			char character;

			if(coordinateItem.item != null) {
				character = coordinateItem.item.character;
			}else {
				character = '-';
			}
			grid[row][col] = character;
		}

		return grid;
	}

	/**
	 * Produces a String from a 2d array of chars
	 * @param charArray The 2d array of chars
	 * @return A string
	 */
	public String charArray2dToString(char[][] charArray) {
		String finalString = "";

		for (char[] cs : charArray) {
			for(char c : cs) {
				finalString += c;
			}
			finalString += "\n";
		}

		return finalString;
	}

	/**
	 * Removes all rows with no null values
	 * @return a number of rows removed
	 */
	public int removeRowsIfPossible() {
		int rowsRemoved = 0;
		for(int row = this.getRows() - 1; row>=0; row--) {
			if(!this.nullExistsInRow(row)) {
				this.fillRowWithNulls(row);
				rowsRemoved++;
			}
		}
		for(int i = 0; i<rowsRemoved; i++) {
			this.shiftRows();
		}

		return rowsRemoved;
	}

	/**
	 * Loops through rows starting at bottom and checks if the row is filled with null values
	 * If so then copies all values from the row above and sets them in the current row
	 * Then sets the whole row above to be null values
	 */
	public void shiftRows() {
		for(int row = this.getRows() - 1; row>0; row--) {
			if(this.rowFilledWithNulls(row)) {
				for(int col = 0; col< this.getCols(); col++) {
					Coordinate currentCoord = new Coordinate(row, col);
					Coordinate coordAbove = new Coordinate(row - 1, col);
					this.set(currentCoord, this.get(coordAbove));
					this.set(coordAbove, null);
				}
			}
		}
	}

	/**
	 * Checks if a row is filled with null values
	 * @param row to check
	 * @return true if the row is all nulls
	 */
	public boolean rowFilledWithNulls(int row) {
		Iterator<CoordinateItem<Tile>> gridIterator = this.iterator();
		while(gridIterator.hasNext()) {
			CoordinateItem<Tile> currentTile = gridIterator.next();
			if(currentTile.coordinate.row != row) {
				continue;
			}
			if(currentTile.item != null) return false;
		}
		return true;
	}

	/**
	 * A method for checking if the given row has a null value in it
	 * @param row to check
	 * @return True if null exists and false if not
	 */
	public boolean nullExistsInRow(int row) {
		ArrayList<CoordinateItem<Tile>> rowArray = this.copyRow(row);
		for(CoordinateItem<Tile> tile : rowArray) {
			if(tile.item == null) return true;
		}
		return false;
	}

	/**
	 * A method for filling a certain row with null value
	 * @param row
	 */
	public void fillRowWithNulls(int row) {
		int cols = this.getCols();
		for(int col = 0; col<cols; col++) {
			this.set(new Coordinate(row, col),  null);
		}
	}

	/**
	 * Copies a given row
	 * @param row
	 * @return a copy
	 */
	private ArrayList<CoordinateItem<Tile>> copyRow(int row){
		ArrayList<CoordinateItem<Tile>> values = new ArrayList<>();
		Iterator<CoordinateItem<Tile>> gridIterator = this.iterator();
		while(gridIterator.hasNext()) {
			CoordinateItem<Tile> currentTile = gridIterator.next();
			if(currentTile.coordinate.row != row) {
				continue;
			}
			values.add(currentTile);
		}
		return values;
	}

}
