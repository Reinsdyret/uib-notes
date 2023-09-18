package labyrinth.gui;

import java.awt.BorderLayout;
import java.awt.event.ActionEvent;
import java.awt.event.ActionListener;
import java.awt.event.KeyEvent;
import java.awt.event.KeyListener;
import java.util.function.Supplier;

import javax.swing.JButton;
import javax.swing.JFrame;
import javax.swing.JLabel;
import javax.swing.JPanel;

import datastructure.GridDirection;
import labyrinth.ILabyrinth;
import labyrinth.MovePlayerException;

/**
 * The application in which labyrinthComponent is displayed.
 * 
 * @author eivind
 */
public class LabyrinthGUI extends JPanel implements ActionListener, KeyListener {

	private static final long serialVersionUID = 8755882090377973497L;

	/**
	 * Initializes a JFrame in which we place the a CellAutomataGUI containing
	 * the given ILabyrinth.
	 * 
	 * @param ca
	 */
	public static void run(Supplier<ILabyrinth> labyrinthMaker) {
		JFrame f = new JFrame("Inf101 labyrinth");
		f.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
		LabyrinthGUI ap = new LabyrinthGUI(labyrinthMaker);
		ap.initialize();
		f.add("Center", ap);
		f.pack();
		f.setVisible(true);
	}

	LabyrinthComponent labyrinthComponent;
	ILabyrinth labyrinth;

	private javax.swing.Timer timer;
	private JButton leftButton;
	private JButton upButton;
	private JButton downButton;

	private JButton rightButton;

	private JButton setBoardButton;

	private JLabel message;

	private final Supplier<ILabyrinth> labyrinthMaker;

	public LabyrinthGUI(Supplier<ILabyrinth> labyrinthMaker) {
		this.labyrinthMaker = labyrinthMaker;
		this.labyrinth = labyrinthMaker.get();
	}

	/*
	 * (non-Javadoc)
	 *
	 * @see
	 * java.awt.event.ActionListener#actionPerformed(java.awt.event.ActionEvent)
	 *
	 * Called whenever a button is pressed or the timer tells us its time to
	 * make a step.
	 */
	@Override
	public void actionPerformed(ActionEvent e) {
		try {
			if (e.getSource() == timer) {
				timer.restart();
				labyrinthComponent.repaint();
				return;
			} else if (e.getSource() == leftButton) {
				labyrinth.movePlayer(GridDirection.WEST);
				labyrinthComponent.repaint();
			} else if (e.getSource() == rightButton) {
				labyrinth.movePlayer(GridDirection.EAST);
				labyrinthComponent.repaint();
			} else if (e.getSource() == upButton) {
				labyrinth.movePlayer(GridDirection.NORTH);
				labyrinthComponent.repaint();
			} else if (e.getSource() == downButton) {
				labyrinth.movePlayer(GridDirection.SOUTH);
				labyrinthComponent.repaint();
			} else if (e.getSource() == setBoardButton) {
				labyrinth = labyrinthMaker.get();
				labyrinthComponent.setLabyrinth(labyrinth);
				labyrinthComponent.repaint();
			}
			updateMessage();
		} catch (MovePlayerException ex) {
			message.setText("Bad");
		}
	}

	/**
	 * Sets up the GUI.
	 */
	public void initialize() {
		setLayout(new BorderLayout());

		// labyrinth.clear();
		labyrinthComponent = new LabyrinthComponent(labyrinth);

		JPanel p = new JPanel();

		leftButton = new JButton();
		leftButton.addActionListener(this);
		leftButton.setText("←");
		leftButton.addKeyListener(this);

		upButton = new JButton();
		upButton.addActionListener(this);
		upButton.setText("↑");
		upButton.addKeyListener(this);

		downButton = new JButton();
		downButton.addActionListener(this);
		downButton.setText("↓");
		downButton.addKeyListener(this);

		rightButton = new JButton();
		rightButton.addActionListener(this);
		rightButton.setText("→");
		rightButton.addKeyListener(this);

		setBoardButton = new JButton();
		setBoardButton.addActionListener(this);
		setBoardButton.setText("Reset Board");
		setBoardButton.addKeyListener(this);

		message = new JLabel();
		message.setText("Foo!");

		addKeyListener(this);

		p.add(leftButton);
		p.add(rightButton);
		p.add(upButton);
		p.add(downButton);
		p.add(setBoardButton);
		add(p, BorderLayout.NORTH);
		add(labyrinthComponent, BorderLayout.CENTER);
		add(message, BorderLayout.SOUTH);
		timer = new javax.swing.Timer(1000 / 20, this);
	}

	@Override
	public void keyPressed(KeyEvent e) {
		int key = e.getKeyCode();
		try {
			if (key == KeyEvent.VK_LEFT) {
				labyrinth.movePlayer(GridDirection.WEST);
				labyrinthComponent.repaint();
			} else if (key == KeyEvent.VK_RIGHT) {
				labyrinth.movePlayer(GridDirection.EAST);
				labyrinthComponent.repaint();
			} else if (key == KeyEvent.VK_UP) {
				labyrinth.movePlayer(GridDirection.NORTH);
				labyrinthComponent.repaint();
			} else if (key == KeyEvent.VK_DOWN) {
				labyrinth.movePlayer(GridDirection.SOUTH);
				labyrinthComponent.repaint();
			}
			updateMessage();
		} catch (MovePlayerException ex) {
			message.setText(ex.getMessage());
		}
	}

	@Override
	public void keyReleased(KeyEvent arg0) {
		// TODO Auto-generated method stub

	}

	@Override
	public void keyTyped(KeyEvent arg0) {
		// TODO Auto-generated method stub

	}

	private void updateMessage() {
		if (labyrinth.isPlaying()) {
			message.setText("OK | Gold: " + labyrinth.getPlayerGold() + " HP: " + labyrinth.getPlayerHitPoints());
		} else {
			message.setText(
					"GAME OVER | Gold: " + labyrinth.getPlayerGold() + " HP: " + labyrinth.getPlayerHitPoints());
		}
	}
}
