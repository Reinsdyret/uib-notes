package inf101.sem2.game.games;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

import inf101.grid.Location;
import inf101.grid.Move;
import inf101.sem2.GUI.Graphics;
import inf101.sem2.game.Game;
import inf101.sem2.game.GameBoard;
import inf101.sem2.player.Player;

public class BlobWars extends Game<Move>{
	
	/**
	 * Constructs a BlobWars game with 2 players where p1 starts the game.
	 * 
	 * @param graphics - The graphics used to display the game for the human players
	 * @param p1       - First player
	 * @param p2       - Second player
	 */
	public BlobWars(Graphics graphics, Player p1, Player p2) {
		super(new GameBoard(8, 8), graphics);
		addPlayer(p1);
		addPlayer(p2);
		initializeBoard();
	}

	public BlobWars(Graphics graphics, Iterable<Player> players) {
		// Thanks to Ljubomir Simic who helped me debug these constructors and tipped me about only using two constructors
		super(new GameBoard(8,8), graphics, players);
		initializeBoard();
	}

	private void initializeBoard() {
		this.board.swap(new Location(0,0), getCurrentPlayer());
		players.nextPlayer();
		this.board.swap(new Location(this.board.numRows()-1, this.board.numColumns() - 1), getCurrentPlayer());
		players.nextPlayer();
	}

	@Override
	public Game<Move> copy() {
		BlobWars copy = new BlobWars(this.graphics, this.players);
		this.copyTo(copy);
		return copy;
	}

	@Override
	public void makeMove(Move move) {
		// TODO Auto-generated method stub
		// If the move is not valid then it should not be excecuted
		if (!validMove(move))
            throw new IllegalArgumentException(move.toString() + " Is not valid");
		
		// Delete start blob if necessary
		if(move.moveType) {
			this.board.swap(move.start, null);
		}
		
		// Loop through neighboring cells of the end move and change if occupied
		for(Location loc : move.end.allNeighbors()) {
			
			// Skip cell if empty or if it is not on the board
			if(!this.board.isOnBoard(loc) || this.board.get(loc) == null ) continue;
			
			// Change cell into player that made move
			this.board.swap(loc, this.getCurrentPlayer()); 
		}
		
		// Place the end blob
		board.swap(move.end, this.getCurrentPlayer());
		
	}
	
	@Override
	public boolean validMove(Move move) {
		// TODO Auto-generated method stub
		if(hasCurrentPlayer(move.start) && this.board.canPlace(move.end)) {
			return move.isValid();
		}
		return false;
	}

	@Override
	public boolean isWinner(Player player) {
		// TODO Auto-generated method stub
		// If current player has no pieces left it is not the winner
		if(this.noCellsLeft(player)) return false;
		
		// If the board is full check if the player has most pieces
		if(this.board.isFull()) {
			return winnerByPieces(player);
		}
		
		// Last option of winning is if the other(s) have no pieces left
		return winnerByEliminating(player);
	}
	
	@Override
	public boolean gameOver() {
		return noCellsLeft(this.getCurrentPlayer()) || this.board.isFull();
	}
	
	@Override
	public void restart() {
		board.clear();
		players.restart();
		graphics.display(board);
		graphics.displayMessage("Welcome!");
		this.initializeBoard();
	}
	
	/**
	 * Checks if given player has won because the other player(s) has no pieces left
	 * @param player
	 * @return True if player is winner, false otherwise
	 */
	private boolean winnerByEliminating(Player player) {
		for(Player enemyPlayer : this.players) {
			// Skip player if it is the one we are checking
			if(enemyPlayer.equals(player)) continue;
			
			if(this.board.countPieces(enemyPlayer) != 0) return false;
		}
		return true;
	}
	
	/**
	 * Checks if given player has won when the board if full and has the most pieces
	 * @param player
	 * @return True if player is winner, false otherwise
	 */
	private boolean winnerByPieces(Player player) {
		// Variable to keep track of the most pieces an enemy player has
		int mostPieces = 0;
		
		for(Player enemyPlayer : this.players) {
			// Skip player if it is the one we are checking
			if(enemyPlayer.equals(player)) continue;
			
			if(this.board.countPieces(enemyPlayer) > mostPieces) mostPieces = this.board.countPieces(enemyPlayer);
		}
		
		return this.board.countPieces(player) > mostPieces;
	}

	/** 
	 * Checks if the given player has no pieces left
	 * @param player
	 * @return True if there is no pieces left, False otherwise
	 */
	private boolean noCellsLeft(Player player) {
		return this.board.countPieces(player) == 0;
	}

	@Override
	public List<Move> getPossibleMoves() {
		// TODO Auto-generated method stub
		// List of moves
		List<Move> possibleMoves = new ArrayList<>();
		
		// Loop through the board
		for(Location loc : board.locations()) {
			// Skip cell if it is not the current players
			if(!this.hasCurrentPlayer(loc)) continue;
			
			// Loop through all the possible moves
			for(int row = loc.row - 2; row <= loc.row + 2; row++) {
				for(int col = loc.col - 2; col <= loc.col + 2; col++) {
					Location newLoc = new Location(row, col);
					// If a piece cannot be placed then we skip
					if(!this.board.canPlace(newLoc)) continue;
					
					possibleMoves.add(new Move(loc,newLoc));
				}
			}
		}
		
		return possibleMoves;
	}

	@Override
	public String getName() {
		// TODO Auto-generated method stub
		return "Blob Wars";
	}

}
