package inf101v22.grid;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;


public class Grid<E> implements IGrid<E>{
	private final List<ArrayList<E>> cells;
	private final int rows;
	private final int cols;

	/**
	 * Create a new grid object
	 * Fills the grid with null values
	 * @param rows Number of rows in the grid
	 * @param cols Number of cols in the grid
	 */
	public Grid(int rows, int cols) {
		this.rows = rows;
		this.cols = cols;

		this.cells = new ArrayList<>(rows);
		for(int i = 0; i < rows; ++i) {
			this.cells.add(new ArrayList<E>(cols));
			for (int j = 0; j < cols; j++) {
				this.cells.get(i).add(null);
			}
		}
	}

	/**
	 * Create a new grid object
	 * Fills the grid with defaultValue
	 * @param rows Number of rows in the grid
	 * @param cols Number of cols in the grid
	 * @param defaultValue
	 */
	public Grid(int rows, int cols, E defaultValue) {
		this.rows = rows;
		this.cols = cols;

		this.cells = new ArrayList<>();
		for(int i = 0; i < rows; ++i) {
			this.cells.add(new ArrayList<E>());
		}

		for(int row = 0; row < this.rows; row++) {
			for(int col = 0; col < this.cols; col++) {
				this.cells.get(row).add(defaultValue);
			}
		}
	}



	@Override
	public Iterator<CoordinateItem<E>> iterator() {
		// TODO Auto-generated method stub
		List<CoordinateItem<E>> wholeGrid = new ArrayList<>();
		for(int row = 0; row < this.rows; row++) {
			for(int col = 0; col < this.cols; col++) {
				E value = this.cells.get(row).get(col);
				Coordinate coord = new Coordinate(row,col);
				wholeGrid.add(new CoordinateItem<>(coord,value));
			}
		}
		return wholeGrid.iterator();
	}

	@Override
	public int getRows() {
		// TODO Auto-generated method stub
		return this.rows;
	}

	@Override
	public int getCols() {
		// TODO Auto-generated method stub
		return this.cols;
	}

	@Override
	public void set(Coordinate coordinate, E value) throws IndexOutOfBoundsException{
		// TODO Auto-generated method stub

		if(!this.coordinateIsOnGrid(coordinate))
			throw new IndexOutOfBoundsException();

		this.cells.get(coordinate.row).set(coordinate.col, value);

	}

	@Override
	public E get(Coordinate coordinate) {
		// TODO Auto-generated method stub
		return this.cells.get(coordinate.row).get(coordinate.col);
	}

	@Override
	public boolean coordinateIsOnGrid(Coordinate coordinate) {
		// TODO Auto-generated method stub
		if((coordinate.col < 0) || (coordinate.col >= this.getCols()) || (coordinate.row < 0) || (coordinate.row >= this.getRows()))
			return false;
		return true;
	}


}
