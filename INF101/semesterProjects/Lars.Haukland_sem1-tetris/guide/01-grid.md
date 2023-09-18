forrige &bullet; [oversikt](../README.md#steg-for-steg) &bullet; [neste](./02-tegnrutenett.md)

# 1 Implementer grid

Implementer klassene, `Grid`, `Coordinate` og `CoordinateItem` i pakken *inf101v22.grid*. Når du er ferdig skal alle testene som ble delt ut sammen med prosjektet passere. Bruk gjerne en av grid -klassene du har sett tidligere (i forelesning eller på laber) som inspirasjon/utgangspunkt og gjør tilpasninger.

 - Opprett en klasse `Coordinate`.
    - Objekter av denne klassen skal ha to feltvariabler `row` og `col` som hver holder på en int. For å sikre oss mot tukling men gi enkel tilgang, gjør vi feltene for row og col `public final` (*final* betyr at verdien ikke kan endres).
    - Implementer *hashCode* og *equals*, slik at to objekter med like verdier blir ansett som like. Her kan du gjerne ta utgangspunkt i din IDE sin auto-generings-funksjon (i VS Code kan man installere en litt skranglete plugin for dette som heter *Java Code Generators*, i Eclipse er en strålende generator innebygd).
    - Implementer *toString*, slik at det printes en leselig melding når objektet skrives ut. Se testene for å se hvordan formatet skal være. 

 - Opprett en klasse `CoordinateItem` som jobber med en generisk type `E`.  
    - Objekter av denne typen skal ha to feltvariabler, ett `Coordinate` -objekt og ett element av typen `E`. For å sikre oss mot tukling men gi enkel tilgang, gjør vi feltene `public final`.
    - Implementer hashCode og equals, slik at to objekter regnes som like dersom begge feltvariablene deres er regnet som like.
    - Implementer *toString*, slik at det printes en leselig melding når objektet skrives ut. Se testene for å se hvordan formatet skal være. 

 - Opprett en klasse `Grid` som representerer et grid. Gridet skal jobbe med en generisk type `E`, og må implementere grensesnittet `IGrid<E>` som ligger klart fra før. 
    - Begynn med å opprette alle metoder som kreves av grensesnittet, men som i første omgang returner "dummy" verdier (e.g. 0 eller `null` alt ettersom).
    - Se på beskrivelsene i IGrid og hvordan testene er skrevet for å se hvilke metoder og konstruktører du trenger å implementere.

 Dersom du føler behov for å opprette andre "hjelpeklasser" for å fullføre oppgaven, står du fritt til å gjøre det.

## Iterable&lt;CoordinateItem&lt;E&gt;&gt;

Du legger kanskje merke til at `IGrid` utvider grensesnittet `Iterable<CoordinateItem<E>>`. Den helt klart vanskeligste deloppgaven i dette steget er å implementere metoden *iterator* i Grid-klassen, hvis man gjør det "skikkelig." Men det finnes også en snarvei som vi anbefaler alle å ta:
 - I metoden `iterator`, opprett en ny variabel av typen `ArrayList<CoordinateItem<E>>`.
 - Gå deretter igjennom alle koordinater i grid'et med nøstede for-løkker.
 - Hvor hvert koordinat: opprett ett *CoordinateItem&lt;E&gt;* med riktige koordinater og element, og legg det til i listen.
 - Avslutt med å returnere iteratoren du får ved å kalle `.iterator()` på ArrayList-objektet.

 Den største ulempen med denne snarveien er at vi okkuperer mer minne enn nødvendig. Men kostnaden er ikke større enn at dette er en fair løsning enn så lenge.

---

:white_check_mark: Du er ferdig når alle testene passerer.