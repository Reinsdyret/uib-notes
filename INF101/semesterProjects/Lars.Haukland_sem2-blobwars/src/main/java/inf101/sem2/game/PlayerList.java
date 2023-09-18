package inf101.sem2.game;

import java.util.ArrayList;
import java.util.Iterator;

import inf101.sem2.player.Player;

/**
 * This class keep track of players and who's turn it is.
 * In this project that might be overkill since there are only 2 players.
 * But I think the code becomes more readable when we have a PlayerList class
 *
 * @author Martin Vatshelle - martin.vatshelle@uib.no
 */
public class PlayerList implements Iterable<Player> {

	private ArrayList<Player> players;
	private int currentIndex;

	/**
	 * Creates a new empty PlayerList
	 */
	public PlayerList() {
		players = new ArrayList<>();
		currentIndex = 0;
	}

	public PlayerList(Iterable<Player> players) {
		this();
		for (Player p : players) {
			add(p);
		}
	}

	/**
	 * Adds a player to the list
	 *
	 * @param p
	 */
	public void add(Player p) {
		if (players.contains(p)) {
			throw new IllegalArgumentException("Can not have 2 players with same symbol in the same list.");
		}
		players.add(p);
	}

	@Override
	public Iterator<Player> iterator() {
		return players.iterator();
	}

	/**
	 * @return the player who has turn now
	 */
	public Player getCurrentPlayer() {
		return players.get(currentIndex);
	}

	/**
	 * advances the PlayerList to the next player
	 */
	public Player nextPlayer() {
		currentIndex++;
		updateIndex();
		return getCurrentPlayer();
	}

	/**
	 * If the end is reached
	 * the index restarts on the beginning.
	 */
	private void updateIndex() {
		if (currentIndex >= players.size() || currentIndex < 0) {
			currentIndex = 0;
		}
	}

	/**
	 * This method removes a player from the list.
	 * In this project there is no need to remove a Player.
	 * But in general one might think of games with more players
	 * where those who loose the game is removed
	 *
	 * @param index The index to remove
	 * @return The Player removed from the PlayerList
	 */
	public Player remove(int index) {
		Player p = players.remove(index);
		if (index < currentIndex) {
			currentIndex--;
		}
		updateIndex();
		return p;
	}

	public Player remove(Player p) {
		int index = players.indexOf(p);
		if (index < 0) {
			throw new IllegalArgumentException("Can not remove Player which is not in the list.");
		}
		return remove(index);
	}

	public PlayerList copy() {
		PlayerList list = new PlayerList(this);
		list.currentIndex = this.currentIndex;
		return list;
	}

	public void setCurrentPlayer(Player player) {
		int index = players.indexOf(player);
		if (index < 0) {
			throw new IllegalArgumentException("This player is not in the list");
		}
		currentIndex = index;
	}

	public void restart() {
		currentIndex = 0;
	}

	public int numPlayers() {
		return players.size();
	}
}
