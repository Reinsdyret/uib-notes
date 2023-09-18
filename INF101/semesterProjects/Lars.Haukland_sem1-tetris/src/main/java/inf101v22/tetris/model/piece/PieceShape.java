package inf101v22.tetris.model.piece;

import java.awt.Color;

import inf101v22.tetris.model.Tile;

public class PieceShape {
	private Tile tile;
	private boolean[][] shape;

	static final PieceShape T = new PieceShape(new Tile(Color.magenta, 'm'), new boolean[][] {
		{  true,  true,  true },
		{ false,  true, false },
	});

	static final PieceShape S = new PieceShape(new Tile(Color.green ,'g'), new boolean[][] {
		{ false,  true,  true },
		{  true,  true, false },
	});

	static final PieceShape Z = new PieceShape(new Tile(Color.red, 'r'), new boolean[][] {
		{  true,  true, false },
		{ false,  true,  true },
	});

	static final PieceShape I = new PieceShape(new Tile(Color.cyan, 'c'), new boolean[][] {
		{ true,  true,  true, true }
	});

	static final PieceShape J = new PieceShape(new Tile(Color.blue, 'b'), new boolean[][] {
		{ true, false, false },
		{ true, true,  true },
	});

	static final PieceShape L = new PieceShape(new Tile(Color.orange, 'o'), new boolean[][] {
		{ false, false,  true },
		{  true,  true,  true },
	});

	static final PieceShape O = new PieceShape(new Tile(Color.yellow, 'y'), new boolean[][] {
		{  true,  true },
		{  true,  true }
	});

	public static final PieceShape[] STANDARD_TETRIS_PIECES = {T, S, Z, I, J, L, O};

	/**
	 * Create a new PieceShape with given tile and shape
	 * @param tile
	 * @param shape
	 */
	public PieceShape(Tile tile, boolean[][] shape) {
		this.tile = tile;
		this.shape = shape;
	}

	/**
	 * Get the tile that is representing the piece
	 * @return a tile object
	 */
	public Tile getTile() {
		return this.tile;
	}

	/**
	 * Get the shape of the piece
	 * @return a 2d boolean array representing the shape
	 */
	public boolean[][] getShape() {
		return this.shape;
	}

	/**
	 * @return The height of the shape array
	 */
	public int getHeight() {
		return this.shape.length;
	}

	/**
	 * @return The width of the shape array
	 */
	public int getWidth() {
		return this.shape[0].length;
	}

	/**
	 * Produces a rotated copy of the pieceshape using transposition and reversing
	 * Method of doing this found on stackoverflow: https://stackoverflow.com/a/8664879
	 * @return a new pieceShape with the rotated shape
	 */
	public PieceShape rotatedCopy() {
		boolean[][] transpositioned = new boolean[this.getWidth()][this.getHeight()];
		boolean[][] rotated = new boolean[this.getWidth()][this.getHeight()];

		// Transposition the shape
		for(int row = 0; row < this.getHeight(); row++) {
			for(int col = 0; col < this.getWidth(); col++) {
				transpositioned[col][row] = this.shape[row][col];
			}
		}

		// Reverse each row
		for(int row = transpositioned.length - 1, row2 = 0; row >= 0; row--, row2++) {
			for(int col = 0; col < transpositioned[0].length; col++) {
				rotated[row2][col] = transpositioned[row][col];
			}
		}

		return new PieceShape(this.getTile(), rotated);
	}
}
