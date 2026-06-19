# GridCellar blueprint, arbetsformat

## Status

Detta är arbetsformat för blueprint. Dokumentet beskriver fastställda mål, nödvändiga inferenser, föreslagna avgränsningar och öppna frågor innan materialet eventuellt konsolideras till `blueprint.md`.

Dokumentet är uppdaterat efter kvalitetsgranskning 2026-06-17. Tidigare formuleringar om användarsynliga fältnycklar, texttaggar, Markdown-presentation, diagrametiketter per vy och hårdkodade bild-/antal-/öppnade-fält skall inte längre förstås som styrande.

## Produktidé

GridCellar är en mobil- och desktopanpassad webapp för att katalogisera källarlagrade objekt. Första användningsfallet är flaskor med öl och vin, men MVP-modellen är generisk: objekt beskrivs med användardefinierade fält och visas i ett tidslinjediagram.

Det primära värdet är en tidsbaserad visualisering där användaren kan se hur lagrade objekt fördelar sig över tid och hur de grupperas enligt användarens egna fält. Appen skall i första hand visualisera källarens innehåll, i andra hand hjälpa användaren avgöra när objekt bör användas eller öppnas, och i tredje hand fungera som inventarieförteckning.

Systemet skall inte låsas till en hårdkodad modell för öl, vin eller flaskor. Källarprojektet skall i stället ha globala användardefinierade fält, globala listor per listfält, sparade diagramvyer och en central diagramvy som påminner om en förenklad pivot-tabell över tid. Öl, vin och flaskor får användas som exempel och testdata, men skall inte hårdkodas i datamodellen.

## Fastställd målbild

- Appen skall vara en webapp för både mobil och desktop.
- Mobil och desktop är lika viktiga.
- Mobil och desktop skall använda samma grund-UI; skillnaden skall främst ligga i överblick, yta och skalning.
- Första målbilden omfattar en logisk källare.
- Arkitekturen skall hållas öppen för flera källare senare.
- Appen skall sakna inloggning i första målbilden.
- Teknikval, lagringsform och arkitektur skall beslutas senare.
- MVP skall ha komplett manuell export och import av ett helt projekt, inklusive bilder, för backup och flytt. CSV/Excel-import och CSV/Excel-export ingår inte i MVP.
- MVP skall kräva persistent lagring och som målprincip kunna fungera utan internet.
- Synk mellan flera enheter och flera samtidiga användare ingår inte i MVP.
- Teknikval skall endast jämföra öppna och fria tekniker.
- Startmallar för exempelvis öl och vin skall hållas öppna för framtiden men inte ingå i första målbilden.

## Begreppsmodell

### Källarprojekt

Ett källarprojekt har internt id och visningsnamn. Visningsnamnet används i UI, medan internt id gör modellen redo för flera projekt senare.

Ett källarprojekt är den överordnade behållaren för:

- objekt,
- globala fältdefinitioner,
- globala listor per listfält,
- sparade diagramvyer,
- global fältordning,
- global detaljruteformatering per fält,
- global diagrametikett,
- globala diagraminställningar, exempelvis radhöjd, namnkolumnbredd och tidsintervall,
- export-/importfunktioner,
- eventuell framtida projektkonfiguration.

Första versionen behöver bara hantera ett källarprojekt, men modellen skall inte blockera flera projekt senare.

### Objekt

Ett objekt är en post i källarprojektet. Objektet kan representera en enskild fysisk enhet eller ett parti av flera enheter. Första användningsfallet är flaskor och flaskpartier, men detta är inte hårdkodat i modellen.

Objektet skall inte ha hårdkodade domänfält som `namn`, `kategori`, `antal`, `öppnade`, `status`, `huvudbild` eller `plats`. Sådant skapas av användaren som globala fält.

Enda hårdkodade systemfältet är:

- internt id.

Det interna id:t skall vara stabilt, diskret synligt för användaren och alltid användas som sista sorteringsnyckel. Det gör att två objekt aldrig får samma slutliga sorteringsposition.

Om användaren vill hantera antal, ursprungligt antal, öppnade flaskor eller förbrukad status görs det med vanliga användardefinierade fält. Första målbilden skall inte innehålla en särskild öppna- eller förbruka-funktion.

Objekt skall kunna dupliceras. Duplicering skapar ett nytt objekt med kopierade fältvärden men nytt internt id.

### Globala fält

Fältdefinitioner är globala inom ett källarprojekt. Alla objekt använder samma uppsättning fält.

Användaren skall kunna skapa fält även när objekt redan finns. Befintliga objekt får då tomt värde i det nya fältet. Nya fält läggs sist i global fältordning och blir automatiskt tillgängliga i filter och y-hierarki.

Fält skall väljas i UI via fältnamn och internt fält-id. Första målbilden skall inte använda användarsynliga fältnycklar, texttaggar eller textbaserade fältreferenser.

Fält kan byta namn efter att de skapats. Eftersom vyer och värden kopplas via internt fält-id påverkar namnbyte bara visningen i UI.

Alla globala fält visas automatiskt i detaljrutan. Fält kan inte döljas från detaljrutan utan att tas bort från källarprojektet.

Varje fält har:

- internt fält-id,
- visningsnamn,
- fälttyp,
- värdeläge: ett värde eller flera värden,
- obligatorisk-markering,
- global ordning,
- globalt detaljformat när formatval är relevant.

Fältordning ändras med upp- och nedknappar i konfigurationsvyn. Samma enkla interaktion skall kunna användas på mobil och desktop.

Ett fält får bara tas bort om fältet saknar värden på alla objekt och inte används i någon konfiguration, exempelvis global etikett, vy, filter, gruppering eller inkluderade datumfält. Ett fälts typ får ändras endast om fältet saknar värden på alla objekt och inte används i någon sådan konfiguration. Om obligatorisk-markering slås på måste alla befintliga objekt ha giltigt värde innan ändringen kan sparas.

### Fälttyper

Fälttyper i första målbilden:

- text,
- tal,
- datum,
- val från lista,
- bild.

Datum skall vara fullständiga datum, exempelvis `2028-05-14`. Grova datum som endast år, år+månad eller intervall ingår inte i första målbilden. `Ja/nej` ingår inte som egen fälttyp i MVP och kan vid behov lösas med listfält, exempelvis `Ja` och `Nej`. Längre anteckningar är inte en egen fälttyp, utan löses med textfält och detaljformatet `Längre textblock`.

### Flervärdesfält

När ett fält skapas väljer användaren om fältet tillåter ett värde eller flera värden. Detta värdeläge får bara ändras om fältet är tomt på alla objekt.

Alla fälttyper utom bild kan vara flervärdesfält.

Flervärdesfält redigeras som en ordnad lista eller chips där användaren kan:

- lägga till värde,
- ta bort värde,
- flytta värde upp,
- flytta värde ned.

När ett flervärdesfält används för sortering jämförs värdena i användarens inbördes ordning:

1. första värdet,
2. andra värdet om första är lika,
3. tredje värdet om tidigare värden är lika,
4. och så vidare.

Ett flervärdesfält kan vara obligatoriskt. Då måste fältet innehålla minst ett värde.

Filter kan användas på flervärdesfält. För flervärdesfält betyder filtervillkoret att minst ett av objektets värden måste matcha.

Ett objekt skall ändå alltid ha en enda placering i y-hierarkin. Ett objekt skall alltså inte dupliceras till flera grenar bara för att ett fält har flera värden.

### Listfält

Val-listor är globala per fält. Exempel kan vara ölstilar, länder, druvor eller vad användaren själv väljer.

Listvärden har intern identitet och visningsnamn. Objekt refererar till listvärdets interna identitet, inte bara dess text.

Listvärden administreras direkt på respektive listfält i fältadministrationen. I MVP kan nya listvärden inte skrivas in direkt vid objektredigering; de skall först skapas i fältadministrationen. Detta håller sorteringsordning och liststruktur kontrollerad.

Ett listfält får finnas med tom lista, men då kan användaren inte välja något värde förrän listvärden har skapats.

Listvärden skall kunna:

- ändra ordning,
- byta namn även om objekt använder dem,
- tas bort bara när de inte används av objekt och inte används i sparade filter.

Listvärden skall inte kunna inaktiveras i MVP.

Listvärdesnamn skall vara unika inom samma listfält. Samma namn får däremot förekomma i olika listfält.

När listfält används för sortering skall listans manuella ordning användas i stället för naturlig sortering.

### Bildfält

Bildfält är en vanlig fälttyp och inte ett systemfält. Ett källarprojekt kan ha noll, ett eller flera bildfält.

I första målbilden innehåller ett bildfält en bild per objekt. Bildfält följer global fältordning i detaljrutan. Tomma bildfält visas diskret, exempelvis med diskret bildplatshållare eller `—`.

Bildfält skall inte användas i filter, sortering eller y-hierarki i första målbilden. Bildfält skall kunna vara obligatoriskt.

## Fältpresentation och detaljruta

Detaljrutan visar automatiskt alla globala fält i global fältordning. Ett fält visas högst en gång.

Varje fält skall ha ett globalt detaljformat. Formatet anges i fältadministrationen och gäller för fältets presentation i detaljrutan.

Om ett fält bara kan ha ett rimligt format skall användaren inte behöva välja format. Om flera format är rimliga visas bara relevanta format för aktuell fälttyp.

Detaljformat i första målbilden:

- normal rad,
- rubrikrad,
- kompakt etikett/chip,
- längre textblock,
- bild,
- datum,
- tal.

Formatregler:

- `Rubrikrad`: värdet visas större och tydligare än normal rad. Fältnamnet kan döljas om värde finns. Om värde saknas visas fältnamn och diskret tomvärde.
- `Normal rad`: standardrad med fältnamn och värde. Layouten får anpassas efter skärmbredd, men formatet skall vara enkelt och konsekvent.
- `Kompakt etikett/chip`: fältnamn visas som rubrik och värden visas som chips. För flervärdesfält visas ett chip per värde.
- `Längre textblock`: fältnamn visas som rubrik och värdet visas som vanlig radbruten text. Markdown ingår inte i MVP.
- `Bild`: bilden visas på fältets plats i detaljvyn, maxad till panelens bredd med bibehållen proportion. Tomt bildfält visas som diskret platshållare.
- `Datum`: datum visas som fullständigt datum.
- `Tal`: tal visas enligt fältets talinställning, heltal eller decimaltal.

Exempel på lågkomplex tillämpning:

- Bildfält visas som bild.
- Datumfält visas som datum.
- Talfält visas som tal.
- Textfält kan visas som normal rad, rubrikrad eller längre textblock när det är rimligt.
- Listfält kan visas som normal rad eller kompakt etikett/chip.

Markdown-presentation, textbaserade mallar och fälttaggar skall inte ingå i första målbilden. Presentation skall styras via UI-val, inte via textnycklar.

Tomma fält visas diskret, exempelvis som `—`. Obligatoriska fält måste vara ifyllda för att användaren skall kunna lämna redigeringsläge.

Detaljrutan öppnas först i visningsläge. Användaren kan växla till redigeringsläge, ändra värden och spara där. Första målbilden skall inte tillåta att användaren lämnar redigeringsläge medan obligatoriska fält saknar värden.

## Central diagramvy

Diagrammet är appens huvudvy. Det är inte en sekundär rapport, utan den primära arbetsytan för att förstå källarens innehåll.

### Grundprincip

- Varje objekt visas som en egen horisontell rad.
- X-axeln visar tid.
- Varje visat datumvärde visas som en punkt på objektets rad.
- Ett objekt får en sammanhängande linje från tidigaste till senaste visade datum.
- Alla datumfält visas som standard, men varje vy kan ändra vilka datumfält som visas.
- Nya datumfält ingår automatiskt i alla befintliga vyer.
- En vy kan inkludera och exkludera datumfält, men inte ändra datumfältens ordning.
- Datumfältens interna ordning vid samma datum styrs av global fältordning.
- Datum före och efter dagens datum behandlas inte visuellt olika i första målbilden.

Datumfält kan vara flervärdesfält. Om ett visat datumfält innehåller flera datum visas alla dessa datum som punkter på objektets rad.

Ett objekt utan visade datumfält skall ändå visas i diagrammet. Objektet får vanlig y-position enligt aktuell y-hierarki och sortering, men hela raden markeras med diskret röd ton och saknar datumlinje.

### Layout, scroll och axlar

Diagrammet skall ha fast radhöjd som kan ställas in globalt. Många objekt hanteras med vanlig vertikal scroll.

Tidsled hanteras med horisontell panorering eller scroll. Namnkolumnen är sticky till vänster och tidslinjen rör sig bakom kolumnen. Namnkolumnens bredd är en global inställning.

X-axeln skall ligga sticky längst ned i bild. Den skall alltså vara synlig även när användaren scrollar vertikalt i diagrammet.

Långa objektetiketter kapas med `…` i diagrammet. Full etikett visas vid tryck eller hover på radnamnet.

Det skall finnas en knapp som passar in aktuell visning så att alla synliga datum visas och inte överlappas av etikettkolumnen. Knappen påverkar bara aktuell visning för stunden och ändrar inte sparad vy eller global tidsinställning.

Första målbilden skall inte använda fri zoomgest. Tidszoom hanteras med de fasta globala tidsvalen `Visa allt`, `5 år`, `10 år` och `Egen period`. Det skall inte finnas minikarta eller översiktskarta i MVP.

Vid byte av vy återställs vertikal position till toppen. Horisontell tidsposition återställs enligt aktuell global tidsinställning, eftersom tidsintervall inte är vybundet i MVP.

### Datumpunkter och tooltip

Om samma objekt har flera punkter på exakt samma datum visas en punkt på datumet.

Vid tooltip, hover eller tryck på datumpunkt visas:

- objektets globala diagrametikett,
- datumfältets namn,
- datumvärdet,
- alla fält/värden som ligger på samma datum om flera sammanfaller.

Exempel:

```text
Cantillon Gueuze 2021
Tappdatum: 2022-03-18
Öppnas tidigast: 2026-03-18
```

Tryck på datumpunkt öppnar inte detaljpanelen i första målbilden. Datumpunkt används för tooltip eller markering.

### Radnamn och detaljpanel

Objektets radnamn är den globala diagrametiketten.

Radnamnet skall ligga låst vid diagrammets vänsterkant. För objekt med datum skall radnamnet ligga strax till vänster om objektets linje när det är möjligt inom den låsta vänsterytan.

Detaljpanelen öppnas endast genom tryck på objektets radnamn. Linje och datumpunkter öppnar inte detaljpanelen.

### Idag-linje och tidsinställning

Diagrammet skall ha en tydlig idag-linje på x-axeln. Idag-linjen visas bara om dagens datum ligger inom synligt tidsintervall.

Tidszoom skall i första målbilden styras med fasta globala tidsval: `Visa allt`, `5 år`, `10 år` och `Egen period`. Fri zoomgest ingår inte i MVP.

### Y-hierarki

Byggandet av y-hierarki är centralt och skall finnas direkt i huvudgränssnittet. Funktionen skall påminna om en förenklad pivot-tabell i Excel.

En sparad vy skall i första målbilden kunna använda noll till tre fält i y-hierarkin.

Exempel:

```text
Land → Region → Producent
Kategori → Årgång → Namn
Stil → Land → Datumfält
```

Sorteringsriktning skall kunna väljas per fält i hierarkin. Listfält sorteras enligt listans manuella ordning. Övriga fält sorteras naturligt enligt fälttyp och vald riktning. Internt id används alltid som sista sorteringsnyckel och kan inte stängas av.

Gruppetiketter från y-hierarkin visas som egna tunna rubrikrader i diagrammet. De visas även när gruppen bara innehåller ett objekt. Gruppetiketter räknas som egna rader, men har lägre höjd än objektrader. Endast grupper som innehåller minst ett synligt objekt visas.

Varje hierarkinivå får visuellt indrag i namnkolumnen. Tomma hierarkivärden samlas under gruppen `Saknar värde`.

När ett flervärdesfält används i y-hierarkin placeras objektet i MVP efter fältets första värde. Övriga värden syns i detaljvyn och kan användas i filter, men skapar inte egna hierarkigrupper.

Om datumfält används i y-hierarkin grupperar MVP efter år. Om talfält används i y-hierarkin grupperar MVP efter exakt värde. Intervallgruppering hålls öppen för senare.

Diagrammet skall i första målbilden inte ha expanderbara eller hopfällbara grupper, men modellen skall inte stänga den möjligheten.

### Filter

Filter i en vy skall döljas helt från diagrammet när objekt inte matchar. Nedtonade men synliga filtrerade objekt ingår inte i första målbilden.

Filter sparas som en lista med villkor där alla måste uppfyllas. Första målbilden innehåller bara AND-logik. Det skall inte finnas filtergrupper, OR-villkor eller avancerade filteruttryck.

En vy kan ha noll filter.

Filteroperatorer i första målbilden:

- Text: innehåller, är exakt.
- Tal: lika med, större än, mindre än, intervall.
- Datum: före, efter, mellan.
- Lista: är någon av valda.
- Tomhet: är tomt.

Textfilter skall inte vara skiftlägeskänsliga.

Filter skall kunna ha valet `inkludera tomma värden`, med standard av. Detta betyder att objekt med tomt värde i filterfältet får följa med tillsammans med objekt som matchar villkoret. Operatorn `är tomt` används när användaren explicit vill hitta objekt där ett fält saknar värde.

Listfilter skall kunna välja flera listvärden. För flervärdesfält räcker det att ett av objektets värden matchar.

Ogiltiga filter skall markeras och får inte sparas förrän användaren har rättat eller tagit bort dem. Exempel på ogiltigt filter är ett filter som inte längre är meningsfullt efter ändring av fältdefinition.

Alla fält utom bildfält skall kunna användas för filter, förutsatt att fälttypen har meningsfull filterlogik.

### Global diagrametikett

Diagrametiketten är global för källarprojektet och sparas inte per vy.

Diagrametiketten konfigureras med en fältlista i UI. Den skall inte konfigureras med malltext, Markdown, taggar eller textnycklar.

Regler:

- Diagrametiketten består av 1–5 fält i vald ordning.
- Appen använder fast separator, exempelvis ` – `.
- Tomma fält hoppas över.
- Separatorer städas automatiskt.
- När projektet saknar fält får diagrametiketten vara tom.
- När minst ett fält finns måste minst ett etikettfält alltid finnas.
- Sista etikettfältet kan inte tas bort förrän ett annat fält valts.
- Om alla etikettfält är tomma för ett objekt visas internt id diskret som fallback.

### Mobil och desktop

Mobil och desktop är lika viktiga. Diagrammet skall därför designas som en responsiv huvudvy, inte som en mobilvy med sekundär desktop-anpassning.

Samma grund-UI skall användas på mobil och desktop. Skillnaden skall främst ligga i skärmstorlek, överblick och skalning.

## Sparade vyer

Appen skall stödja flera sparade diagramvyer.

En sparad vy skall spara:

- y-hierarki/grupperingsfält,
- sorteringsriktning per grupperingsnivå,
- filter,
- val av synliga datumfält.

En sparad vy skall inte spara diagrametiketten, tidsintervall eller layoutinställningar, eftersom dessa är globala för projektet i MVP.

Vykontroller skall finnas direkt i huvudfönstret medan användaren ser diagrammet. Kontroller för redigering av vy skall vara alltid tillgängliga i huvudfönstret.

Huvudvyn skall ha en dropdown för att välja aktiv vy.

Funktioner för att skapa, duplicera, byta namn på, återställa och ta bort vyer skall ligga i huvudvyn. Det skall inte krävas en separat vy enbart för vyhantering.

En ny vy skall skapas med enkel standardkonfiguration:

- alla objekt visas,
- inga filter,
- ingen y-hierarki,
- ingen separat objektsortering,
- internt id används som sista och enda sorteringsnyckel,
- datumfält följer standardregeln och är inkluderade.

En vy skall kunna återställas till samma enkla standardkonfiguration.

Vynamn skall vara unika inom samma källarprojekt.

Y-hierarki i vykonfiguration väljs med tre dropdown-rader: `Nivå 1`, `Nivå 2` och `Nivå 3`. Varje vald nivå har egen sorteringsriktning. Tom nivå betyder att nivån inte används. MVP har ingen separat objektsortering utöver grupperingsfältens sorteringsriktning och internt id som sista sorteringsnyckel.

Datumfält inkluderas eller exkluderas med en checklista över alla datumfält. Nya datumfält är ikryssade som standard. En vy behöver inte ha något datumfält valt, eftersom objekt utan visade datum ändå visas med röd radton.

Det skall inte finnas ett särskilt standardvy-begrepp. Den senast använda vyn öppnas automatiskt. Om den saknas öppnas första vyn.

Det måste alltid finnas minst en sparad diagramvy. En vy kan tas bort även om den är aktiv, men appen byter då till första kvarvarande vy. Om det bara finns en vy får den inte tas bort förrän en ny vy skapats.

## Detaljvy och inmatning

Detaljpanelen skall öppnas ovanpå diagrammet. Den öppnas endast genom tryck på objektets radnamn.

Detaljpanelen öppnas i visningsläge. Användaren kan växla till redigeringsläge, ändra värden och spara där.

Redigeringsläge skall ha knapparna `Spara` och `Återställ`. `Återställ` återgår till senaste sparade värden. Ändringar sparas inte löpande.

Appen skall varna om användaren försöker stänga detaljpanelen med osparade ändringar.

Valideringsfel skall kunna visas direkt när de uppstår, men hård spärr sker vid `Spara` eller när användaren försöker lämna redigeringsläge.

Användaren skall inte kunna lämna redigeringsläge om obligatoriska fält saknar värden. Obligatoriska fält som saknar värde markeras med röd ton eller ram.

`Lägg till objekt` öppnar detaljpanelen i skapandeläge. Samma panel används för visning, redigering och skapande.

Formulär för nytt objekt skall visa alla globala fält i global fältordning. Nya objekt skapas med tomma värden i alla globala fält. Obligatoriska fält måste fyllas innan ett nytt objekt kan sparas.

Objekt skall kunna skapas och sparas utan datumfält, så länge obligatoriska fält är ifyllda. Sådana objekt visas som datumlösa rader i diagrammet med diskret röd ton.

Talfält skall kunna vara `heltal` eller `decimaltal` som underinställning till huvudtypen tal.

Fält skall inte grupperas visuellt i detaljvyn i början, men möjligheten skall hållas öppen.

## Global konfiguration

Det skall finnas en separat konfigurationspanel ovanpå huvudvyn för att administrera källarens globala konfiguration. Användaren lämnar därmed inte appens kontext, men diagrammet behöver inte samsas med fältadministration samtidigt.

Den globala konfigurationspanelen skall minst hantera:

- skapa fält,
- ändra fältnamn,
- välja fälttyp,
- välja ett värde eller flera värden,
- markera fält som obligatoriska,
- välja detaljformat där det är relevant,
- hantera listvärden,
- ändra listvärdens ordning,
- ändra global fältordning,
- konfigurera global diagrametikett,
- rensa ett fältvärde från alla objekt,
- ta bort fält enligt gällande regler.

Fältlistan i konfigurationspanelen skall visa:

- fältnamn,
- typ,
- obligatorisk status,
- enkelvärde eller flervärde,
- detaljformat,
- upp-/ner-knappar för fältordning.

Nya fält skapas från fältlistan via `Lägg till fält`. Användaren anger namn, typ, obligatorisk status, enkel-/flervärde och format där formatval är relevant.

Fältnamn skall vara unika inom samma källarprojekt. Eftersom fält väljs via UI-namn minskar det förvirring.

Om objekt redan finns skall ett nytt fält först skapas som icke-obligatoriskt. Fältet kan göras obligatoriskt när alla objekt har giltigt värde.

All radering skall kräva bekräftelse. MVP skall inte ha papperskorg; radering är permanent efter bekräftelse.

Ändringar som påverkar många objekt skall kräva extra bekräftelse. Exempel är att rensa ett fältvärde från alla objekt, göra ett fält obligatoriskt, ta bort ett fält eller genomföra andra breda ändringar.

Mallar för startkonfiguration, exempelvis öl/vin-mallar, hålls öppna men ingår inte i första målbilden.

## Fysisk placering

Fysisk placering i källaren skall behandlas som vanliga användardefinierade fält.

Exempel:

```text
Rum
Hylla
Sektion
Låda
Rad
```

Det skall inte finnas en särskild platsmodell i första målbilden. Fält som beskriver plats kan ändå användas för filtrering, sortering, detaljvy och y-hierarki som alla andra fält, om de inte är bildfält.

## Projektstart och grundkonfiguration

Ett nytt projekt skall kunna skapas helt tomt. Vid första start skapas ett tomt projekt automatiskt med ett tillfälligt namn, exempelvis `Min källare`, och en första vy, exempelvis `Alla objekt`.

Första vyn skall ha enkel standardkonfiguration:

- alla objekt visas,
- inga filter,
- ingen y-hierarki/gruppering,
- sortering endast efter internt id,
- datumfält följer standardregeln.

Användaren skall mötas av huvudvyn direkt. Om projektet saknar objekt visas en tom diagramyta med kort startinstruktion och primära val:

- `Skapa första fältet`,
- `Importera projekt`.

Det skall krävas minst ett fält innan objekt kan skapas på meningsfullt sätt. `Lägg till objekt` får vara synlig även när inga fält finns, men skall då visa ett tomläge som förklarar att minst ett fält måste skapas först och ge direkt väg till fältadministration.

Konfigurationspanelen skall kunna öppnas även när projektet saknar fält och objekt.

Projektet skall äga global fältordning, globala diagraminställningar, global diagrametikett, fält, listvärden och sparade vyer. Vyer skall inte skriva över globala layoutinställningar i MVP.

Globala diagraminställningar i MVP:

- radhöjd,
- namnkolumnbredd,
- global diagrametikett,
- globalt tidsintervall/zoomläge.

## Tidsintervall och x-axel

Standardtidsintervallet är `Visa allt`. Diagrammet beräknar då minsta och största synliga datum i aktuell vy och lägger på marginal.

Globala tidsval i MVP:

- `Visa allt`,
- `5 år`,
- `10 år`,
- `Egen period`.

`5 år` och `10 år` räknas från dagens datum och framåt. `Egen period` har användarvald start och slut.

Objekt med datum utanför synligt tidsintervall skall ligga kvar om de matchar vyfilter och gruppering. Datum utanför intervallet ritas inte. Om inget av objektets datum syns inom intervallet behandlas raden som utan visade datum och får diskret röd ton.

X-axeln växlar automatiskt mellan år, månad och dag beroende på synligt tidsintervall. Punkter placeras alltid efter exakt datum även när axeln visar grövre etiketter.

`Visa allt` skall lägga automatisk marginal, ungefär 5 procent av datumspannet på varje sida, med minsta marginal så att första och sista punkt inte hamnar precis vid kanten.

`Passa in alla datum` skall ta hänsyn till namnkolumnen så att tidigaste synliga datumpunkt hamnar till höger om namnkolumnen och inte bakom den.

Datumetiketter skall inte visas direkt på punkter. Datumfält och värden visas vid tryck eller hover.

## Inmatning och värderepresentation

Datumfält skall använda datumväljare men även acceptera manuell inmatning i formatet `YYYY-MM-DD`. Ogiltiga datum markeras direkt. Datum lagras som rena kalenderdatum utan tid och tidzon.

Talfält valideras enligt fältets undertyp: `heltal` eller `decimaltal`. Decimaltal får matas in med både komma och punkt i UI, men normaliseras internt. Decimaltal visas med lokal decimalform i UI.

Textfält har ingen användarstyrd maxlängd i MVP. Längre text hanteras genom detaljformatet `Längre textblock`, inte genom en separat textmall eller Markdown.

Tomt värde är inte samma sak som tom text. Tom text normaliseras till tomt värde. Textvärden trimmas från blanksteg i början och slut. Interna mellanslag påverkas inte.

Flervärdesfält kan inte innehålla tomma enskilda värden. Om en flervärdeslista är tom räknas hela fältet som tomt. Flervärden sorteras manuellt med upp-/ner-knappar.

Nya objekt får tomma värden i alla fält. Standardvärden för nya objekt ingår inte i MVP.

Listvärden väljs med dropdown för enkelvärde och multiselect/chips för flervärde. Listvärden visas i listans manuella ordning vid val, visning, sortering och gruppering.

Bild läggs till via filväljare. Kamera/fotoflöde hålls öppet för senare, men kan fungera indirekt om webbläsaren erbjuder kamera via filväljaren. Bild kan tas bort i redigeringsläge. Om bildfältet är obligatoriskt går det inte att spara utan ny bild. Bildbeskärning och komprimeringsinställningar ingår inte i MVP.

## Objektflöden

`Lägg till objekt` skall alltid finnas i huvudfönstret nära vy- och konfigurationskontrollerna. Funktionen öppnar samma detaljpanel som används för redigering, men i skapandeläge.

Efter att ett nytt objekt sparats läggs det direkt in i aktuell vy om det matchar vyns filter. Om objektet inte matchar aktuell vy visas en kort bekräftelse om att objektet sparats men inte syns i aktuell vy.

Nytt objekt kan skapas även när aktiv vy har filter.

`Duplicera` skall finnas i detaljpanelens meny. Det skapar ett nytt objekt med kopierade värden och nytt internt id, öppnat i redigeringsläge innan det sparas.

`Ta bort objekt` skall ligga i detaljpanelen och kräva bekräftelse.

## Huvudfönster och vyredigering

Huvudfönstret skall ha en fast toppbar. Toppbaren innehåller projektnamn, vy-dropdown, `Spara vy` när ändringar finns, `Lägg till objekt` och meny för projekt/konfiguration.

Vy-dropdown är alltid synlig. Övriga vyåtgärder ligger i en meny nära dropdownen:

- `Ny vy`,
- `Byt namn`,
- `Duplicera`,
- `Ta bort`,
- `Spara vy`.

Ändringar i vykonfiguration skall slå igenom direkt i diagrammet. Vyändringar blir permanenta först när användaren väljer `Spara vy`. Aktiv vy skall markeras diskret med exempelvis `Osparade ändringar` när den skiljer sig från senast sparad konfiguration.

Det finns ingen separat `Återställ vyändringar`. Användaren kan välja samma vy i dropdownen igen för att ladda om senast sparad version. Användaren kan byta vy direkt även om aktuell vy har osparade ändringar; osparade ändringar försvinner då.

Vyinställningar skall ligga i huvudfönstret, eftersom de påverkar aktuell vy direkt:

- `Filter` visas som knapp/sektion med antal aktiva filter, exempelvis `Filter (3)`.
- Filterlistan öppnas ovanpå eller bredvid diagrammet beroende på skärmbredd.
- Gruppering visas som kompakt rad med tre dropdowns och sorteringsriktning per vald nivå.
- `Datumfält` visas som knapp/sektion med antal valda datumfält, exempelvis `Datumfält (4/6)`.
- Datumfält redigeras med checklista.
- Tidsintervall ligger direkt synligt som dropdown.
- Vid `Egen period` visas start- och slutdatum.
- `Passa in alla datum` ligger direkt synligt nära tidsintervallkontrollen.

## Responsiv layout och tillgänglighet

Mobil och desktop skall använda samma kontroller och flöden. Mobil toppbar får vara kompakt med radbrytning eller horisontell scroll. Viktigast synligt först är vy, tidsintervall och `Lägg till objekt`.

Filter- och datumfältspaneler öppnas som bottenpanel eller helskärmspanel på mobil och som sidopanel på desktop. Funktionen skall vara densamma.

Detaljpanelen är modal på både mobil och desktop. På mobil blir den nästan helskärm. På desktop blir den en större panel ovanpå diagrammet.

Diagrammet skall använda hela återstående yta under toppbar och kontroller. Desktop skall få mer överblick genom större yta, inte genom annan struktur eller extra sidomenyer i MVP.

Appen skall kunna användas utan hover. All information som visas vid hover skall också kunna nås med tryck/klick.

Primära åtgärder skall vara möjliga med tangentbord på desktop. MVP-nivå är tab-navigering, Enter för val/öppna och Escape för att stänga paneler/modaler. Radnamnen i vänsterkolumnen är fokuserbara och Enter öppnar detaljpanelen. Datumpunkter behöver inte vara separat tangentbordsfokus i MVP.

Paneler och modaler skall låsa fokus när de är öppna tills de stängs.

MVP har inget krav på separat mörkt/ljust tema. Den skall använda ett tema med god kontrast.

## Fel, spärrar och beroenden

Ogiltig global diagrametikett skall normalt inte kunna uppstå. När projektet saknar fält får global diagrametikett vara tom. När minst ett fält finns måste global diagrametikett innehålla minst ett valt fält. Om ett ogiltigt läge ändå uppstår markeras etikettkonfigurationen som ogiltig och måste rättas.

Fält som används i vyer, filter, gruppering, inkluderade datumfält eller global etikett får inte tas bort. Fält får inte heller tas bort om något objekt har värde i fältet. Fälttyp får bara ändras om fältet är tomt på alla objekt och inte används i någon sådan konfiguration.

Spärrade åtgärder skall visa kort orsak, exempelvis `Fältet används i 2 vyer och global etikett`.

Fältadministrationen skall visa var ett fält används. Varje fält kan visa en enkel användningssammanfattning, exempelvis `Används i: etikett, 3 vyer, 12 objektvärden`. Sammanfattningen kan öppnas för en enkel detaljlista över vilka vyer och konfigurationer som använder fältet. Objekt listas inte individuellt i MVP; där räcker antal objekt med värde.

Bekräftelsen för att rensa ett fältvärde från alla objekt skall visa antal objekt som påverkas och att åtgärden inte kan ångras i MVP.

Listvärden skall visa användningsantal. Ett listvärde får inte tas bort om det används av objekt eller sparade filter. Användningsantalet förklarar varför borttagning kan vara spärrad.

Vyer med ogiltiga filter eller saknad konfiguration markeras i vy-dropdownen och måste rättas innan de kan sparas.

## Sökning

MVP skall ha fritextsökning i huvudfönstret. Sökningen söker i text- och listfält. Datum och tal ingår inte i fritextsökningen i MVP; de hanteras med filteroperatorer.

Sökning är inte skiftlägeskänslig. Sökningen sparas inte i vyn och påverkar inte sparad vykonfiguration.

När sökning är aktiv visas diskret status, exempelvis `Sökning aktiv`, och en knapp för att rensa sökningen. Sökning döljer objekt som inte matchar, på samma sätt som filter.

## Visuell status

Röd används i olika sammanhang:

- I diagrammet betyder diskret röd radton att objektet saknar visade datum.
- I formulär betyder röd ram eller ton valideringsfel.

Objektlinjer har samma grundutseende i MVP. Färgkodning efter fält hålls öppen för senare. Filter och sökning döljer objekt i stället för att tona ned dem.

Markerad rad eller markerad datumpunkt får diskret visuell markering tills användaren väljer något annat eller klickar bort.

`Idag` visas som tunn vertikal linje med etikett längst ned vid x-axeln, endast om dagens datum ligger inom synligt intervall.

## Export, import och lagring

MVP skall ha manuell export av hela källarprojektet som komplett projektfil. Exporten är till för backup och flytt, inte för dataanalys. Excel/CSV-export ingår inte i MVP.

MVP skall ha manuell import av en sådan projektfil. Import ersätter nuvarande projekt efter tydlig bekräftelse. Sammanfogning av projekt ingår inte i MVP.

Exportfilen skall innehålla bilder så att projektet kan återskapas utan externa filer.

Appen skall inte visa när projektet senast exporterades i MVP. Export/import placeras i projektsektionen i konfigurationspanelen.

MVP kräver persistent lagring. Appen skall som målprincip kunna fungera utan internet. Synk mellan flera enheter och flera samtidiga användare ingår inte i MVP.

Dataägarskap är ett uttalat mål: användaren skall kunna exportera hela projektet inklusive bilder.

## Intern datamodell på målnivå

Projekt, objekt, fält, listvärden och vyer skall ha stabila interna id:n. Objektets interna id är diskret synligt i detaljpanelen; övriga id:n visas normalt inte.

Objektvärden lagras som koppling mellan objekt och fält. Ett objekt består alltså av internt objekt-id plus en samling fältvärden kopplade till fält-id.

Flervärden lagras som ordnade listor. Även enkelvärdesfält kan i modellen betraktas som en lista med högst ett värde.

Vyer, filter, gruppering, datumfältsval och global etikett refererar till fält-id, inte fältnamn. Därför kan fält byta namn utan att vyer går sönder.

Datamodellen skall förberedas för flera projekt via `projectId`, även om MVP bara hanterar ett aktivt projekt.

## Namngivning i UI

UI använder `Objekt` för poster i projektet. Öl, vin, flaskor och liknande är användarens egna fältvärden eller framtida mallar, inte hårdkodade UI-begrepp.

UI använder `Projekt` eller projektnamnet. `Källarprojekt` används främst i dokumentation och datamodell.

`Vy` används för sparad diagramkonfiguration. `Fält` används för användardefinierade egenskaper. UI använder `Gruppering`; dokumentation kan använda `Y-hierarki`.

## MVP-gräns och acceptanskriterier

MVP skall vara en verkligt användbar app för katalogisering i liten skala, inte bara en teknisk prototyp. Den behöver inte vara polerad, men data skall kunna sparas, exporteras och återimporteras.

Design och utseende specificeras inte i detalj i blueprinten. Blueprinten skall beskriva layout, funktion och interaktion. Exakta färger, typografi och visuell stil hålls öppet till prototyp.

Prestandariktmärke: MVP skall fungera rimligt med minst 500 objekt, 50 fält och 10 sparade vyer. Detta är ett riktmärke, inte ett optimeringskrav. Bildlagring, storleksgränser och eventuell komprimering beslutas med arkitekturen.

Objektmodellen är godkänd när användaren kan skapa, redigera, duplicera och ta bort objekt som innehåller fälttyperna text, tal, datum, lista och bild.

Fältmodellen är godkänd när användaren kan skapa, ändra, sortera, göra obligatoriskt och ta bort fält enligt spärrreglerna. Fält skall kunna vara enkelvärde eller flervärde, utom bild som är enkelbild.

Huvuddiagrammet är godkänt när objekt visas som rader med sticky namnkolumn, sticky x-axel längst ned, datumlinjer med punkter, röd radton för objekt utan visade datum och detaljpanel via radnamn.

Vyfunktionen är godkänd när användaren kan skapa, byta, spara, byta namn, duplicera och ta bort vyer. En vy skall kunna lagra filter, 0–3 grupperingsfält, sorteringsriktningar och inkluderade datumfält.

Datahantering är godkänd när data finns kvar mellan sessioner och hela projektet kan exporteras/importeras som komplett projektfil inklusive bilder.

## Testdata och verifiering

Intern testdata skall finnas för utveckling och test, men inte visas som användarens startläge. Testdata får vara öl/vin-relaterad för att testa verkligt användningsfall, men datamodellen skall fortfarande vara generisk.

Testdata bör innehålla cirka 10–20 objekt och täcka text, tal, datum, listor, bild, flervärden, tomma värden, objekt utan datum, flera datum på samma dag och andra avsiktliga normalfallsbrott.

Blueprinten skall innehålla manuella acceptanstest för fält, objekt, vyer, diagram, filter och import/export innan den konsolideras till produktionsformat.

## Dokumentation och arbetsfiler

`blueprint.work.md` är primär målbeskrivning. Den skall beskriva målbild, begrepp, regler, UI-flöden och MVP-gräns så att dokumentet är läsbart utan att man behöver följa hela chatten.

`user-decisions.md` är beslutslogg. Den skall innehålla korta beslutspunkter i kronologisk eller tematisk ordning och behöver inte upprepa all förklaring från blueprinten.

`plan.md` skall bli konkret genomförandeplan när målet är stabilt. Planen skall inte införa nya mål.

Öppna frågor skall samlas i egen sektion. Beslut utanför MVP skall dokumenteras som senare möjligheter så att de inte råkar smyga in i MVP.

## Arkitekturkrav utan teknikval

Blueprinten skall beskriva krav på arkitekturen utan att välja teknikstack.

Arkitekturen måste stödja:

- användardefinierade fält,
- stabila interna id:n,
- persistent lagring,
- offline-princip,
- komplett export/import av projekt,
- mobil och desktop,
- diagram som central arbetsyta,
- framtida möjlighet till flera projekt,
- framtida möjlighet till synk utan att synk byggs i MVP,
- framtida möjlighet till flera diagramtyper utan att fler diagramtyper byggs i MVP.

Teknikvalet skall göras efter att MVP-målet är tillräckligt stabilt. Teknikvalet skall endast jämföra öppna och fria tekniker. Proprietära, slutna eller licensmässigt låsande alternativ skall inte vara förstahandskandidater.

Eftersom MVP saknar inloggning, synk och flera samtidiga användare skall server inte införas utan tydlig nytta.

MVP har en huvuddiagramtyp: tidslinjediagram med objekt på y-led och datum på x-led.

## Medvetet utanför första målbilden

Följande skall inte införas utan nytt beslut:

- inloggning,
- flera källare i faktisk första version,
- CSV/Excel-import och CSV/Excel-export,
- separat tabell- eller listvy,
- streckkodsskanning,
- extern databas över öl/vin,
- rekommendationslogik för bästa öppningsår,
- särskild öppna/förbruka-funktion,
- hårdkodade fält för antal, öppnade, förbrukade eller status,
- särskilt huvudbilds- eller extrabildsbegrepp,
- användarsynliga fältnycklar eller texttaggar,
- Markdown-baserad presentation,
- textbaserade etikettmallar,
- diagrametikett per vy,
- särskild platsmodell,
- expanderbara diagramgrupper,
- färgkodning efter listfält,
- startmallar,
- fältgrupper i detaljvyn,
- papperskorg eller ångra-modell,
- minikarta eller översiktskarta i diagrammet,
- fri zoomgest i tidsled,
- inaktivering av listvärden,
- direkt skapande av listvärden från objektredigering,
- synk mellan flera enheter,
- flera samtidiga användare,
- flera aktiva projekt i UI,
- bildbeskärning eller användarstyrda komprimeringsinställningar,
- `ja/nej` som egen fälttyp,
- längre anteckning som egen fälttyp,
- detaljerade designbeslut om färger, typografi och visuell stil.

Flera av dessa är önskvärda framtida möjligheter, men de skall inte styra första målbilden innan arkitektur och implementation beslutas.

## Öppna frågor inför konsolidering till blueprint.md

Dessa frågor behöver inte stoppa fortsatt blueprint-arbete, men bör hanteras innan `blueprint.work.md` konsolideras till produktionsformatet `blueprint.md` eller innan implementation påbörjas.

1. Är målbilden nu tillräckligt färdigformulerad för att konsolideras till `blueprint.md`?
2. Vilken öppet licensierad teknikstack skall väljas när målet är färdigt nog för arkitekturbeslut?
3. Vilken persistent lagringsmodell skall användas för offline-princip, bilder och komplett projektfil?
4. Hur skall responsiv diagraminteraktion verifieras praktiskt mellan mobil och desktop?
5. Vilken minsta uppsättning manuella acceptanstest skall skrivas innan prototyparbete?

## Rekommenderad nästa beslutspunkt

Målbilden är nu bredare och mer komplett än tidigare versioner. De centrala MVP-frågorna om objekt, fält, diagram, vyer, filter, import/export, lagring, startflöde och arkitekturkrav är tillräckligt styrande för att nästa arbete bör vara konsolidering.

Nästa beslut bör vara om `blueprint.work.md` skall konsolideras till ett renare `blueprint.md`, eller om vi först skall göra en sista kvalitetsgranskning av målbilden med fokus på konflikter, MVP-gräns och implementerbarhet.
