package inf101v22.tetris.model.piece;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;

import inf101v22.grid.Coordinate;
import inf101v22.grid.CoordinateItem;
import inf101v22.tetris.model.Tile;

public class PositionedPiece implements Iterable<CoordinateItem<Tile>>{
	private PieceShape shape;
	private Coordinate upperLeftCoord;


	PositionedPiece(PieceShape shape, Coordinate firstCoord) {
		this.shape = shape;
		this.upperLeftCoord = firstCoord;
	}

	/**
	 * @return the width of the shape
	 */
	public int getWidth() {
		return this.shape.getWidth();
	}

	/**
	 * @return the height of the shape
	 */
	public int getHeight() {
		return this.shape.getHeight();
	}

	/**
	 * @return The tile that is used to represent piece
	 */
	public Tile getTile() {
		return this.shape.getTile();
	}

	/**
	 * @return The shape of the piece
	 */
	public PieceShape getShape() {
		return this.shape;
	}

	@Override
	public Iterator<CoordinateItem<Tile>> iterator() {
		// TODO Auto-generated method stub
		List<CoordinateItem<Tile>> coords = new ArrayList<>();
		boolean[][] actualShape = this.shape.getShape();

		for(int row = 0; row < this.getHeight(); row++) {
			for(int col = 0; col < this.getWidth(); col++) {
				if(actualShape[row][col]) {
					coords.add(new CoordinateItem <>(new Coordinate(row + this.upperLeftCoord.row,col + this.upperLeftCoord.col), this.shape.getTile()));
				}
			}
		}
		return coords.iterator();
	}

	/**
	 * Generates a new PositionedPiece with shifted coordinates based on parameters
	 * @param deltaRow number of rows to shift piece
	 * @param deltaCol number of rows to shift piece
	 * @return an iterator of the new coordinateItems
	 */
	public PositionedPiece copyMoved(int deltaRow, int deltaCol) {
		Coordinate newUpperLeft = new Coordinate(this.upperLeftCoord.row + deltaRow, this.upperLeftCoord.col + deltaCol);
		return new PositionedPiece(this.shape, newUpperLeft);
	}

	/**
	 * Makes a rotated copy of the shape
	 * @return a new rotated positioned piece
	 */
	public PositionedPiece rotated() {
		Coordinate center = new Coordinate(this.getHeight() / 2, this.getWidth() / 2);
		PieceShape rotatedShape = this.shape.rotatedCopy();
		Coordinate newCenter = new Coordinate(this.getWidth() / 2 , this.getHeight() / 2);
		Coordinate centerDiff = new Coordinate(center.row - newCenter.row, center.col - newCenter.col);
		Coordinate newUpperLeft = new Coordinate(this.upperLeftCoord.row + centerDiff.row, this.upperLeftCoord.col + centerDiff.col);

		return new PositionedPiece(rotatedShape, newUpperLeft);
	}

	/**
	 * Get the upper left coordinate of the piece
	 * @return The coordinate
	 */
	public Coordinate getUpperLeftCoord() {
		return this.upperLeftCoord;
	}

}
