package inf101v22.tetris.model;

import java.util.Iterator;

import inf101v22.grid.Coordinate;
import inf101v22.grid.CoordinateItem;
import inf101v22.tetris.controller.TetrisControllable;
import inf101v22.tetris.model.piece.PositionedPiece;
import inf101v22.tetris.model.piece.PositionedPieceFactory;
import inf101v22.tetris.view.TetrisViewable;

public class TetrisModel implements TetrisViewable, TetrisControllable{

	private TetrisBoard board = new TetrisBoard(15,10);

	PositionedPiece piece;
	PositionedPieceFactory pieceFactory;
	GameScreen gameScreen = GameScreen.WELCOME_SCREEN;
	private int gameScore = 0;
	public final int fallTime = 1000;
	int piecesMade = 0;

	/**
	 * Create a new TetrisModel with a new pieceFactory with a standard set of pieces
	 * Also inits the first piece
	 */
	public TetrisModel() {
		/*
		board.set(new Coordinate(0,0), new Tile(Color.yellow, 'y'));
		board.set(new Coordinate(14,0), new Tile(Color.red, 'r'));
		board.set(new Coordinate(0,9), new Tile(Color.blue, 'b'));
		board.set(new Coordinate(14,9), new Tile(Color.green, 'g'));*/

		this.pieceFactory = new PositionedPieceFactory();
		this.pieceFactory.setCenterColumn(Math.round(this.getCols()/2));
		this.piece = this.pieceFactory.getNextPositionedPiece();
		this.piecesMade++;
	}

	/**
	 * Creates a new TetrisModel with a given pieceFactory
	 * Inits the first piece and all pieces after from the set of pieces in the given factory
	 * @param factory The custom pieceFactory
	 */
	public TetrisModel(PositionedPieceFactory factory) {
		this.pieceFactory = factory;
		this.pieceFactory.setCenterColumn(this.getCols()/2);
		this.piece = this.pieceFactory.getNextPositionedPiece();
		this.piecesMade++;
	}

	/**
	 * Makes a char array that represents the tetris board.
	 * '-' represents empty cells and a letter represents what color that cell is
	 * @return Char array representing tetris board
	 */
	public char[][] charBoardWithPiece(){
		char[][] charBoard = this.board.toCharArray2d();
		Iterator<CoordinateItem<Tile>> coordsOfPiece = this.piece.iterator();
		char colorChar = this.piece.getTile().character;

		while(coordsOfPiece.hasNext()) {
			Coordinate currentCoords = coordsOfPiece.next().coordinate;
			charBoard[currentCoords.row][currentCoords.col] = colorChar;
		}

		return charBoard;

	}

	@Override
	public int getRows() {
		// TODO Auto-generated method stub
		return this.board.getRows();
	}

	@Override
	public int getCols() {
		// TODO Auto-generated method stub
		return this.board.getCols();
	}

	@Override
	public int getScore() {
		return this.gameScore;
	}

	@Override
	public void setActiveGame() {
		this.gameScreen = GameScreen.ACTIVE_GAME;
	}

	@Override
	public Iterable<CoordinateItem<Tile>> Iterable() {
		// TODO Auto-generated method stub
		return this.board;
	}

	@Override
	public Iterator<CoordinateItem<Tile>> getPiece() {
		// TODO Auto-generated method stub
		return this.piece.iterator();
	}

	@Override
	public boolean moveFallingPiece(int deltaRow, int deltaCol) {
		// TODO Auto-generated method stub
		PositionedPiece movedPiece = this.piece.copyMoved(deltaRow, deltaCol);

		// Test if movement is valid
		if(!this.validMovement(movedPiece)) return false;

		// Update piece
		this.piece = this.piece.copyMoved(deltaRow, deltaCol);

		return true;
	}



	@Override
	public boolean rotatePiece() {
		// Check if piece can be rotated
		if(!this.validMovement(this.piece.rotated())) return false;

		// Update piece
		this.piece = this.piece.rotated();

		return true;

	}

	/**
	 * Checks if the given iterator of tiles are open on the board
	 * @param newLocations iterator of tiles
	 * @return true if tiles are open, false otherwise
	 */
	private boolean canMoveToNewLocation(Iterator<CoordinateItem<Tile>> newLocations) {
		while(newLocations.hasNext()) {
			CoordinateItem<Tile> currentTile = newLocations.next();
			if(this.board.get(currentTile.coordinate) != null)
				return false;
		}
		return true;
	}

	/**
	 * Checks if the given pieceIterator is possible to be placed on the board
	 * Checks if coordinates are out of bounds and if the coordinates are open
	 * @param piece
	 * @return a boolean value representing if the movement is valid or not
	 */
	private boolean validMovement(PositionedPiece piece) {

		Iterator<CoordinateItem<Tile>> pieceIterator = piece.iterator();

		Coordinate firstCoord = piece.getUpperLeftCoord();
		int pieceWidth = piece.getWidth();
		int pieceHeight = piece.getHeight();

		Coordinate lastCoord = new Coordinate(firstCoord.row + pieceHeight , firstCoord.col + pieceWidth);

		//Test first tile
		if(this.rowOutOfBounds(firstCoord.row) || this.colOutOfBounds(firstCoord.col) )
			return false;
		if(this.rowOutOfBounds(lastCoord.row) || this.colOutOfBounds(lastCoord.col))
			return false;
		else if(!this.canMoveToNewLocation(pieceIterator))
			return false;

		return true;
	}

	@Override
	public void handleDropPiece() {
		this.dropPiece();
		this.pieceHitGround();
	}

	/**
	 * Does all the necessaries when the piece hits the ground:
	 * Puts the piece on the board
	 * Removes rows and updates score
	 * Inits a new falling piece
	 */
	public void pieceHitGround() {
		this.putPieceOnBoard();
		this.removeRowsAndUpdateScore();
		this.createNewPiece();
	}

	/**
	 * Removes all full rows and updates gamescore accordingly
	 */
	public void removeRowsAndUpdateScore() {
		// Remove full rows and shift others
		int removedRows = this.board.removeRowsIfPossible();
		this.gameScore += Math.pow(removedRows, 2);
	}

	/**
	 * Updates the gameScreen if necessary
	 */
	public void updateGameScreen() {
		if(!this.canMoveToNewLocation(this.piece.iterator())) {
			this.gameScreen = GameScreen.GAME_OVER;
		}
	}

	/**
	 * Creates a new piede and sets the field variable to this
	 * Updates the pieceMade count
	 * Updates to game over using method
	 */
	public void createNewPiece() {
		this.piece = this.getNextPiece();
		this.piecesMade++;
		// If the newly created piece does not fit
		this.updateGameScreen();
	}

	/**
	 * Set the piece to a new generated piece
	 */
	public PositionedPiece getNextPiece() {
		return this.pieceFactory.getNextPositionedPiece();
	}

	/**
	 * Sets the piece tiles on the board
	 */
	public void putPieceOnBoard() {
		Iterator<CoordinateItem<Tile>> pieceIterator = this.piece.iterator();

		while(pieceIterator.hasNext()) {
			CoordinateItem<Tile> currentTile = pieceIterator.next();

			this.board.set(currentTile.coordinate, currentTile.item);
		}
	}

	/**
	 * Drop the falling piece to the bottom of the board or as far as it gets before colliding.
	 */
	public void dropPiece() {
		int row = this.getRows();
		int i = 1;
		while(this.moveFallingPiece(i, 0)) {
			if(row <= 0) {
				break;
			}
			row--;
		}
	}

	/**
	 * Checks if given row is within bounds of board
	 * @param row
	 * @return true if row is within, false otherwise
	 */
	private boolean rowOutOfBounds(int row) {
		if((row < 0) || (row > this.getRows()))
			return true;
		return false;
	}

	/**
	 * Checks if given column is within bounds of board
	 * @param col
	 * @return true if column is within, false otherwise
	 */
	private boolean colOutOfBounds(int col) {
		if((col < 0) || (col > this.getCols()))
			return true;
		return false;
	}

	@Override
	public GameScreen getGameScreen() {
		// TODO Auto-generated method stub
		return this.gameScreen;
	}

	@Override
	public int getFallTime() {
		// TODO Auto-generated method stub
		return (int) Math.pow(this.fallTime, Math.pow(0.998, this.piecesMade));
	}

	@Override
	public void clockTick() {
		// TODO Auto-generated method stub
		if(this.moveFallingPiece(1, 0)) return;
		this.pieceHitGround();
	}



}
