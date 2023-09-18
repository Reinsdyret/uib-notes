package datastructure;

import cellular.CellState;

public class CellGrid implements IGrid {
	
	int rows;
	int columns;
	CellState initialState;
	
	CellState[][] grid;
    public CellGrid(int rows, int columns, CellState initialState) {
		// TODO Auto-generated constructor stub
    	this.rows = rows;
    	this.columns = columns;
    	this.initialState = initialState;
    	
    	grid = new CellState[rows][columns];
    	
    	for (int row = 0; row < grid.length; row++) {
			for (int col = 0; col < grid[row].length; col++) {
				this.grid[row][col] = initialState;
			}
		}
	}

    @Override
    public int numRows() {
        // TODO Auto-generated method stub
        return this.rows;
    }

    @Override
    public int numColumns() {
        // TODO Auto-generated method stub
        return this.columns;
    }

    @Override
    public void set(int row, int column, CellState element) {
        // TODO Auto-generated method stub
        this.grid[row][column] = element;
    }

    @Override
    public CellState get(int row, int column) {
        // TODO Auto-generated method stub
        return this.grid[row][column];
    }

    @Override
    public IGrid copy() {
        // TODO Auto-generated method stub
        IGrid copyCellGrid = new CellGrid(this.rows, this.columns, this.initialState);
        for(int i = 0; i < this.grid.length; i++) {
        	for(int j = 0; j < this.grid[i].length; j++) {
        		copyCellGrid.set(i,j,this.grid[i][j]);
        	}
        }
		return copyCellGrid;
    }
    
}