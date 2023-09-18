package cellular.gui;


import java.awt.Component;
import java.awt.Dimension;
import java.awt.Graphics;

import cellular.ICellAutomaton;
import datastructure.Location;

/**
 * A Component that draws the cells in a ICellAutomaton.
 * @author eivind
 */
class AutomatonComponent extends Component {
	/**
	 * The dimension of the grid containing the cells.
	 */
	private final Dimension gridDim;
	/**
	 * The automaton to paint the cells of.
	 */
	private final ICellAutomaton automaton;

	/**
	 * The height of each cell in pixels.
	 */
	private final int cellHeight = 15;

	/**
	 * The width of each cell in pixels.
	 */
	private final int cellWidth = 15;
	/**
	 * The size of the space between each cell and between the cell and the edge
	 * of the window.
	 */
	private final int padding = 1;
	private static final long serialVersionUID = 4548104480314044678L;

	/**
	 * Construct a AutomatonComponent that will paint the given automaton.
	 * @param grid
	 */
	public AutomatonComponent(ICellAutomaton automaton) {
		this.automaton = automaton;
		gridDim = new Dimension((cellWidth + padding) * automaton.numberOfColumns()
				+ padding, (cellHeight + padding) * automaton.numberOfRows() + padding);
	}

	/*
	 * (non-Javadoc)
	 *
	 * @see java.awt.Component#getPreferredSize() Returns the dimensions of the
	 * grid.
	 */
	@Override
	public Dimension getPreferredSize() {
		return gridDim;
	}

	/*
	 * (non-Javadoc)
	 *
	 * @see java.awt.Component#paint(java.awt.Graphics) Paints the automaton.
	 */
	@Override
	public void paint(Graphics g) {
		for(Location loc : automaton.locations()) {
			g.setColor(automaton.getCellState(loc).getColor());
			g.fillRect(loc.col * (cellHeight + padding) + padding,
					loc.row * (cellHeight + padding) + padding,
					cellHeight, cellWidth);
		}
	}
}
