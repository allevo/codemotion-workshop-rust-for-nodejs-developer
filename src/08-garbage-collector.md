# Come viene gestita la memoria?

## In C
In C (e C++) la gestione della memoria è *manuale*, tanto che spetta allo sviluppatore scrivire "ehi! mi daresti 5MB di memoria?" e "oki, ho finito con questi 5MB, potresti liberarli?".
In questo modo è il programma, scritto in toto dallo sviluppatore, che chiede esplicitamente della memoria.

## In Node.js
In Node.js la memoria viene gestita tramite il Garbage Collector, ovvero un componente di *runtime* che esegue una serie di controlli per conoscere quale variabile è da deallocare.
Node.js non è l'unico che utilizza questo approcchio, anzi. Un altro esempio, forse il più famoso è Java (ma anche PHP, python, Go) seppur con una differenza di implementazione.