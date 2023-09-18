package inf101v22.tetris.view;

import java.awt.Color;
import java.awt.Dimension;
import java.awt.Font;
import java.awt.Graphics;
import java.util.Iterator;

import javax.swing.JComponent;

import inf101v22.grid.CoordinateItem;
import inf101v22.tetris.model.GameScreen;
import inf101v22.tetris.model.Tile;

public class TetrisView extends JComponent{
	/**
	 *
	 */
	private static final long serialVersionUID = 1L;

	{
		// This code (between curly braces) is executed when an object is
		// created (before the call to the constructor, if one exists).

		// The call to setFocusable enables the panel to receive events from
		// the user, such as key-presses and mouse movements.
		this.setFocusable(true);
	}
	int margin = 50;
	int tilePadding = 10;
	TetrisViewable viewable;

	public TetrisView(TetrisViewable viewable) {
		this.viewable = viewable;
	}


	// The paint method is called by the Java Swing framework every time either
	// -- the window opens or resizes, or
	// -- someone calls .repaint() on this object (note: do NOT call paint
	// directly), or
	// -- for some other reason Java Swing believes it is time for painting the
	// canvas.
	@Override
	public void paint(Graphics canvas) {
		// This will call the paint -method of the JComponent class
		// It is often a good practice to call the super method when
		// overriding an existing method from an inherited class --
		// it might be doing something important.
		super.paint(canvas);


		int windowWidth = this.getWidth();
		int windowHeight = this.getHeight();

		if(this.viewable.getGameScreen() == GameScreen.WELCOME_SCREEN) {
			this.drawWelcomeScreen(canvas);
		}

		// If the game has begun, or if it is over. Draw the tetris board and pieces, nice in the backGround of game over
		if((this.viewable.getGameScreen() == GameScreen.ACTIVE_GAME) ||  (this.viewable.getGameScreen() == GameScreen.GAME_OVER)) {
			this.drawTetrisBoard(canvas, this.margin, this.margin, windowWidth - (2*this.margin), windowHeight- (2*this.margin), this.tilePadding);

			this.drawFallingPiece(canvas, this.margin, this.margin, windowWidth- (2*this.margin), windowHeight-(2* this.margin), this.tilePadding);

			this.drawScore(canvas);
		}

		if(this.viewable.getGameScreen() == GameScreen.GAME_OVER) {
			this.drawGameOver(canvas);
		}

	}

	private void drawWelcomeScreen(Graphics canvas) {
		int windowWidth = this.getWidth();
		int windowHeight = this.getHeight();

		Color color = new Color(0, 0, 0, 128);
		canvas.setColor(color);
		canvas.fillRect(0, 0 , windowWidth, windowHeight);
		Color newColor = Color.green;
		int textX = windowWidth / 4;
		int textY = windowHeight / 8;
		int textHeight = textY * 2;
		int textWidth = textX * 2;
		this.drawString( canvas, "Welcome to Tetris!", textX, textY, textWidth, textHeight, newColor);
		this.drawString(canvas, "Press enter to begin!", textX, textY*2, textWidth, textHeight, newColor);
	}

	private void drawGameOver(Graphics canvas) {
		int windowWidth = this.getWidth();
		int windowHeight = this.getHeight();
		Color color = new Color(0, 0, 0, 128);
		canvas.setColor(color);
		canvas.fillRect(0, 0 , windowWidth, windowHeight);

		Color newColor = Color.green;
		int textHeight = 100;
		this.drawString( canvas, "Game Over!", this.margin, windowHeight/2, windowWidth, textHeight, newColor);
	}

	private void drawString(Graphics canvas, String string, int left, int top, int width, int height, Color color) {
		canvas.setColor(color);
		Font myFont = new Font("SansSerif", Font.BOLD, 42);
		canvas.setFont(myFont);
		GraphicHelperMethods.drawCenteredString(canvas, string, left, top, width, height);
	}

	@Override
	public Dimension getPreferredSize() {
		// This method returns the "preferred" size of the component. However, if
		// the user resize the window, the values returned here will not matter.
		// Hence, do not use the return value from here to calculate the size of
		// your components; use this.getWidht() and this.getHeight() instead
		int tileSideLength = 50;
		int rows = this.viewable.getRows();
		int cols = this.viewable.getCols();
		int width = ((tileSideLength + this.tilePadding) * cols) + this.tilePadding ;
		int height = ((tileSideLength + this.tilePadding) * rows) + this.tilePadding;
		return new Dimension(width, height);
	}

	/**
	 * Draws the tile with given parameters
	 * @param canvas the graphics object to draw on
	 * @param x the x value of top right corner of tile
	 * @param y the y value of top right corner of the tile
	 * @param width the width of the tile including padding
	 * @param height the height of the tile including padding
	 * @param padding the padding for the under and right side of tile
	 * @param color the color of the tile
	 */
	private void drawTileWithRightBottomPadding(Graphics canvas, int x, int y, int width, int height, int padding, Color color) {
		canvas.setColor(color);
		canvas.fillRect(x, y, width - padding, height - padding);
	}

	/**
	 * Goes through the board and draws each tile with color black
	 * @param canvas Graphics object
	 * @param boardX x position of the top right of the board
	 * @param boardY y position of the top right of the board
	 * @param boardWidth width of the board
	 * @param boardHeight height of the board
	 * @param tilePadding padding of each tile
	 */
	private void drawBoardWithRightBottomPadding(Graphics canvas, int boardX, int boardY, int boardWidth, int boardHeight, int tilePadding) {

		Color color;

		int rows = this.viewable.getRows();
		int cols = this.viewable.getCols();

		// Assumes that the calculation becomes an integer
		int tileWidth = boardWidth / cols;
		int tileHeight = boardHeight / rows;

		Iterable<CoordinateItem<Tile>> tiles = this.viewable.Iterable();

		for(CoordinateItem<Tile> coordinateItem : tiles) {
			int row = coordinateItem.coordinate.row;
			int col = coordinateItem.coordinate.col;
			int x = boardX + (col * tileWidth);
			int y = boardY + (row * tileHeight);

			if(coordinateItem.item != null) {
				color = coordinateItem.item.color;
			}else {
				color = Color.black;
			}

			this.drawTileWithRightBottomPadding(canvas, x, y, tileWidth, tileHeight, tilePadding, color);
		}
	}

	private void drawTetrisBoard(Graphics canvas, int boardX, int boardY, int boardWidth, int boardHeight, int tilePadding) {
		this.drawBoardWithRightBottomPadding(canvas, boardX, boardY, boardWidth, boardHeight, tilePadding);
	}

	private void drawFallingPiece(Graphics canvas, int boardX, int boardY, int boardWidth, int boardHeight, int tilePadding) {
		int rows = this.viewable.getRows();
		int cols = this.viewable.getCols();

		// Assumes that the calculation becomes an integer
		int tileWidth = boardWidth / cols;
		int tileHeight = boardHeight / rows;

		Iterator<CoordinateItem<Tile>> pieceTiles = this.viewable.getPiece();

		while(pieceTiles.hasNext()) {
			CoordinateItem<Tile> currentTile = pieceTiles.next();

			int row = currentTile.coordinate.row;
			int col = currentTile.coordinate.col;
			int x = boardX + (col * tileWidth);
			int y = boardY + (row * tileHeight);
			Color color = currentTile.item.color;

			this.drawTileWithRightBottomPadding(canvas, x, y, tileWidth, tileHeight, tilePadding, color);
		}
	}

	private void drawScore(Graphics canvas) {
		String scoreString = "Score: " + Integer.toString(this.viewable.getScore());
		int heigth = this.margin;
		int left = 0;
		int top = 0;
		Color scoreColor = Color.MAGENTA;
		this.drawString(canvas, scoreString, left, top, this.getWidth(), heigth, scoreColor);
	}
}
