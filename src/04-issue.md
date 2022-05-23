# Issue

Quindi:
- per gli amanti dell'approccio funzionale, non siamo in grado di garantire che qualcosa cambi i parametri di input
- in JS, le variabili allocate nello heap sono passate per referenza (oggetti, array, etc...). I metodi che invochiamo possono quindi cambiarlo

*Quale sarebbe il comportamento che ci piacerebbe invece avere?*
- permettere "la modifica solo quando veramente la vogliamo"
- non permetterla altrimenti

Ovvero: che la possibilità di modifica sia **esplicita**


**Idee ?**
