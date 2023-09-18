package inf101v22.tetris.view;

import java.awt.Font;
import java.awt.FontMetrics;
import java.awt.Graphics;

public class GraphicHelperMethods {

	/**
	 * Find the width of string s in the given font if painted on canvas
	 * @param canvas the canvas to be painted
	 * @param f the font
	 * @param s the string to be painted
	 * @return the width of the string in pixels
	 */
	public static int getStringWidth(Graphics canvas, Font f, String s) {
		FontMetrics fm = canvas.getFontMetrics(f);
		java.awt.geom.Rectangle2D rect = fm.getStringBounds(s, canvas);
		return (int) Math.round(rect.getWidth());
	}

	/**
	 * Find the height of string s in the given font if painted on canvas
	 * @param canvas the canvas to be painted
	 * @param f the font
	 * @param s the string to be painted
	 * @return the height of the string in pixels
	 */
	public static int getStringHeight(Graphics canvas, Font f, String s) {
		FontMetrics fm = canvas.getFontMetrics(f);
		java.awt.geom.Rectangle2D rect = fm.getStringBounds(s, canvas);
		return (int) Math.round(rect.getHeight());
	}

	/**
	 * The font ascent is the distance from the font's baseline to the top
	 * of most alphanumeric characters.
	 * @param canvas the canvas to be pained
	 * @param f the font
	 * @return the ascent
	 */
	public static int getStringAscent(Graphics canvas, Font f) {
		// Find the size of string s in the font of the Graphics context "page"
		FontMetrics fm = canvas.getFontMetrics(f);
		return fm.getAscent();
	}

	/**
	 * Draw a text string centered in the rectangle given by left, top, width, height.
	 * The text may overflow outside the bounds, so if width and height are 0, then the
	 * text will be centered around (left, top)
	 * @param canvas the graphics canvas on which to paint
	 * @param s the string to print
	 * @param left left coordinate
	 * @param top top coordinate
	 * @param width with of box
	 * @param height height of box
	 */
	public static void drawCenteredString(Graphics canvas, String s, int left, int top, int width, int height) {
		Font font = canvas.getFont();
		int textWidth = getStringWidth(canvas, font, s);
		int textHeight = getStringHeight(canvas, font, s);
		int textAscent = getStringAscent(canvas, font);
		// Center text horizontally and vertically within provided rectangular bounds
		int textX = left + ((width - textWidth) / 2);
		int textY = top + ((height - textHeight) / 2) + textAscent;
		canvas.drawString(s, textX, textY);
	}
}
