package labyrinth;

import labyrinth.gui.LabyrinthGUI;

public class Main {
	// Labyrint
	public static char[][] testLabyrint = { //
			{ '*', '*', '*', '*' }, //
			{ '*', ' ', ' ', '*' }, //
			{ '*', ' ', '*', '*' }, //
			{ '*', 's', '*', '*' }, //
			{ '*', '*', '*', '*' }, };

	public static void main(String[] args) {
		// LabyrinthGUI.run(() -> new
		// Labyrinth(LabyrinthHelper.loadGrid(testLabyrint)));
		LabyrinthGUI.run(() -> new Labyrinth(LabyrinthHelper.makeRandomGrid(20, 30)));
	}
}
