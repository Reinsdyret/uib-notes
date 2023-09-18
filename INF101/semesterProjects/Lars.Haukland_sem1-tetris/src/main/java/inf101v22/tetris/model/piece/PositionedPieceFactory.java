package inf101v22.tetris.model.piece;
import java.util.ArrayList;
import java.util.concurrent.ThreadLocalRandom;

import inf101v22.grid.Coordinate;

public class PositionedPieceFactory {
	int centerColumn;
	ArrayList<PieceShape> pieces;

	/**
	 * Create a new pieceFactory
	 */
	public PositionedPieceFactory() {}

	/**
	 * Create a new pieceFactory with a given list of pieces to choose from
	 * @param listOfPieces
	 */
	public PositionedPieceFactory(ArrayList<PieceShape> listOfPieces) {
		this.pieces = listOfPieces;
	}

	/**
	 * Defines the column of which pieces should be centered, stored in centerColumn
	 */
	public void setCenterColumn(int column) {
		this.centerColumn = column;
	}

	/**
	 * Generates a new tetris piece with either from the standard tetris pieces in pieceshape or from array given in constructor
	 * @return The new PositionedPiece
	 */
	public PositionedPiece getNextPositionedPiece() {
		PositionedPiece returningPiece = null;
		if(this.pieces != null) {
			if(this.pieces.size() > 0) {
				int max = this.pieces.size();

				// Way of finding random integer found on https://stackoverflow.com/a/363692
				// This way is supposedly more efficient as there is no need to initiate a random object
				int randomNum = ThreadLocalRandom.current().nextInt(0, max);

				PieceShape randPiece = this.pieces.get(randomNum);

				Coordinate upperLeftCoord = new Coordinate(0, Math.round(this.centerColumn - (randPiece.getWidth() / 2)));

				returningPiece = new PositionedPiece(randPiece, upperLeftCoord);
			}

		}else {
			int max = PieceShape.STANDARD_TETRIS_PIECES.length;

			// Way of finding random integer found on https://stackoverflow.com/a/363692
			// This way is supposedly more efficient as there is no need to initiate and random object
			int randomNum = ThreadLocalRandom.current().nextInt(0, max);

			PieceShape randPiece = PieceShape.STANDARD_TETRIS_PIECES[randomNum];

			Coordinate upperLeftCoord = new Coordinate(0, Math.round(this.centerColumn - (randPiece.getWidth() / 2)));

			returningPiece = new PositionedPiece(randPiece, upperLeftCoord);

		}

		return returningPiece;
	}
}
