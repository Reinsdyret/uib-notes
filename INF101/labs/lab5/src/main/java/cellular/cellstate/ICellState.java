package cellular.cellstate;

import java.awt.Color;

public interface ICellState {

	/**
	 * The color that represents the the cell state
	 * @return The color that represents the the cell state
	 */
	Color getColor();

	/**
	 * The state of the cell as a number
	 * @return The state of the cell as a number
	 */
	int getValue();

}
