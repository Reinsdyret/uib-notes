[forrige](./05-roterebrikke.md) &bullet; [oversikt](../README.md#steg-for-steg) &bullet; [neste](./07-fjernefullerekker.md)

# 6 Droppe brikken og håndtere Game Over

Når du er ferdig med dette kapittelet, kan du droppe brikker ved å trykke på mellomrom -tasten. Game Over vil vises når du har tapt.

[<img src="./pics/dropPiece.gif"  width="250">](./pics/dropPiece.gif)

## Droppe brikken

- Definer en metode for å droppe brikken i `TetrisControllable`.
- Kall på overnevnte metode i `TetrisController` når brukeren trykker på space.
- Implementer overnevnte metode i `TetrisModel`. Til dette formålet bruker vi noen hjelpemetoder:
    - En hjelpemetode for å hente en ny fallende brikke. Vi skal gjøre mer i denne metoden (for å håndtere game over, se under), men i første omgang kan vi nøye oss med å hente ut en ny brikke fra brikke-fabrikken og oppdatere feltvariabelen for fallende brikke i modellen.

    - En hjelpemetode for å lime fast den fallende brikken til brettet. I denne metoden itererer du over koordinatene til den fallende brikken, og legger fliser på disse posisjonene i brettet. På slutten av denne metoden, gjør et kall til metoden som henter en ny brikke.

    - Til slutt, selve metoden som dropper en brikke. Her flytter du brikken ett og ett steg nedover i en løkke (har vi allerede en hjelpe-metode vi kan bruke?) så lenge det er mulig. Når det ikke lengre er mulig å flytte brikken nedover, limer vi den fast til brettet og henter en ny fallende brikke.

## Håndtere Game Over

Når spillet er game over skal vi vise en "Game over" -skjerm i visningen, og tastene skal ikke kunne flytte på brikkene mer.
For at visningen og kontrolleren skal kunne tilpasse oppførselen sin, må vi ha en tilstandsvariabel i modellen som forteller hvorvidt vi er game over eller ikke. 
 - Opprett en enum `GameScreen` i *inf101v22.tetris.model* som har tilstandene ACTIVE_GAME og GAME_OVER. Vi kan da enkelt legger til f. eks. WELCOME_SCREEN senere om vi ønsker.
 - La både `TetrisViewable` og `TetrisControllable` ha en metode `getGameScreen` som returnerer et objekt av typen GameScreen.
 - La `TetrisModel` ha en feltvariabel av typen GameScreen. I første omgang kan vi initiere den til GameScreen.ACTIVE_GAME. La metoden getGameScreen returnere denne variabelen.

Spillet er over når det ikke er plass til å hente en ny fallende brikke.
- I metoden som henter ut en ny fallende brikke: sjekk om brikken vi nettopp hentet er plassert lovlig på brettet. Hvis den ikke er lovlig, sett feltvariabelen til GAME_OVER.

### Visningen

Dersom spillet er game over, kan vi tegne en rute med gjennomsiktig farge (f. eks. `new Color(0, 0, 0, 128);`) som dekker hele skjermen, og oppå der skrive game over. Se på eksemplene i SampleView.

### Kontrolleren

Dersom spillet er game over, skal det ikke skje noe på tastetrykk.

---

:white_check_mark:  Du er ferdig med dette kapittelet når du kan droppe brikker med mellomromstasten, og game over vises når brettet er så fullt at en ny brikke ikke får plass.