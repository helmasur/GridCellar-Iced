# Manuella acceptanstest

## Användning

Testen verifierar MVP enligt `blueprint.md` utan antaganden om implementationens interna teknik.

Resultat anges per test:

- Ej kört
- Godkänt
- Underkänt
- Blockerat

Vid underkänt eller blockerat resultat dokumenteras datum, testmiljö och kort orsak i testets resultatfält. Funktionella avvikelser registreras även i `implementation-status.md`.

## 1. Första start

### AT-01 Tomt projekt skapas

Förutsättning: Appens lokala data är återställd.

Steg:

1. Starta appen.

Förväntat resultat:

- Ett tomt projekt med tillfälligt namn skapas.
- En första sparad vy skapas.
- Ingen exempeldata visas.
- Huvudvyn visar tydliga val för att skapa första fältet och importera projekt.

Resultat: Ej kört.

### AT-02 Objekt kräver minst ett fält

Förutsättning: Projektet saknar fält.

Steg:

1. Försök lägga till ett objekt.

Förväntat resultat:

- Inget tomt objektformulär öppnas.
- Appen förklarar att ett fält måste skapas och erbjuder väg till fältadministrationen.

Resultat: Ej kört.

## 2. Projektinställningar

### AT-03 Byt projektnamn

Förutsättning: Ett projekt finns.

Steg:

1. Öppna projektinställningarna.
2. Ändra projektnamnet.
3. Spara och starta om appen.

Förväntat resultat:

- Det nya namnet visas direkt och finns kvar efter omstart.
- Projektets övriga data påverkas inte.

Resultat: Ej kört.

### AT-04 Globala diagraminställningar

Förutsättning: Testprojektet är inläst.

Steg:

1. Ändra radhöjd och namnkolumnens bredd.
2. Byt sparad vy.
3. Starta om appen.

Förväntat resultat:

- Inställningarna påverkar alla vyer.
- De skrivs inte över vid vybyte.
- De finns kvar efter omstart.

Resultat: Ej kört.

## 3. Fältadministration

### AT-05 Skapa samtliga fälttyper

Förutsättning: Ett tomt projekt finns.

Steg:

1. Skapa fält av typerna text, heltal, decimaltal, datum, lista och bild.
2. Skapa både enkelvärdes- och flervärdesfält där det är tillåtet.

Förväntat resultat:

- Alla fälttyper kan skapas.
- Bildfält kan endast vara enkelvärde.
- Nya fält placeras sist och visas i objektformulär och detaljpanel.

Resultat: Ej kört.

### AT-06 Ändra namn och ordning

Förutsättning: Minst tre fält och ett objekt med värden finns.

Steg:

1. Byt namn på ett fält.
2. Flytta fält uppåt och nedåt.

Förväntat resultat:

- Värden och vykonfigurationer finns kvar efter namnbytet.
- Objektformulär och detaljpanel följer den nya ordningen.

Resultat: Ej kört.

### AT-07 Spärra ogiltiga fältändringar

Förutsättning: Ett fält har objektvärden och används i diagrametikett, filter eller gruppering.

Steg:

1. Försök ta bort fältet.
2. Försök ändra dess typ eller värdeläge.

Förväntat resultat:

- Åtgärderna blockeras.
- Orsaken visas begripligt och anger användning eller antal objektvärden.

Resultat: Ej kört.

### AT-08 Obligatoriskt fält

Förutsättning: Ett fält är tomt på minst ett befintligt objekt.

Steg:

1. Försök göra fältet obligatoriskt.
2. Fyll därefter giltiga värden på samtliga objekt och försök igen.

Förväntat resultat:

- Första försöket blockeras med orsak.
- Andra försöket kan sparas.

Resultat: Ej kört.

## 4. Listvärden

### AT-09 Administrera listvärden

Förutsättning: Ett listfält finns.

Steg:

1. Skapa minst tre listvärden.
2. Ändra namn och manuell ordning.
3. Försök skapa ett dubblettnamn.

Förväntat resultat:

- Namn och ordning kan ändras.
- Dubblettnamn inom samma listfält blockeras.
- Val, visning, gruppering och sortering följer den manuella ordningen.

Resultat: Ej kört.

### AT-10 Spärra radering av använt listvärde

Förutsättning: Ett listvärde används av ett objekt eller sparat filter.

Steg:

1. Försök ta bort listvärdet.
2. Ta bort samtliga användningar och försök igen.

Förväntat resultat:

- Första försöket blockeras med orsak.
- Listvärdet kan tas bort när inga användningar återstår.

Resultat: Ej kört.

## 5. Objektflöden

### AT-11 Skapa och visa objekt

Förutsättning: Fält för samtliga MVP-typer finns.

Steg:

1. Skapa ett objekt med text, tal, datum, lista och bild.
2. Använd flera värden i tillåtna flervärdesfält.
3. Spara och öppna objektet via diagrammets radnamn.

Förväntat resultat:

- Värdena sparas och visas i global fältordning.
- Flervärden visas i användarens ordning.
- Objektets interna id visas diskret.
- Bilden visas med bibehållen proportion.

Resultat: Ej kört.

### AT-12 Normalisering och validering

Förutsättning: Text-, decimal-, datum- och obligatoriska fält finns.

Steg:

1. Skriv text med inledande och avslutande mellanslag.
2. Skriv ett decimaltal med komma.
3. Försök spara med ett obligatoriskt fält tomt eller ett ogiltigt datum.
4. Rätta värdena och spara.

Förväntat resultat:

- Yttre mellanslag trimmas och interna mellanslag bevaras.
- Decimaltecknet normaliseras utan att värdet ändras.
- Ogiltiga eller saknade obligatoriska värden blockerar sparande.

Resultat: Ej kört.

### AT-13 Redigera och återställ

Förutsättning: Ett sparat objekt finns.

Steg:

1. Öppna objektet och gå till redigeringsläge.
2. Ändra flera värden.
3. Välj Återställ.
4. Ändra igen och försök stänga panelen utan att spara.

Förväntat resultat:

- Återställ återger senast sparade värden.
- Appen varnar innan osparade ändringar förloras.

Resultat: Ej kört.

### AT-14 Duplicera och ta bort

Förutsättning: Ett objekt med flera fältvärden och bild finns.

Steg:

1. Duplicera objektet.
2. Kontrollera kopian.
3. Ta bort kopian och bekräfta.

Förväntat resultat:

- Kopian har samma fältvärden men ett nytt internt id.
- Originalet påverkas inte.
- Radering kräver bekräftelse och tar bort kopian.

Resultat: Ej kört.

## 6. Vyhantering

### AT-15 Skapa, byta och ändra vy

Förutsättning: Testprojektet är inläst.

Steg:

1. Skapa en vy.
2. Ange två grupperingsnivåer med olika sorteringsriktningar.
3. Exkludera ett datumfält och lägg till ett filter.
4. Byt vy och återvänd.

Förväntat resultat:

- Vyn sparar gruppering, sorteringsriktningar, filter och datumfältsurval.
- Global diagrametikett, layout och tidsintervall ändras inte av vybytet.
- Vertikal position återställs till toppen.

Resultat: Ej kört.

### AT-16 Duplicera, byt namn, återställ och ta bort vy

Förutsättning: Minst två vyer finns.

Steg:

1. Duplicera en vy och byt namn på kopian.
2. Ändra och återställ kopian.
3. Ta bort den aktiva kopian.
4. Försök ta bort den enda återstående vyn.

Förväntat resultat:

- Dupliceringen bevarar vykonfigurationen.
- Återställ ger alla objekt, inga filter, ingen gruppering och alla datumfält.
- Efter radering aktiveras en kvarvarande vy.
- Den sista vyn kan inte tas bort.

Resultat: Ej kört.

### AT-17 Nya datumfält i befintliga vyer

Förutsättning: Minst två sparade vyer finns.

Steg:

1. Skapa ett nytt datumfält.
2. Öppna varje befintlig vy.

Förväntat resultat:

- Det nya datumfältet är inkluderat i samtliga befintliga vyer.

Resultat: Ej kört.

## 7. Filter och sökning

### AT-18 Filteroperatorer och AND-logik

Förutsättning: Testprojektet är inläst.

Steg:

1. Prova relevanta operatorer för text, tal, datum och lista.
2. Kombinera två villkor.
3. Prova `är tomt` och `inkludera tomma värden`.

Förväntat resultat:

- Endast matchande objekt visas.
- Alla kombinerade villkor måste uppfyllas.
- Textfilter är inte skiftlägeskänsliga.
- Ett flervärdesfält matchar när minst ett värde matchar.

Resultat: Ej kört.

### AT-19 Fritextsökning

Förutsättning: Objekt med sökbara text- och listvärden finns.

Steg:

1. Sök med del av ett värde och varierad skiftläge.
2. Kombinera sökningen med ett aktivt filter.
3. Töm sökfältet.

Förväntat resultat:

- Sökningen döljer objekt som inte matchar.
- Sökning och filter tillämpas tillsammans.
- Tömning återställer filterresultatet.

Resultat: Ej kört.

## 8. Diagram

### AT-20 Rader, grupper och datum

Förutsättning: Testprojektet är inläst.

Steg:

1. Öppna vyn med gruppering.
2. Inspektera objekt med flera datum, samma datum och utan synliga datum.

Förväntat resultat:

- Varje synligt objekt har exakt en objektrad.
- Gruppetiketter är egna tunnare rader med indrag per nivå.
- Datum visas som punkter och en linje mellan tidigaste och senaste datum.
- Flera värden på samma datum visas som en punkt med samlad information.
- Objekt utan synliga datum ligger kvar och har diskret röd radton.

Resultat: Ej kört.

### AT-21 Sticky ytor och panorering

Förutsättning: Diagrammet innehåller fler rader och ett bredare datumspann än skärmen rymmer.

Steg:

1. Scrolla vertikalt.
2. Panorera eller scrolla horisontellt.

Förväntat resultat:

- Namnkolumnen ligger kvar till vänster.
- X-axeln ligger kvar längst ned.
- Tidslinjen rör sig bakom namnkolumnen.

Resultat: Ej kört.

### AT-22 Interaktion och tooltip

Förutsättning: Ett objekt har flera datumfält på samma datum.

Steg:

1. Klicka eller tryck på en datumpunkt.
2. Klicka eller tryck på linjen.
3. Öppna objektet via radnamnet.

Förväntat resultat:

- Punkten visar objektetikett samt alla fältnamn och datum på positionen.
- Punkt och linje kan markeras men öppnar inte detaljpanelen.
- Endast radnamnet öppnar detaljpanelen.

Resultat: Ej kört.

### AT-23 Tidsintervall

Förutsättning: Testprojektet innehåller datum över ett långt spann.

Steg:

1. Prova Visa allt, 5 år, 10 år och Egen period.
2. Använd Passa in alla datum.

Förväntat resultat:

- X-axelns nivå anpassas mellan dag, månad och år.
- Objekt utanför intervallet ligger kvar men markeras om inga datum syns.
- Idag-linjen visas endast när dagens datum ligger i intervallet.
- Passa in påverkar aktuell visning utan att ändra sparad vy eller global tidsinställning.

Resultat: Ej kört.

## 9. Lokal persistens

### AT-24 Data finns kvar mellan sessioner

Förutsättning: Projektet har ändrade inställningar, fält, listvärden, objekt, bild och vyer.

Steg:

1. Avsluta appen helt.
2. Starta appen igen utan internetanslutning.

Förväntat resultat:

- Hela projektet återställs.
- Senast använda vy öppnas.
- Appens centrala funktioner kan användas utan internet.

Resultat: Ej kört.

## 10. Export och import

### AT-25 Komplett export och import

Förutsättning: Testprojektet med bild är inläst.

Steg:

1. Exportera projektet.
2. Ändra eller ersätt det aktiva projektet.
3. Importera den exporterade projektfilen och bekräfta ersättning.

Förväntat resultat:

- Importen återskapar projektinställningar, interna id:n, fält, listvärden, objekt, ordning, bilder, vyer, filter och diagrametikett.
- Den tidigare aktiva projektinformationen ersätts och sammanfogas inte.

Resultat: Ej kört.

### AT-26 Avbruten eller ogiltig import

Förutsättning: Ett befintligt projekt finns.

Steg:

1. Starta import men avbryt ersättningsbekräftelsen.
2. Försök importera en ogiltig eller skadad fil.

Förväntat resultat:

- Avbruten import ändrar ingenting.
- Ogiltig import visar ett begripligt fel.
- Befintligt projekt förblir intakt.

Resultat: Ej kört.

## 11. Smal och bred layout

### AT-27 Samma grundflöde på smal och bred yta

Förutsättning: Appen kan köras på smal och bred fönsteryta.

Steg:

1. Genomför vybyte, filtrering, objektvisning och objektredigering på båda bredderna.

Förväntat resultat:

- Samma funktioner och grundstruktur finns på båda.
- Smal layout prioriterar yta utan att ta bort funktioner.
- Detaljpanelen är nästan helskärm på smal yta och en större modal panel på bred yta.
- Horisontell diagraminteraktion fungerar på båda.

Resultat: Ej kört.

### AT-28 Tangentbord och fokus

Förutsättning: Appen körs på desktop.

Steg:

1. Navigera primära åtgärder med Tab.
2. Öppna ett objekt från fokuserat radnamn med Enter.
3. Öppna en modal och försök flytta fokus utanför den.
4. Stäng med Escape.

Förväntat resultat:

- Primära åtgärder kan nås med tangentbord.
- Enter öppnar detaljpanelen från radnamnet.
- Fokus hålls inom öppen modal eller panel.
- Escape stänger när det är säkert; osparade ändringar skyddas.
- Nödvändig information kräver inte hover.

Resultat: Ej kört.

## 12. Kapacitetsriktmärke

### AT-29 Rimlig användbarhet vid riktmärket

Förutsättning: Ett projekt med minst 500 objekt, 50 fält och 10 sparade vyer finns.

Steg:

1. Starta projektet.
2. Byt vy, filtrera, scrolla diagrammet och öppna ett objekt.

Förväntat resultat:

- Appen förblir rimligt användbar utan uppenbara låsningar eller informationsförlust.
- Testet är ett riktmärke och inte ett krav på en särskild svarstid.

Resultat: Ej kört.
