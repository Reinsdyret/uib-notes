package inf101.sem2.GUI;

import java.awt.Color;
import java.awt.Dimension;
import java.awt.GridLayout;
import java.awt.event.MouseAdapter;
import java.awt.event.MouseEvent;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;

import javax.swing.JPanel;

import inf101.grid.Grid;
import inf101.grid.Location;
import inf101.sem2.game.GameBoard;
import inf101.sem2.player.Player;

/**
 * This class is a grid of Game panels.
 * This is also a MouseListener for all those GamePanels
 * so whenever any of the panels in the grid is clicked
 * a method in this class is envoked.
 * <p>
 * This click will be translated into a Location and a
 * move will be sent to the game.
 *
 * @author Anna Eilertsen - anna.eilertsen@uib.no
 * @author Martin Vatshelle - martin.vatshelle@uib.no
 */
public class ClickableGrid<T> extends JPanel {

	private static final long serialVersionUID = 8755882090377973497L;
	private MouseAdapter adapter;
	private Grid<GamePanel> clickablePanels; // clickable grid for user input
	private GameBoard board;
	/**
	 * Location of panel most recently clicked
	 */
	private Location clickedPanel = null;
	/**
	 * Locations of panels which have been selected
	 */
	private List<Location> selectedPanels;

	/**
	 * Boolean used to confirm double click of a panel to
	 * deselect all panels
	 */
	private boolean confirmMove;

	// HashMap is something we will learn about in INF102
	HashMap<T, Color> colorMap;

	public ClickableGrid(GameBoard board, Iterable<T> players, List<Color> colors) {
		this.board = board;
		adapter = new ClickableGridListener();

		// assign colors to the players
		setColors(players, colors);

		// create clickable panels
		int rows = board.numRows();
		int cols = board.numColumns();
		clickablePanels = new Grid<>(rows, cols);
		selectedPanels = new ArrayList<>();
		setLayout(new GridLayout(rows, cols));
		makeClickablePanels();

		// set gui options
		setPreferredSize(new Dimension(100 * board.numRows(), board.numColumns()));
		setMinimumSize(new Dimension(100 * board.numRows(), board.numColumns()));
		setRequestFocusEnabled(true);
		requestFocus();
		setVisible(true);
		validate();
	}

	/**
	 * Initializes a Map from Player to Color.
	 * This is later used color the grid of GamePanel to match
	 * the moves of the different players.
	 */
	private void setColors(Iterable<T> pieces, List<Color> colors) {
		colorMap = new HashMap<>();
		for (T p : pieces) {
			colorMap.put(p, colors.get(colorMap.size()));
		}
		// color.put(null, Color.WHITE);
	}

	/**
	 * Should be called after a click to update the UI to reflect the current game
	 * state
	 */
	public void updateGui() {
		for (Location loc : board.locations()) {
			Color color = getColor(board.get(loc));
			clickablePanels.get(loc).setColor(color);
		}
		validate();
		repaint();
	}

	/**
	 * Finds the color to use on those locations where the given
	 * player has placed.
	 *
	 * @param player
	 */
	protected Color getColor(Player player) {
		if (player == null) {
			return null;
		}
		return colorMap.get(player);
	}

	/**
	 * Makes a grid of same size as the one in game and fills it with clickable
	 * panels
	 */
	private void makeClickablePanels() {
		for (Location loc : board.locations()) {
			GamePanel pan = new GamePanel(adapter);
			clickablePanels.set(loc, pan);
			add(pan);
		}
		updateGui();
	}

	public Location getLastClick() {
		return clickedPanel;
	}

	public void clearLastClick() {
		clickedPanel = null;
	}

	public List<Location> getSelectedPanels() {
		return selectedPanels;
	}

	/**
	 * Sets <code>panel</code> as selected in GamePanel class and adds
	 * <code>panel</code> to list of selected panels
	 */
	public void setSelected(GamePanel panel) {
		if (panel == null)
			throw new NullPointerException("GamePanel is null.");
		if (!clickablePanels.contains(panel))
			throw new IllegalArgumentException("GamePanel is not part of the grid.");

		panel.setSelected(true);
		Location panelLocation = clickablePanels.locationOf(panel);
		selectedPanels.add(panelLocation);
	}

	/**
	 * Iterate through all selected panels and deselect.
	 * Clear list of selected panels.
	 */
	public void deselectPanels() {
		for (Location loc : selectedPanels) {
			clickablePanels.get(loc).setSelected(false);
		}
		selectedPanels.clear();
	}

	class ClickableGridListener extends MouseAdapter {

		// This is what happens when the mouse clicks on one of the squares of the grid.
		@Override
		public void mousePressed(MouseEvent me) {
			if (clickablePanels.contains(me.getSource())) {
				try {
					GamePanel currentPanel = (GamePanel) me.getSource();
					Location currentLocation = clickablePanels.locationOf(currentPanel);
					// Check if panel has been previously selected
					if (selectedPanels.contains(currentLocation)) {
						confirmMove = true;
					}
					setSelected(currentPanel);

					if (confirmMove) {
						deselectPanels();
						confirmMove = false;
					}
					updateGui();
				} catch (Exception e) {
					System.err.println(e.getMessage());
				}
			} else {
				System.err.println("Clicked on wrong thing: " + me.getSource());
			}
		}
	}

}
