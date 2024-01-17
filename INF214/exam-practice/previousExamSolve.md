# Exam h22

## Task 3 
```java

int PLAYERS = 3

sem gloggDrunk = 0;

sem missingGlogg = 0;
sem missingMug = 0;
sem missingAlmond = 0;

sem[] missingList = [missingGlogg, missingMug, missingAlmond]

process player[i = 1 to PLAYERS] {
    while (true) {
        // The ingredient that the player has is missingList[i]
        // We then wait for the bartender to signal to that semaphore (to signal it is missing)
        P(missingList[i]);

        // Pick up ingredients and drink
    
        V(gloggDrunk);
    }
}

process bartender {
    while (true){
        // Pick out random number to choose what ingredient is missing 
    
        int randomNum = random.randint(0,3); //Pick random number from 0 to 3 not including 3 
        V(missingList[randomNum]);

        // Wait for them to drink it 
        P(gloggDrunk);
    }
}

```
