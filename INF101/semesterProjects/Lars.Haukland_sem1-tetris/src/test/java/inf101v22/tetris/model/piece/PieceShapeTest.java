package inf101v22.tetris.model.piece;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.awt.Color;

import org.junit.jupiter.api.Test;

import inf101v22.tetris.model.Tile;

/**
 * Testing for PieceShape.java
 * @author Lars Moeen Haukland (Yral)
 *
 */

public class PieceShapeTest {

	Tile tile = new Tile(Color.red, 'r');
	boolean[][] shape =new boolean[][] {
		{true, true, true},
		{false, true, false}
	};
	PieceShape myPiece = new PieceShape(this.tile, this.shape);

	@Test
	public void getTileTest() {
		assertEquals(this.tile, this.myPiece.getTile());

		PieceShape myNewPiece = PieceShape.STANDARD_TETRIS_PIECES[0];
		assertEquals(new Tile(Color.magenta, 'm').color, myNewPiece.getTile().color);
	}

	@Test
	public void getShapeTest() {
		assertEquals(this.shape, this.myPiece.getShape());
	}

	@Test
	public void getWidthTest() {
		assertEquals(this.shape[0].length, this.myPiece.getWidth());
	}

	@Test
	public void getHeightTest() {
		assertEquals(this.shape.length, this.myPiece.getHeight());
	}

	@Test
	public void rotatedCopyTest() {
		boolean[][] wantedArray = {
				{true, false},
				{true, true},
				{true, false}
		};
		
		System.out.println(this.myPiece.rotatedCopy().getShape().length);
		System.out.println(wantedArray.length);
		assertTrue(this.testBooleanArray(wantedArray, this.myPiece.rotatedCopy().getShape()));
	}
	
	private boolean testBooleanArray(boolean[][] arr1, boolean[][] arr2) {
		for (int i = 0; i < arr2.length; i++) {
			for (int j = 0; j < arr2[i].length; j++) {
				if(arr1[i][j] != arr2[i][j]) return false;
			}
		}
		return true;
	}

	/**
	 * Helper function when testing rotatedCopy, prints out a boolean[][]
	 * @param piece
	 */
	private void printBooleanArray(PieceShape piece) {
		boolean[][] shape = piece.getShape();
		for(boolean[]row : shape) {
			for(boolean col : row) {
				System.out.print(col + ", ");
			}
			System.out.println("\n");
		}
	}
}
