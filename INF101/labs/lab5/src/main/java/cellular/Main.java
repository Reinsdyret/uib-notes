package cellular;

import cellular.gui.CellAutomataGUI;

public class Main {

	public static void main(String[] args) {

		//ICellAutomaton ca = new LangtonsAnt(50, 50, "RRLLLRLLLRRR");
		ICellAutomaton ca = new SeedsAutomaton(50, 50);
		//ICellAutomaton ca = new GameOfLife(50, 50);

		CellAutomataGUI.run(ca);
	}

}
