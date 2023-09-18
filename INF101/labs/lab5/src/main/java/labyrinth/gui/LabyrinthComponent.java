package labyrinth.gui;

import java.awt.Component;
import java.awt.Dimension;
import java.awt.Graphics;

import datastructure.Location;
import labyrinth.ILabyrinth;

/**
 * A Component that draws the cells in a ICelllabyrinth.
 * @author eivind
 */
class LabyrinthComponent extends Component {
	private static final long serialVersionUID = 4548104480314044678L;
	/**
	 * The dimension of the grid containing the cells.
	 */
	private Dimension gridDim;

	/**
	 * The labyrinth to paint the cells of.
	 */
	private ILabyrinth labyrinth;

	/**
	 * The height of each cell in pixels.
	 */
	private final int cellHeight = 20;
	/**
	 * The width of each cell in pixels.
	 */
	private final int cellWidth = 20;
	/**
	 * The size of the space between each cell and between the cell and the edge
	 * of the window.
	 */
	private final int padding = 1;

	/**
	 * Construct a labyrinthComponent that will paint the given labyrinth.
	 * @param grid
	 */
	public LabyrinthComponent(ILabyrinth labyrinth) {
		setLabyrinth(labyrinth);
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
	 * @see java.awt.Component#paint(java.awt.Graphics) Paints the labyrinth.
	 */
	@Override
	public void paint(Graphics g) {
		for(Location loc : labyrinth.locations()){
			g.setColor(labyrinth.getColor(loc));
			g.fillRect(loc.col * (cellWidth + padding) + padding, //
					loc.row * (cellHeight + padding) + padding, //
					cellWidth, cellHeight);
		}
	}

	public void setLabyrinth(ILabyrinth lab) {
		this.labyrinth = lab;
		gridDim = new Dimension((cellWidth + padding) * labyrinth.numberOfColumns() + padding,
				(cellHeight + padding) * labyrinth.numberOfRows() + padding);
	}
}
