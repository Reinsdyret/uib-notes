package inf101.sem2.GUI;

import inf101.grid.Location;
import inf101.grid.Move;
import inf101.sem2.game.Game;

/**
 * Abstract class to handle input type based on game type.
 * The type parameter in Game dictates the type of moves the 
 * game uses. In TicTacToe one places a piece on the board, so one move
 * is selecting one location on the board. In TicTacToe the type parameter given to Game
 * is <code>Location</code>. Hence the user input (either GUI or Console) should return
 * a location. 
 * In Connect4 one places a piece in a column and the piece drops from the top (row=0),
 * so one move is selecting one column (integer) on the board. The type parameter
 * given to Game is <code>Integer</code>. Hence the user input (GUI or Console) should
 * return an integer.
 */
public abstract class Input {

	/**
	 * Input type name
	 */
	String name;
	protected Input(String name){
		this.name = name;
	}
	
	public Location getLocation() {
		throw new UnsupportedOperationException(this.name+" does not support Location move");
	}

	public Integer getColumn() {
		throw new UnsupportedOperationException(this.name+" does not support Column move");
	}
	
	public Move getMove() {
		throw new UnsupportedOperationException(this.name+" does not support Move move");
	}
	
	public <T> T getMove(Game<T> game) {
		for(T move : game.getPossibleMoves()) {
			// If move type in Game is Location, return a location
			if(move instanceof Location) {
				return (T) getLocation();
			}
			// If move type in Game is Integer, return an integer
			if(move instanceof Integer){
				return (T) getColumn();
			}
			
			if(move instanceof Move) {
				return (T) getMove();
			}
			throw new UnsupportedOperationException(this.name+" does not support this type of move");
		}
		//if no possible moves, pass
		return null;
	}
}
