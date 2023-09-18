package cellular.cellstate;

import java.awt.Color;
import java.util.Random;

/**
 * The State of a cell
 */
public class CellState implements ICellState {


	public static final CellState ALIVE = new CellState(2),
			DYING = new CellState(1),
			DEAD = new CellState(0);

	private static final CellState[] values = {ALIVE, DYING, DEAD};

	protected final int data;

	private final int levels;

	/**
	 * Construct a CellState which is either alive, dying or dead
	 * @param data The value of the cell (between 0 and 3)
	 */
	public CellState(int data) {
		this.data = data;
		levels = 3;
	}

	/**
	 * Construct a CellState which is either alive, dying or dead
	 * @param data The value of the cell (between 0 and levels)
	 * @param levels The number of possible states the cell can have
	 */
	public CellState(int data, int levels) {
		this.data = data;
		this.levels = levels;
	}

	/**
	 * Returns one of the 'standard' values.
	 * @param rand
	 * @return
	 */
	public static CellState random(Random rand) {
		return values[rand.nextInt(values.length)];
	}

	@Override
	public int getValue() {
		return data;
	}

	@Override
	public Color getColor() {
		int val = getValue() * 256 / levels;
		return new Color(255, 255 - val, 255 - val);
	}
}

