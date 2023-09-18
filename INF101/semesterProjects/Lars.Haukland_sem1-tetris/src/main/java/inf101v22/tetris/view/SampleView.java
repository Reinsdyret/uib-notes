package inf101v22.tetris.view;

import java.awt.Color;
import java.awt.Dimension;
import java.awt.Font;
import java.awt.Graphics;

import javax.swing.JComponent;

public class SampleView extends JComponent {
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

		// Painting on the canvas: the point (0, 0) is in the top-left corner.
		// Increasing x-value (first coordinate) means further to the right.
		// Increasing y-value means further down (i.e. OPPOSITE of regular math)

		// A small rectangle
		canvas.setColor(Color.BLACK);
		canvas.fillRect(20, 20, 100, 50); // x, y of top-left corner, then width, height

		// A small rectangle further down
		canvas.setColor(Color.RED);
		canvas.fillRect(20, 120, 100, 50);

		// A small rectangle further to the right
		canvas.setColor(Color.YELLOW);
		canvas.fillRect(220, 20, 100, 50);

		// A wider rectangle, also with a custom RGB color
		// See "hex" column at https://en.wikipedia.org/wiki/List_of_colors:_A%E2%80%93F
		// The color "Absolute Zero" defined by hex code #0048BA can be obtained by:
		int r = 0x00;
		int g = 0x48; // Writing 0x... means that the following integer is written in hexadecimal.
		int b = 0xBA;
		Color absoluteZero = new Color(r, g, b);
		canvas.setColor(absoluteZero);
		canvas.fillRect(20, 220, 200, 50);

		// A taller rectangle
		canvas.setColor(new Color(0xB0, 0xBF, 0x1A)); // Color: Acid green, #B0BF1A
		canvas.fillRect(20, 320, 100, 150);

		// Text placed on the same exact same coordinate as the "taller rectangle"
		// Surprised at where it shows up?
		canvas.setColor(Color.BLACK);
		Font myFont = new Font("SansSerif", Font.BOLD, 16);
		canvas.setFont(myFont);
		canvas.drawString("Hello Graphics", 20, 320);

		// Draw a chess board rectangle that resize together with the window/component size
		int componentWidth = this.getWidth();
		int componentHeight = this.getHeight();
		int chessX = 20;
		int chessY = 520;
		int chessWidth = componentWidth - 40;
		int chessHeight = componentHeight - 540;
		this.drawChess(canvas, chessX, chessY, chessWidth, chessHeight);

		// Using helper method from GraphicsHelperMethod to draw centered text
		// over the resizing chess board
		canvas.setColor(Color.GREEN);
		GraphicHelperMethods.drawCenteredString(
				canvas, "Centered text",
				20, 520, componentWidth - 40, componentHeight - 540);

		// A semi-transparent rectangle
		Color transparentRed = new Color(0xff, 0, 0, 0x80); // fourth argument is transparency - try out different values 0x00 - 0xff
		canvas.setColor(transparentRed);
		canvas.fillRect(220, 500, 80, 40);
	}

	private void drawChess(Graphics g, int x, int y, int width, int height) {
		// For testing purposes; the red rectangle should not be visible,
		// or else we haven't painted the full area we're responsible for
		g.setColor(Color.RED);
		g.fillRect(x, y, width, height);

		// Draw the actual chess board
		for (int row = 0; row < 8; row++) {
			for (int col = 0; col < 8; col++) {
				// Here we paint the chess tile at location (row, col)
				// Pick the color
				if (((row + col) % 2) == 0) {
					g.setColor(Color.BLACK);
				}
				else {
					g.setColor(Color.WHITE);
				}

				// Calculate the coordinates of the tile and paint it
				int tileX = x + ((row * width) / 8);
				int tileY = y + ((col * height) / 8);
				int nextTileX = x + (((row + 1) * width) / 8);
				int nextTileY = y + (((col + 1) * height) / 8);
				int tileWidth = nextTileX - tileX;
				int tileHeight = nextTileY - tileY;
				g.fillRect(tileX, tileY, tileWidth, tileHeight);
			}
		}
	}

	@Override
	public Dimension getPreferredSize() {
		// This method returns the "preferred" size of the component. However, if
		// the user resize the window, the values returned here will not matter.
		// Hence, do not use the return value from here to calculate the size of
		// your components; use this.getWidht() and this.getHeight() instead.
		int width = 500;
		int height = 750;
		return new Dimension(width, height);
	}
}
