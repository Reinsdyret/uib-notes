package cellular.cellstate;

import java.awt.Color;

public class AntCellState implements ICellState {

	@Override
	public Color getColor() {
		return Color.yellow;
	}

	@Override
	public int getValue() {
		return -1;
	}

}
