package inf101v22.tetris.model;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

import org.junit.jupiter.api.Test;

import inf101v22.tetris.model.piece.PieceShape;
import inf101v22.tetris.model.piece.PositionedPiece;
import inf101v22.tetris.model.piece.PositionedPieceFactory;

/**
 * Testing for TetrisModel.java
 * @author Lars Moeen Haukland (Yral)
 *
 */

public class TetrisModelTest {
	PositionedPiece piece = null;
	TetrisModel model = new TetrisModel();
	

	@Test
	public void getGameScreenTest() {
		 assertEquals(GameScreen.WELCOME_SCREEN, model.getGameScreen());
	}
	
	@Test
	public void setGameScreenTest() {
		model.setActiveGame();
		assertEquals(GameScreen.ACTIVE_GAME, model.getGameScreen());
	}
	
	@Test
	public void moveFallingPieceTest() {
		ArrayList<PieceShape> arr = new ArrayList<PieceShape>();
		arr.add(PieceShape.STANDARD_TETRIS_PIECES[0]);
		this.model = new TetrisModel(new PositionedPieceFactory(arr));
		this.piece = this.model.getNextPiece();
		this.model.moveFallingPiece(1, 2);
		assertEquals(this.piece.getUpperLeftCoord().row + 1, this.model.getPiece().next().coordinate.row);
		assertEquals(this.piece.getUpperLeftCoord().col + 2, this.model.getPiece().next().coordinate.col);
	}
	
	@Test
	public void dropPieceTest() {
		this.model.dropPiece();
		int row = this.model.getRows() - 2; // 1 because rows start at 0 + 1 because height = 2  
		assertEquals(row, this.model.getPiece().next().coordinate.row);
	}
	
	public void updateGameScreenTest() {
		this.model.createNewPiece();
		this.model.putPieceOnBoard();
		this.model.createNewPiece();
		this.model.putPieceOnBoard();
		assertEquals(GameScreen.GAME_OVER, this.model.getGameScreen());
	}
}
