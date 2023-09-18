package cellular;

import cellular.cellstate.AntCellState;
import cellular.cellstate.CellState;
import cellular.cellstate.ICellState;
import datastructure.Grid;
import datastructure.GridDirection;
import datastructure.IGrid;
import datastructure.Location;

public class LangtonsAnt implements ICellAutomaton {

	/**
	 * The maximum length of any valid rule. Should not exceed 256, otherwise not
	 * all colors will be distinct.
	 */
	private static final int MAX_RULE_LENGTH = 32;

	/**
	 * Stores the rule, in the following sense: Upon reading a state whose value is
	 * i, char[i] indicates whether to turn left ('L') or right ('R')
	 */
	private final char[] rule;

	/**
	 * A grid of the cell states in the current generation
	 */
	private final IGrid<ICellState> currentGeneration;

	/**
	 * The ant
	 */
	private Ant ant;

	/**
	 * The state the ant saw upon moving to the field it is placed on currently.
	 * Determines the next move (using the given rule).
	 */
	protected ICellState seenState;

	/**
	 * @param rows - number of rows in this automaton
	 * @param cols - number of columns in this automaton
	 * @param rule String of characters 'L' and 'R'
	 */
	public LangtonsAnt(int rows, int cols, String rule) {
		this.currentGeneration = new Grid<>(rows, cols, null);
		checkRule(rule);
		this.rule = rule.toCharArray();
		initializeCells();
	}

	/**
	 * Checks the rule for validity. A string is not a rule its length exceeds the
	 * maximum length ({@link #MAX_RULE_LENGTH}) or if it contains characters other
	 * than 'L' and 'R'.
	 * 
	 * @param rule
	 */
	private void checkRule(String rule) {
		if (rule.length() > MAX_RULE_LENGTH) {
			throw new IllegalArgumentException("The rule " + rule + " is too long. Its length is " + rule.length()
					+ " while MAX_RULE_LENGTH = " + MAX_RULE_LENGTH);
		}
		char[] ruleChars = rule.toCharArray();
		for (int i = 0; i < ruleChars.length; i++) {
			if (ruleChars[i] != 'R' && ruleChars[i] != 'L') {
				throw new IllegalArgumentException(
						"The rule " + rule + " is not a valid rule (see " + ruleChars[i] + " at position " + i + ").");
			}
		}
	}

	@Override
	public ICellState getCellState(Location loc) {
		// This method returns different shades of the same color.
		// The scaling depends on the length of the given rule.
		return currentGeneration.get(loc);
	}

	@Override
	public void initializeCells() {
		// Set all fields to be in state 0.
		for (Location loc : currentGeneration.locations()) {
			currentGeneration.set(loc, new CellState(0, rule.length));
		}
		// Initialize the seenState field (All fields are 0 in the beginning)
		this.seenState = new CellState(0, rule.length);
		// Place the ant and initialize the corresponding field variables.
		int midX = this.currentGeneration.numRows() / 2, midY = this.currentGeneration.numColumns() / 2;
		Location midLoc = new Location(midX, midY);
		this.ant = new Ant(midLoc, GridDirection.NORTH);
		this.currentGeneration.set(midLoc, new AntCellState());
	}

	@Override
	public void step() {
		int color = seenState.getValue();
		Ant nextAnt = ant.copy();

		if (rule[color] == 'L') {
			// turn left
			nextAnt.turnLeft();
		}
		if (rule[color] == 'R') {
			// turn right
			nextAnt.turnRight();
		}
		nextAnt.move();
		// throw back into field if necessary
		if (nextAnt.getRow() >= currentGeneration.numRows()) {
			nextAnt.setRow(currentGeneration.numRows() - 3);
		}
		if (nextAnt.getCol() >= currentGeneration.numColumns()) {
			nextAnt.setCol(currentGeneration.numColumns() - 3);
		}
		if (nextAnt.getRow() < 0) {
			nextAnt.setRow(2);
		}
		if (nextAnt.getCol() < 0) {
			nextAnt.setCol(2);
		}

		// Update the state of the filed the ant is leaving.
		currentGeneration.set(ant.getLocation(), new CellState((color + 1) % rule.length, rule.length));

		// Update the state the ant is reading in the field it is moving to.
		seenState = currentGeneration.get(nextAnt.getLocation());
		// Move the ant to the new position
		currentGeneration.set(nextAnt.getLocation(), new AntCellState());
		// Update the field variables corresponding to the ant's position.
		this.ant = nextAnt;
	}

	@Override
	public int numberOfRows() {
		return this.currentGeneration.numRows();
	}

	@Override
	public int numberOfColumns() {
		return this.currentGeneration.numColumns();
	}

	@Override
	public Iterable<Location> locations() {
		return currentGeneration.locations();
	}

}
