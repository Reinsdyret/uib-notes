# Semesteroppgave 2: Blob Wars

### Bekreft at du har lest viktig informasjon

Les [praktiskinfo.md](information/praktiskinfo.md) og åpne java filen `inf101.GetStarted.java` for å bekrefte at du har lest informasjonen. JUnit testene vil ikke virke før du har gjort dette.

`add-commit-push`


**Innleveringsfrist:** Hele oppgaven skal være ferdig og levert via ditt private gitlab-repositorie til **tirsdag 29. april kl. 23:59**.  

### Innlevering 
Oppgaven leveres inn ved å pushe til git.app.uib.no, slik du har gjort med alle tidligere INF101-oppgaver OG koble repository'et til Codegrade. Husk å få med alle nye filer du har opprettet.

**VIKTIG:** *Sjekk serveren og kommentaren etter at du pusher, i tilfelle det skjer feil!* 

Vi anbefaler at du gjør commit hver dag, eller flere ganger hver dag, i tilfelle du mister det du jobber med på din egen maskin. Du kan levere inn så mye og ofte du vil. Versjonen som teller er den **siste du pushet før innleveringsfristen**.

Denne oppgaven teller på din endelige vurdering i faget. Maks poeng er 15. 


## Oversikt
Semesteroppgaven går ut på å utvide en eksisterende kodebase med brettspillet BlobWars. De tre spillene "Tic Tac Toe", "Connect Four" og "Othello" er allerede ferdigimplementert.
Denne semesteroppgaven har 2 hoveddeler:
- Teori, sett deg inn i den eksisterende koden
- Utvid funksjonaliteten ved å legge til et fjerde spill som heter [BlobWars](https://www.twoplayergames.org/game/blob-wars).
Reglene for spillet er ganske enkle men vær sikker på at du forstår reglene rett ellers kan det bli mye ekstra arbeid. 

### Blob Wars - Regler
Merk at reglene er like som spillet i [linken](https://www.twoplayergames.org/game/blob-wars) untatt regelen om "PASS".

 - Spillet begynner med én brikke i hvert hjørne, to av hver sin spiller.
 - Man kan flytte en brikke maksimalt 2 ruter fra hvor den står. Flyttes brikken én rute plasseres en ny brikke på den valgte ruten og brikken som skulle flyttes forblir i sin rute. Flyttes brikken to ruter skal brikken fjernes fra den opprinnelige ruten og plasseres på den valgte ruten.
 - Ved flytt til en rute skal alle motstanderens brikker som ligger én rute unna den flyttede brikken byttes ut med din brikke. 
 - Hvis en spiller ikke har noen gyldige trekk skal hens tur hoppes over og det blir motstanderens tur.
 - **Game Over**: 
    1. Spillet er over hvis ingen av spillerne kan flytte.
    2. Spillet er over dersom alle brikkene til en av spillerne er fjernet.
 - **Vinner**: 
    1. Spillet vinnes dersom man har fjernet alle brikkene til motstanderen
    2. Dersom hverken spiller kan flytte vinner spilleren med flest brikker på brettet.


### Læringsmål:
- sette seg inn i eksisterende kode
- forstå hva oppgaven ber om, (lese seg opp på reglene for spillet)
- planlegge kodeprosjekt, design av klasser og avhengighet mellom disse
- bruke objektorienterte prinsipper som polymorfisme, arv og abstrakte klasser
- ryddig og lesbar kodestil som gjør vedlikehold/utvidelse av koden enklere.

Et klassediagram over klassene i dette prosjektet finner dere på Mitt UiB.

### Kode

* Pakken grid - Så å si identisk til den som ble brukt i Semesteroppgave 1.

* Pakken player - Flere typer Player objekter er implementert: `GuiPlayer`, `ConsolePlayer`, `DumbPlayer`, `RandomPlayer` og `AlphaBetaPlayer`. Alle disse Spillerene kan brukes på begge spillene (og potensielt flere spill).

* Pakken game kjører spillet ved en "game loop" og ber de forskjellige spillerne om input.
  Hvilke type spiller og hvilken type grafikk det er skal ikke være viktig, man kan bytte ut begge deler.

* Pakken GUI - det som trengs for å få grafisk brukergrensesnitt (untatt GuiPlayer)

* Pakken Terminal - det som trengs for å spille fra terminal (untatt ConsolePlayer)

### Bruk
Spillet kan kjøres på to måter: 

 - inf101.sem2.MainTerminal - Kjører spillet via terminalen. 
 - inf101.sem2.MainGUI - Kjører spillet med grafisk brukergrensesnitt. 


## Oppgave 1 - Designanalyse
I denne semesteroppgaven har vi allerede laget et fungerende spill for dere.
Dere skal kjøre spillet, se på klassediagrammet og se på koden slik at dere forstår hvordan koden fungerer.
Vi har allerede brukt noen forelesninger på å forstå hvordan denne koden fungerer.
Pakken gui er litt vanskelig å forstå og ikke en viktig del av programmet, de viktigste pakkene å forstå er grid, game og player.

Skriv en analyse av designet som er valgt for den eksisterende koden i prosjektet. Fokuser på hvilke objektorienterte prinsipper som er brukt. Hvis du ser mulige forbedringer er det veldig positivt om du skriver om dette. Svar på følgende spørsmål:

- Nevn noen viktige klasser og hvorfor vi trenger disse klassene?
- Hvor brukes abstraction, encapsulation, inheritance, composition og polymorphism?
- Hvilke andre spill (eller utvidelser) vil det være enkelt å legge til i dette prosjektet, og hvilke spill vil by på utfordringer?
- Hvor er [SOLID](https://en.wikipedia.org/wiki/SOLID) prinsippene brukt/ikke brukt.
Merk at SOLID prinsippene ikke er pensum før i INF112 så vi forventer ikke så mye av dere her,
men de av dere som ønsker å få 15/15 må vise at dere har prøvd litt på dette.

## Oppgave 2 - Plan og klassediagram

Skriv en kort plan for hva du må gjøre for å implementere spillert Blob Wars.
Her bør du skrive opp de klassene du trenger å lage og hvorfor.
Tegn et klassediagram, dere skal få utlevert et klassediagram av den eksisterende koden hvor dere kan legge til de nye klassene.

Dere kan godt gå tilbake til planen og gjøre endringer hvis dere oppdager at det trengs.
Men det kan være en fordel om dere lager en ordentlig plan før dere begynner slik at dere ikke gjør mer koding enn nødvendig.

**Hint**: Den største forskjellen fra spillene TicTacToe, Connect4, Othello og BlobWars er hvordan brikkene plasseres/beveges på brettet. 
 - I TicTacToe og Othello plasseres brikker gitt en location (row, column). Dette krever én location for å plassere brikken.
 - I Connect4 plasseres alle brikker på en kolonne og faller på plass. Dette krever en enkelt heltalls-verdi for å plassere brikken.
 - I BlobWars plasserer vi ikke nye brikker på brettet men heller flytter eksisterende brikker. Dermed kreves to locations for å plassere/flytte en brikke.

Hvordan kan denne informasjonen gis? Hvordan er det gjort i `Game` med de eksisterende spillene?

## Oppgave 3 - Implementer Blob Wars
Utvid funksjonaliteten til prosjektet slik at det også går an å spille Blob Wars.
Spillet skal kunne spilles både fra terminal og fra GUI med flere valg av spillere.

Implementer valg for vanskelighetsgrad for "Single player (against AI)". I pakken `ai` finner du 3 forskjellige Player-klasser:
    1. `DumbPlayer`
    2. `RandomPlayer`
    3. `AlphaBetaPlayer`
Når brukeren velger "Single player (against AI)" skal de få et valg mellom vanskelighetsgrader hvor `DumbPlayer` er enklest og `AlphaBetaPlayer` er vanskeligst. `AlphaBetaPlayer` tar inn et parameter `level` som tilsier hvor smart den er. Her kan du legge til flere vanskelighetsgrader ved å variere `level`'en på `AlphaBetaPlayer`. Merk at hvis du setter `level` for høyt bruker `AlphaBetaPlayer` veldig lenge på å tenke.

Lesbar kode og gjenbruk av kode er viktig. 

Dere skal teste koden dere skriver. Den koden som er lett å teste med JUnit tester skal dere skrive tester for. Den delen av koden som er vanskelig å teste med JUnit tester skal dere beskrive tester der dere kjører spillet og sjekker at det virker som det skal.

Om dere finner feil i eksisterende kode så rapporterer dere det i svar.md filen.
