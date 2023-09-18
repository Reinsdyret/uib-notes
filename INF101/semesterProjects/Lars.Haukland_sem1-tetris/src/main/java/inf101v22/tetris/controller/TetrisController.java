package inf101v22.tetris.controller;

import java.awt.event.ActionEvent;
import java.awt.event.ActionListener;
import java.awt.event.KeyEvent;
import java.awt.event.KeyListener;

import javax.swing.Timer;

import inf101v22.tetris.midi.TetrisSong;
import inf101v22.tetris.model.GameScreen;
import inf101v22.tetris.view.TetrisView;


public class TetrisController implements KeyListener, ActionListener{
	TetrisControllable model;
	TetrisView view;
	Timer timer;
	TetrisSong song = new TetrisSong();

	
	/**
	 * Create a new TetrisController object
	 * Inits a timer and keylistener, both throwing events back to this object
	 * @param model The tetrismodel, a TetrisControllable
	 * @param view The tetrisview
	 */
	public TetrisController(TetrisControllable model, TetrisView view) {
		this.model = model;
		this.view = view;
		this.timer = new Timer(model.getFallTime(), this);
		view.addKeyListener(this);
		this.timer.start();
		this.song.run();
	}

	@Override
	public void keyPressed(KeyEvent e) {
		// No keypress if gameOver
		if(this.model.getGameScreen() == GameScreen.GAME_OVER) return;

		// If in welcome screen
		if(this.model.getGameScreen() == GameScreen.WELCOME_SCREEN) {
			// If enter is pressed
			if(e.getKeyCode() == KeyEvent.VK_ENTER) {
				// Start game
				this.model.setActiveGame();
			}
			return;
		}
		// TODO Auto-generated method stub
		if (e.getKeyCode() == KeyEvent.VK_LEFT) {
			// Left arrow was pressed
			this.model.moveFallingPiece(0, -1);
		}
		else if (e.getKeyCode() == KeyEvent.VK_RIGHT) {
			// Right arrow was pressed
			this.model.moveFallingPiece(0, 1);
		}
		else if (e.getKeyCode() == KeyEvent.VK_DOWN) {
			if(this.model.moveFallingPiece(1, 0)) this.timer.restart();
		}
		else if (e.getKeyCode() == KeyEvent.VK_UP) {
			// Up arrow was pressed
			this.model.rotatePiece();
		}
		else if (e.getKeyCode() == KeyEvent.VK_SPACE) {
			// Spacebar was pressed
			this.model.handleDropPiece();
			this.timer.restart();
		}
		this.view.repaint();


	}

	private void setTimeDelays() {
		int fallTime = this.model.getFallTime();
		this.timer.setDelay(fallTime);
		this.timer.setInitialDelay(fallTime);
	}

	@Override
	public void keyReleased(KeyEvent arg0) {
		// TODO Auto-generated method stub

	}

	@Override
	public void keyTyped(KeyEvent arg0) {
		// TODO Auto-generated method stub

	}

	@Override
	public void actionPerformed(ActionEvent e) {
		// TODO Auto-generated method stub
		if(this.model.getGameScreen() == GameScreen.ACTIVE_GAME) {
			this.model.clockTick();
			this.view.repaint();
			this.setTimeDelays();
		}

	}


}
