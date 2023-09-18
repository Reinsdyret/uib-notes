# Svar på spørsmål

*I denne filen legger du inn svar på spørsmålene som stilles i oppgaveteksten, eventuelt kommentarer og informasjon om kreative løsninger*

   
## Oppgave 1
### Nevn noen viktige klasser og hvorfor vi trenger disse klassene?
 - The most important classes I see are Grid, Game and GameBoard
	- Grid is the foundation for all the games as they all use some sort of grid to hold pieces
	- Game is another class that is the foundation for all the games as this class is responsible for initializing the game and has the game loop
	- GameBoard is the class responsible for saving and changing the state of the game. It is what connects grid and game together to make a playable game
	
### Hvor brukes abstraction, encapsulation, inheritance, composition og polymorphism?
These four are the fundamental OOP consepts.

 - Abstraction, "the process of hiding certain details and showing only essential information to the user" (w3schools.com/java/java_abstract.asp)
	- Abstraction is typically achieved through interfaces or abstract classes.
	- Interfaces are a some sort of template for what method/s the class that implements it must implement. The interface does not contain any body for these methods. By using interfaces the user coding only sees the interface object, it does not see how the methods are implemented nor if there are any more methods in the class implementing it.
		- Some interfaces that helps abstraction in this project are Player.java, Graphics.java and IGrid.java.
	- Abstract classes are somewhat similar to interfaces but there is a key difference. While in interfaces the user can only see how the interface is defined, in abstract classes the user can only see how the classes extending the abstract class are defined. The big but for abstract classes is that these can contain attributes/fields and body for methods. Making the usecases for abstract classes slightly different from interfaces. It is also important to remember that there cannot be any objects created from an abstract class
	
 - Encapsulation, also called data hiding "is a mechanism of wrapping the data (variables) and code acting on the data (methods) together as a single unit" (https://www.tutorialspoint.com/java/java_encapsulation.htm).
	- To achieve encapsulation we declare the attributes of a class as private (only accessible from its own class), create public getter and setter methods to change and see the attributes. We may also make methods that are only used by its own class private to mitigate what is public for the user
		- Encapsulation in this project is achieved many places, the two most important are probably Grid.java and BoardGame.java
			- Grid.java has the dimensions and list of cells hidden with getters and setters. This makes it easier to for example check if a changing of a cell is valid or not, as without encapsulation the user could set each cell manually.
			- BoardGame.java hides the grid. With respective getters and setters.
			
 - Inheritance, "can be defined as the process where one class acquires the properties (methods and fields) of another" (https://www.tutorialspoint.com/java/java_inheritance.htm). Using inheritance the code becomes more easily managed.
	- Inheritance is achieved by using the keyword extends. You can see that I used this word when describing an abstract class. This is because for abstract classes, and others, inheritance is used where other classes inherits that class.
		- In this project inheritance is used a lot the most important of them all are probably from Grid.java and Game.java. A lot of classes extend these classes and they are responsible for most of the basic game logic.

 - Composition, having classes that can contain object of other classes in fields. An example of this is that a library object contains multiple book objects. The advantages of composition is that you can reuse code, design clean APIs and change the composition of a class in composition without having to alter others. (https://stackify.com/oop-concepts-composition/)
	- In this project alot of composition is going on. I will again look at the key classes for their use, these are grid game and gameBoard
		- The grid class does not have the most obvious examples of composition. The class itself is more used as the object within another, but there is one example. The cells attribute describes a list of the generic type T, this T can be any object that the user wants
		- The game class has a bit more composition, it has the attributes board(GameBoard), graphics(Graphics) and players(PlayerList) all are classes that have been created within the project.
		- The GameBoard class which we saw is already used in composition also has some examples of this. It has an attribute grid with the generic type player.
		These examples show the nice thing about composition, you can put objects in objects in objects and so on. In this example the outer object is game that contains a GameBoard, and the GameBoard contains a Grid, and the grid contains a list of Players.

 - Polymorphism "means "many forms", and it occurs when we have many classes that are related to each other by inheritance." (https://www.w3schools.com/java/java_polymorphism.asp)
	- So a typical example is a class that multiple classes can inherit and use for similar but slightly different purposes.
		- The best example I can see in the project is the Game class. This class is made for a turn based game with win/tie/lose outcomes (from class doc). This is a very general class and we can already see it is inherited from all the games in the games package.

### Hvilke andre spill (eller utvidelser) vil det være enkelt å legge til i dette prosjektet, og hvilke spill vil by på utfordringer?

We know from what I wrote about polymorphism that the games that can be made form the Game class are turnbased games with win/tie/lose outcomes. Form this we can create a lot of multiplayer games, also singleplayer but this is with a robot.

 - Chess is a game with a grid, turnbased with winconditions of win/tie/lose. This would be possible to make, of course it has more advanced rules than TicTacToe but the Game class and others support it. We would have to create some classes for the pieces but that is possible.
	- There is also 4 player chess which in theory should be able to create.

 - Go, the chinese classic is also easily implemented.
 
Games that would be more difficult to create are typical singleplayer games, or multiplayer games where there is no turn. Examples of these:

 - Harold (https://www.metacritic.com/game/pc/harold) is described as a singleplayer sidescroller. This game would be extremely hard to implement with the game class and I would argue you would have to remake the entire project.

 - Minecraft, an openworld single and multiplayer game. Recreating this game would of course be very difficult but a simpler version can be possible. Unfortunately not with the game class here. While you could argue a simple version of Minecraft could use a 3d grid, this project only has a 2d grid and Minecraft is not turnbased, with constant input and changes.
 
### Hvor er SOLID prinsippene brukt/ikke brukt.

Source used for this task: https://en.wikipedia.org/wiki/SOLID all explanations are taken from this but the examples are my own work

Single-responsibility principle: Every class should only have one responsibility
 - Here most of the classes in the grid package are nice examples. 
	- Grid.java has one responsibility: keep track of the grid
	- Location.java: keep track of the row and col of some location on the grid
	- Move.java: This I would argue is not fully single responsibility because of the calculation of the attribute moveType

#### Open–closed principle: 
"Software entities ... should be open for extension, but closed for modification."
 - Game.java is an amazing example of this: It is abstract so no it cannot be instanciated without another class extending it. This forces the open-closed principle
 - Of course all interfaces are also good example where this principle is used.
 
#### Liskov substitution principle: 
"Functions that use pointers or references to base classes must be able to use objects of derived classes without knowing it."
 - How I observe that this is easily achieved is if objects are referred to their highest abstract class or interface that they extend/implement. For example if we have a class "subGame" extending the abstract class "superGame" then objects made from "subGame" should be referred to as a "superGame" type and only methods and attributes from "superGame" should be used.
 - An example of where this is used is for example in MainMenu.java where the game to be played is of the type Game<?>. Because of how this and similar classes are then coded any game extending this class should in theory work. Of course this is only true for the type of games I have described in the last point and not games such as Harold, Minecraft.
 
#### Interface segregation principle: 
states that no code should be forced to depend on methods it does not use.
 - This principle can also be explaned as split large interfaces into smaller ones. So instead of having one giant interface that has all the necessary methods for very different classes we can split this into lots of smaller ones so that the methods dont have to be implemented in every class.
 - Some nice examples of this in this project is all of the interfaces. The Graphics.java interface has methods for graphic classes implementing it and the IGrid.java class has methods that grid classes need. These are not combined into one interface as this would violate the principle
 
#### Dependency inversion principle:
This is a very abstract principle (pun intended) so boiling it down to one sentence is quite tough. Luckily my costudent Mats Dyrøy had a very good exlanation that read "Lag avhengigheter til abstrakte klasser eller grensesnitt, ikke selve implementasjonen." note that this is a direct copy of his work and all credit is due to him.
 - From this explanation I found it quite similar to the Liskov principle but at the same time different
 - Examples of where this is used is for example in MainMenu.java again where the game in question is not referred to as the type BlobWars or Othello but it is referred to as the parent abstract class Game with anything as its generics.
 

## Oppgave 2
### Plan for blobwars:
#### Classes needed:
I could create a move class that specifies what from what locations the two clicks are. Parameters will be loc1 and loc2 from there I can then initialize a new attribute that says if the previous location is to be deleted or not. F.ex if the blob moves two cells forwards then i add a new blob in that location and delete the previous but if the blob moves only one cell then I dont delete the previous. This will be in the grid package.

Of course I need the blobwars game class extending the game class with the new move class as the generic type.
	In this class I need to implement
An init method, populating the board etc
All methods from the game class not already implemented
Some method for handling the two click moves, limited to the 5x5 grid around “blob”
If the player has clicked a piece and then clicks another piece the available moves should be shown according to the newly clicked piece.
If the player makes a valid move, this move should be done

#### Roadmap:
Create tests as I go
Create button for blobWars
Create move class
Create blobwars class
Implement methods and game logic
Make choices for ai level.


## Oppgave 3
A small improvement on the othello game is that the game ends when a player has no more moves. In actual Othello the tactic of limiting your opponents moves is a key tactic and the game would never end if only one player has no more moves as the other player moving pieces would open up moves.

Also a big thanks to a fellow student Ljubomir Simic who helped me debug the constructors in blobWars and tipped me about only using two constructors

Haaper det gaar greit jeg skriver paa engelsk