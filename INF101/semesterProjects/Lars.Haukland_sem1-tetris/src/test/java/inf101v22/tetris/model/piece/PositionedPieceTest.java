package inf101v22.tetris.model.piece;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;

import org.junit.jupiter.api.Test;

import inf101v22.grid.Coordinate;
import inf101v22.grid.CoordinateItem;
import inf101v22.tetris.model.Tile;

/**
 * Testing for PositionedPiece.java
 * @author Lars Moeen Haukland (Yral)
 *
 */

public class PositionedPieceTest {

	@Test
	public void iteratorTest() {
		PieceShape piece = PieceShape.J;
		Coordinate upperLeft = new Coordinate(1,1);
		Tile myTile = piece.getTile();

		List<CoordinateItem<Tile>> expectedArray = new ArrayList<>();
		expectedArray.add(new CoordinateItem<>(new Coordinate(1,1), myTile));
		expectedArray.add(new CoordinateItem<>(new Coordinate(2,1), myTile));
		expectedArray.add(new CoordinateItem<>(new Coordinate(2,2), myTile));
		expectedArray.add(new CoordinateItem<>(new Coordinate(2,3), myTile));

		Iterator<CoordinateItem<Tile>> expectedIterator = expectedArray.iterator();

		Iterator<CoordinateItem<Tile>> testingPieceIterator = new PositionedPiece(piece, upperLeft).iterator();

		assertEquals(this.sizeOfIterator(expectedIterator),this.sizeOfIterator(testingPieceIterator));


	}

	/**
	 * Helper function for iteratorTest
	 * @param iterator of coordinateItems
	 * @return the size of the iterator
	 */
	private int sizeOfIterator(Iterator<CoordinateItem<Tile>> iterator) {
		int i = 0;
		while(iterator.hasNext()) {
			i++;
			iterator.next();
		}
		return i;
	}
}
