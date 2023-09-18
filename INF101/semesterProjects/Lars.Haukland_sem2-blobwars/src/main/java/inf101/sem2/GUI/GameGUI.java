package inf101.sem2.GUI;

import java.awt.BorderLayout;
import java.awt.Color;
import java.awt.Dimension;
import java.awt.event.ActionEvent;
import java.awt.event.ActionListener;
import java.awt.event.WindowEvent;
import java.awt.event.WindowAdapter;

import java.util.Arrays;
import java.util.List;

import javax.swing.JButton;
import javax.swing.JFrame;
import javax.swing.JLabel;
import javax.swing.JPanel;

import inf101.grid.Location;
import inf101.grid.Move;
import inf101.sem2.game.GameBoard;
import inf101.sem2.player.GameEndedException;
import inf101.sem2.player.Player;
import inf101.sem2.player.RestartException;

/**
 * This class combines two buttons with a Clickable grid in one JFrame
 * The buttons are used to end or restart the game while the grid is used to
 * display the state of the game and for the user to select next move.
 *
 * @author Martin Vatshelle - martin.vatshelle@uib.no
 */
public class GameGUI extends Input implements ActionListener, Graphics {

	private JButton backButton; // Button to go back to end the game and go back to main menu
	private JButton restartButton; // Button to restart the game
	private JLabel statusMessage; // field for displaying message to user
	private ClickableGrid<Player> board; // clickable grid for user input
	private Iterable<Player> players;
	private JFrame frame;

	public boolean wantRestart = false;
	public boolean ended = false;

	public GameGUI() {
		super("Game GUI");
		JPanel buttons = createButtonPanel();
		statusMessage = new JLabel();
		statusMessage.setPreferredSize(new Dimension(300, 50));
		statusMessage.setText("Welcome!");

		// make new main window for the game
		frame = new JFrame();
		frame.addWindowListener(new WindowAdapter() {
			@Override
			public void windowClosing(WindowEvent e) {
				frame.setVisible(false);
				ended = true;
			}
		});
		frame.add(statusMessage, BorderLayout.NORTH);
		frame.add(buttons, BorderLayout.SOUTH);
		drawGameBoard();
	}

	public void setName(String name) {
		frame.setTitle(name);
	}

	/**
	 * Draws the gameBoard on the JFrame
	 * and refreshes the rest of the graphics
	 */
	private void drawGameBoard() {
		// add the clickable grid panel
		frame.doLayout();
		frame.pack();
		frame.setVisible(true);
		frame.validate();
	}

	/**
	 * Helper method that creates the button panel
	 */
	private JPanel createButtonPanel() {
		backButton = new JButton();
		backButton.addActionListener(this);
		backButton.setText("Back");

		restartButton = new JButton();
		restartButton.addActionListener(this);
		restartButton.setText("Restart");

		JPanel buttons = new JPanel();
		buttons.add(backButton);
		buttons.add(restartButton);
		return buttons;
	}

	// Called whenever a button is pressed
	@Override
	public void actionPerformed(ActionEvent e) {
		if (e.getSource() == backButton) {
			ended = true;
			System.err.println("GameGui has ended game");
			frame.dispatchEvent(new WindowEvent(frame, WindowEvent.WINDOW_CLOSING));
			return;
		}
		if (e.getSource() == restartButton) {
			wantRestart = true;
			return;
		}
		drawGameBoard();
	}

	public void removeClickablePanels() {
		if (board != null && board.getParent() != null) {
			board.getParent().remove(board);
		}
	}

	@Override
	public void displayMessage(String message) {
		statusMessage.setText(message);
	}

	@Override
	public void display(GameBoard board) {
		// remake the game board in case the number of rows or columns has changed
		removeClickablePanels();
		this.board = new ClickableGrid<Player>(board, players, getColors());
		frame.add("Center", this.board);
		frame.setMinimumSize(new Dimension(100 * board.numColumns(), 100 * board.numRows() + 100));
		drawGameBoard();
	}

	/**
	 * Waits for 1 game panel to be selected and then return its location.
	 * 
	 * @return location of selected panel
	 */
	@Override
	public Location getLocation() {
		List<Location> selectedPanels = getSelectedPanels(1);
		Location loc = selectedPanels.get(0);
		return loc;
	}
	
	/**
	 * Waits for 2 game panels to be selected and then returns their move
	 * @return Move of the selected panels
	 */
	public Move getMove() {
		List<Location> selectedPanels = getSelectedPanels(2);
		Location loc1 = selectedPanels.get(0);
		Location loc2 = selectedPanels.get(1);
		
		return new Move(loc1,loc2);
	}

	/**
	 * Wait until <code>numPanels</code> panels have been selected and return
	 * list of the locations of the panels.
	 * 
	 * @return list of locations of selected panels
	 */
	private List<Location> getSelectedPanels(int numPanels) {
		while (board.getSelectedPanels().size() < numPanels) {
			sleep(100);
			if (wantRestart) {
				throw new RestartException();
			}

			if (ended) {
				throw new GameEndedException();
			}
		}
		return board.getSelectedPanels();
	}

	/**
	 * Maps from Piece values to colors
	 *
	 * @param pieceAt The piece to be drawn
	 * @return The color that this GUI implementation associates with the provided
	 *         piece
	 */
	protected List<Color> getColors() {
		Color[] colors = { Color.BLUE, Color.RED };
		return Arrays.asList(colors);
	}

	@Override
	public Integer getColumn() {
		return getLocation().col;
	}

	public void setPlayers(Iterable<Player> players) {
		this.players = players;
	}

	@Override
	public void sleep(int delay) {
		try {
			Thread.sleep(delay);
		} catch (InterruptedException e) {
			e.printStackTrace();
		}
	}

}
