package inf101.grid;

public class Move {

	/** Starting location for move */
	public Location start;
	
	/** End location for move */
	public Location end;
	
	/** If the start blob should be deleted or not 
	 * A true value represents that the move is over two cells and the start location should be deleted
	 * A false value represents that the move is over one cell and the start location stays
	 * If null then the move is invalid */
	public Boolean moveType;
	
	public Move(Location start, Location end) {
		this.start = start;
		this.end = end;
		
		int deltaCol = Math.abs(this.end.col - this.start.col);
		int deltaRow = Math.abs(this.end.row - this.start.row);
		
		if(deltaCol >= 0 && deltaCol <= 2 && deltaRow >= 0 && deltaRow <= 2) {
			if(deltaCol > 1 || deltaRow > 1) {
				this.moveType = Boolean.TRUE;
			}else {
				this.moveType = Boolean.FALSE;
			}
		}
	}

	
	public boolean isValid() {
		return this.moveType != null;
	}
	
	@Override
	public boolean equals(Object obj) {
		if (obj instanceof Move) {
			Move move = (Move) obj;
			return this.start.equals(move.start) && this.end.equals(move.end);
		}
		return false;
	}
	
	@Override
	public String toString() {
		return "Start: " + this.start.toString() + " End: " + this.end.toString();
	}
}
