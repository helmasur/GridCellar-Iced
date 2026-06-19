# Format för denna fil

- Alla beslut skall kategoriseras och föras in under en för projektet lämplig rubrik, så att det blir lätt att hitta ett beslut i efterhand.
- Varje beslut skall inledas med beslutsdatum i formatet `YYYY-MM-DD`.
- Varje beslut skall formuleras som en kort mening och får inkludera ett kort syfte, så blir det lättare att förstå varför beslutet togs.

# Projektbeslut

## Projektform

- 2026-06-08: Projektet startas som ett repo utan git tills vidare.
- 2026-06-08: Assistenten hanterar projektfilerna under det inledande blueprint-arbetet.

## Produktmål

- 2026-06-08: Projektets initiala mål är att ta fram en arbetsblueprint för en mobilanpassad webapp.
- 2026-06-08: Webappens initiala syfte var att katalogisera flaskor, främst öl och vin, som ligger på lagring i en källare.
- 2026-06-13: Appens prioriterade uppgifter är först tidsbaserad visualisering av källarens innehåll, därefter stöd för öppningsbeslut och därefter inventarieförteckning.
- 2026-06-13: Systemet skall formuleras generellt, så att källarprojektet har användardefinierade objektfält snarare än en hårdkodad flaskmodell.
- 2026-06-13: Första målbilden omfattar en logisk källare, men arkitekturen skall hållas öppen för flera källare i framtiden.
- 2026-06-13: Appen skall vara lika viktig på mobil och i webbläsare på desktop.
- 2026-06-13: Arkitektur, teknikval och lagringsform skall beslutas först när målet är färdigformulerat.
- 2026-06-13: Första målbilden skall inte innehålla inloggning.

## Objektmodell

- 2026-06-13: Ett objekt skall kunna representera antingen en enskild fysisk enhet eller ett parti av flera enheter; flaskor är första användningsfallet men inte hårdkodat.
- 2026-06-13: Enda hårdkodade systemfältet skall vara objektets interna id.
- 2026-06-13: Objektets interna id skall vara diskret synligt för användaren.
- 2026-06-13: Objektets interna id skall alltid användas som sista sorteringsnyckel, så att två objekt inte får samma slutliga sorteringsposition.
- 2026-06-13: Antal, ursprungligt antal, öppnade, förbrukade, bild och liknande skall inte vara hårdkodade systemfält.
- 2026-06-13: Det skall inte finnas en särskild öppna- eller förbruka-funktion i första målbilden.
- 2026-06-13: Om användaren vill hantera antal och öppnade flaskor görs det med vanliga användardefinierade fält.
- 2026-06-13: Objekt skall kunna dupliceras; duplicering skapar ett nytt objekt med kopierade fältvärden men nytt internt id.

## Globala fält

- 2026-06-13: Fältdefinitioner skall vara globala inom samma källarprojekt.
- 2026-06-13: Användaren skall kunna skapa nya fält även efter att objekt redan finns, och befintliga objekt får då tomt värde i det nya fältet.
- 2026-06-13: Fält skall väljas i UI via fältnamn och internt fält-id, inte via användarsynliga nycklar eller texttaggar.
- 2026-06-13: Användarsynliga fältnycklar, taggar och textbaserade fältreferenser skall inte ingå i första målbilden.
- 2026-06-13: Alla globala fält skall visas automatiskt i detaljrutan.
- 2026-06-13: Fält skall inte kunna döljas från detaljrutan utan att tas bort från källarprojektet.
- 2026-06-13: Varje fält skall ha global ordning för detaljrutan.
- 2026-06-13: Fältordning skall ändras med upp- och nedknappar i konfigurationsvyn.
- 2026-06-13: Nya fält skall läggas sist i global fältordning.
- 2026-06-13: Fält skall kunna byta namn efter att de skapats.
- 2026-06-13: Fält skall kunna markeras som obligatoriska.
- 2026-06-13: Obligatorisk-markering kan ändras, men om ett fält görs obligatoriskt måste alla befintliga objekt ha giltigt värde innan ändringen sparas.
- 2026-06-13: Ett fält får bara tas bort om alla objekt har tomt värde i fältet och fältet inte används i någon konfiguration.
- 2026-06-13: Ett fälts typ får ändras endast om fältet saknar värden på alla objekt och inte används i någon konfiguration.
- 2026-06-13: Nya fält skall automatiskt vara tillgängliga för filter och y-hierarki i alla vyer.

## Fälttyper och värden

- 2026-06-13: Fälttyperna i första målbilden skall omfatta text, tal, datum, val från lista och bild.
- 2026-06-13: Datumvärden skall vara fullständiga datum i format motsvarande `2028-05-14`.
- 2026-06-13: Användaren skall välja om ett fält tillåter ett värde eller flera värden när fältet skapas.
- 2026-06-13: Ett fälts värdeläge, ett värde eller flera värden, får bara ändras om fältet är tomt på alla objekt.
- 2026-06-13: Alla fälttyper utom bild skall kunna vara flervärdesfält.
- 2026-06-13: Flervärdesfält skall redigeras som en ordnad lista eller chips med lägg till, ta bort och flytta upp/ned.
- 2026-06-13: När ett flervärdesfält används för sortering skall första värdet styra först, därefter andra värdet om första är lika, och så vidare.
- 2026-06-13: Det skall vara tydligt för användaren att inbördes ordning på värden i ett flervärdesfält kan ändras.
- 2026-06-13: Ett flervärdesfält kan vara obligatoriskt, vilket betyder att fältet måste innehålla minst ett värde.
- 2026-06-13: Filter kan användas på flervärdesfält.
- 2026-06-13: För flervärdesfält betyder filtervillkoret att minst ett av objektets värden måste matcha.

## Listfält

- 2026-06-13: Val-listor skall vara globala per fält och kunna ha manuell ordning.
- 2026-06-13: Listfält skall sorteras enligt listans egen ordning snarare än naturlig sortering.
- 2026-06-13: Listvärden skall kunna ändra ordning efter att de skapats.
- 2026-06-13: Listvärden skall kunna byta namn även om objekt använder dem.
- 2026-06-13: Listvärden får bara tas bort när de inte används av objekt och inte används i sparade filter.

## Bildfält

- 2026-06-13: Bildfält skall vara en vanlig fälttyp och inte ett systemfält.
- 2026-06-13: Ett källarprojekt kan ha noll, ett eller flera bildfält.
- 2026-06-13: Ett bildfält innehåller en bild per objekt i första målbilden.
- 2026-06-13: Bildfält följer global fältordning i detaljrutan.
- 2026-06-13: Tomma bildfält visas diskret.
- 2026-06-13: Bildfält skall inte användas i filter, sortering eller y-hierarki i första målbilden.
- 2026-06-13: Bildfält skall kunna vara obligatoriskt.

## Fältpresentation och detaljruta

- 2026-06-13: Detaljrutan skall visa alla globala fält automatiskt i global fältordning.
- 2026-06-13: Ett fält visas högst en gång i detaljrutan.
- 2026-06-13: Varje fält skall ha ett globalt detaljformat.
- 2026-06-13: Detaljformat skall väljas i fältadministrationen.
- 2026-06-13: Om ett fält bara kan ha ett format skall formatval inte visas för användaren.
- 2026-06-13: Formatval skall bara visa rimliga format för aktuell fälttyp.
- 2026-06-13: Tillgängliga detaljformat i första målbilden skall vara normal rad, rubrikrad, kompakt etikett/chip, längre textblock, bild, datum och tal, där de är relevanta.
- 2026-06-13: Markdown-presentation och textbaserade fältmallar skall inte ingå i första målbilden.
- 2026-06-13: Tomma fält skall visas diskret i detaljrutan, exempelvis som `—`.
- 2026-06-13: Obligatoriska fält måste vara ifyllda för att användaren skall kunna lämna redigeringsläge.
- 2026-06-13: Detaljrutan skall öppnas först i visningsläge och ha ett redigeringsläge.

## Central diagramvy

- 2026-06-08: Appens centrala vy skall vara ett diagram.
- 2026-06-13: Diagrammet skall vara huvudvyn och visa varje objekt som en egen horisontell rad.
- 2026-06-13: Diagrammets x-axel skall visa tid.
- 2026-06-13: Varje visat datumvärde skall visas som en punkt på objektets rad.
- 2026-06-13: Ett objekt skall få en sammanhängande linje från tidigaste till senaste visade datum.
- 2026-06-13: Flera punkter på samma datum för samma objekt visas som en punkt.
- 2026-06-13: Tooltip eller tryck på en datumpunkt skall visa objektets diagrametikett och alla datumfält/värden som ligger på datumet.
- 2026-06-13: Tooltip eller hover skall visa datumfältets namn och värde, exempelvis `Tappdatum: 2028-05-14`.
- 2026-06-13: Tryck på datumpunkt skall inte öppna detaljpanelen i första målbilden.
- 2026-06-13: Detaljpanelen skall bara öppnas genom tryck på objektets radnamn.
- 2026-06-13: Objektets radnamn skall vara global diagrametikett.
- 2026-06-13: Radnamnet skall ligga låst vid diagrammets vänsterkant.
- 2026-06-13: Objekt med datum skall ha radnamnet strax till vänster om sin linje när det är möjligt inom den låsta vänsterytan.
- 2026-06-13: Diagrammet skall ha en tydlig idag-linje på x-axeln.
- 2026-06-13: Idag-linjen visas bara om dagens datum ligger inom synligt tidsintervall.
- 2026-06-13: Datum före och efter dagens datum skall inte behandlas visuellt olika i första målbilden.
- 2026-06-13: Alla datumfält skall visas i en vy som standard, men vyn skall kunna ändra vilka datumfält som visas.
- 2026-06-13: Nya datumfält skall automatiskt ingå i alla befintliga vyer.
- 2026-06-13: En vy kan inkludera och exkludera datumfält, men inte ändra datumfältens ordning.
- 2026-06-13: Datumfältens interna ordning vid samma datum styrs av global fältordning.
- 2026-06-13: Datumfält kan vara flervärdesfält, och alla datumvärden visas som punkter om fältet visas i vyn.
- 2026-06-13: Objekt utan visade datumfält skall ändå ligga kvar på egen rad och markeras med diskret röd ton.
- 2026-06-13: Objekt utan visade datumfält skall sorteras enligt y-hierarki och sortering.
- 2026-06-13: Den röda tonen för objekt utan visade datumfält skall gälla hela raden.
- 2026-06-13: Huvuddiagrammet skall i första målbilden inte ha expanderbara eller hopfällbara grupper, men möjligheten skall hållas öppen.
- 2026-06-13: Diagrammet skall inledningsvis hantera mobil användning genom en kombination av zoom, panorering och hierarkisk avgränsning.
- 2026-06-13: Tidszoom skall i början nöja sig med en global inställning.

## Diagrametikett

- 2026-06-13: Diagrametiketten skall konfigureras globalt, inte per vy.
- 2026-06-13: Diagrametiketten skall konfigureras med en fältlista i UI, inte med malltext, Markdown, taggar eller textnycklar.
- 2026-06-13: Global diagrametikett skall kunna bestå av 1–5 fält i vald ordning.
- 2026-06-13: Appen skall använda fast separator mellan etikettfält, exempelvis ` – `.
- 2026-06-13: Tomma fält i diagrametiketten skall hoppas över och separatorer städas automatiskt.
- 2026-06-13: Global diagrametikett måste ha minst ett fält när projektet har minst ett fält; tomma projekt får sakna etikett.
- 2026-06-13: Om alla etikettfält är tomma för ett objekt skall internt id visas diskret som fallback.
- 2026-06-13: Det sista fältet i diagrametiketten skall inte kunna tas bort förrän ett annat fält valts.

## Y-hierarki, sortering och filter

- 2026-06-13: Byggandet av y-hierarki är centralt och skall finnas direkt i huvudgränssnittet.
- 2026-06-13: Y-hierarkin skall likna en förenklad pivot-tabell i Excel.
- 2026-06-13: En vy skall kunna använda noll till tre fält i y-hierarkin.
- 2026-06-13: Sorteringsriktning skall kunna väljas per fält i hierarkin.
- 2026-06-13: Ett objekt skall alltid ha en enda placering i y-hierarkin, även när fältet har flera värden.
- 2026-06-13: Filter skall döljas helt från diagrammet när de inte matchar vyns filter.
- 2026-06-13: Filter i en vy skall sparas som en lista med villkor där alla måste uppfyllas.
- 2026-06-13: En vy skall kunna ha noll filter.
- 2026-06-13: För textfält behövs filteroperatorerna innehåller och är exakt.
- 2026-06-13: För talfält behövs filteroperatorerna lika med, större än, mindre än och intervall.
- 2026-06-13: För datumfält behövs filteroperatorerna före, efter och mellan.
- 2026-06-13: För listfält behövs filteroperatorn är någon av valda.
- 2026-06-13: Bildfält skall inte användas i filter, sortering eller y-hierarki.
- 2026-06-13: Färgkodning baserad på exempelvis ölstil eller annat listfält är önskvärd senare men hålls utanför första målbilden.

## Sparade vyer

- 2026-06-13: Appen skall stödja flera sparade diagramvyer.
- 2026-06-13: En sparad vy skall spara hierarki/grupperingsfält, sorteringsriktning per grupperingsnivå, filter och val av synliga datumfält.
- 2026-06-13: En sparad vy skall inte spara diagrametiketten, eftersom diagrametiketten är global.
- 2026-06-13: Huvudvyn skall ha en dropdown för att välja aktiv vy.
- 2026-06-13: Funktioner för att skapa, duplicera, byta namn på och ta bort vyer skall ligga i en meny i huvudvyn, inte i en separat vy.
- 2026-06-13: Det skall inte finnas ett särskilt standardvy-begrepp.
- 2026-06-13: Den senast använda vyn skall öppnas automatiskt; om den saknas öppnas första vyn.
- 2026-06-13: Det måste alltid finnas minst en sparad diagramvy.
- 2026-06-13: En vy kan tas bort även om den är aktiv, men appen byter då till första kvarvarande vy.
- 2026-06-13: Om det bara finns en vy får den inte tas bort förrän en ny vy skapats.
- 2026-06-13: Temporär osparad vy skall inte vara ett särskilt begrepp i MVP om det inte senare behövs.

## Detaljvy och inmatning

- 2026-06-13: Detaljvyn skall öppnas ovanpå diagrammet.
- 2026-06-13: Detaljvyn skall öppnas endast genom tryck på objektets radnamn i diagrammet.
- 2026-06-13: Detaljvyn skall öppnas i visningsläge och kunna växlas till redigeringsläge.
- 2026-06-13: Användaren skall kunna ändra värden i redigeringsläget och spara där.
- 2026-06-13: Användaren skall inte kunna lämna redigeringsläge om obligatoriska fält saknar värden.
- 2026-06-13: `Lägg till objekt` skall öppna detaljpanelen i skapandeläge; samma panel används för visning, redigering och skapande.
- 2026-06-13: Formulär för nytt objekt skall visa alla globala fält i global fältordning.
- 2026-06-13: Nya objekt skapas med tomma värden i alla globala fält.
- 2026-06-13: Obligatoriska fält måste fyllas innan ett nytt objekt kan sparas.
- 2026-06-13: Objekt skall kunna sparas trots att datumfält saknas, så länge obligatoriska fält är ifyllda.
- 2026-06-13: Fält skall inte grupperas visuellt i detaljvyn i början, men möjligheten skall hållas öppen.

## Global konfiguration

- 2026-06-13: Det skall finnas en vy för att administrera källarens globala konfiguration.
- 2026-06-13: Den globala konfigurationsvyn skall hantera fält, fältnamn, fälttyp, obligatorisk-markering, listvärden, listordning, fältordning, detaljformat och global diagrametikett.
- 2026-06-13: Mobil och desktop skall använda samma grund-UI, med skillnad främst i överblick, yta och skalning.
- 2026-06-13: Startmallar för exempelvis öl/vin är en bra framtida idé men hålls utanför första målbilden.

## Fysisk placering

- 2026-06-13: Fysisk placering i källaren skall behandlas som vanliga användardefinierade fält och inte som ett särskilt systemkoncept.


## Diagramlayout och diagraminteraktion

- 2026-06-15: Diagrammet skall ha fast radhöjd som kan ställas in globalt.
- 2026-06-15: Diagrammet skall hantera många objekt med vertikal scroll.
- 2026-06-15: Diagrammet skall hantera tid i sidled med horisontell panorering eller scroll.
- 2026-06-15: Det skall finnas en knapp som passar in aktuell visning så att alla synliga datum visas och inte överlappas av etikettkolumnen.
- 2026-06-15: Passa-in-knappen skall bara påverka aktuell visning och inte ändra sparad vy eller global tidsinställning.
- 2026-06-15: Namnkolumnen skall vara låst till vänster och tidslinjen skall röra sig bakom kolumnen.
- 2026-06-15: X-axeln skall ligga sticky längst ned i bild.
- 2026-06-15: Långa objektetiketter skall kapas med `…` i diagrammet och full etikett visas vid tryck eller hover.
- 2026-06-15: Namnkolumnens bredd skall vara en global inställning.
- 2026-06-15: Första målbilden skall inte använda fri zoomgest, utan fasta zoomlägen eller global tidsinställning.
- 2026-06-15: Vid byte av vy skall vertikal position återställas till toppen och horisontell tidsposition följa aktuell global tidsinställning.
- 2026-06-15: Diagrammet skall inte ha minikarta eller översikt i MVP.
- 2026-06-15: Endast radnamnet skall öppna detaljpanelen; linje och punkter används för markering och tooltip.

## Y-hierarkins diagramlayout

- 2026-06-15: Gruppetiketter från y-hierarkin skall visas som egna tunna rubrikrader i diagrammet.
- 2026-06-15: Hierarkinivåer skall ha visuellt indrag i namnkolumnen.
- 2026-06-15: Tomma värden i y-hierarkin skall samlas under gruppen `Saknar värde`.
- 2026-06-15: När ett flervärdesfält används i y-hierarkin skall MVP placera objektet efter fältets första värde.
- 2026-06-15: Gruppetiketter skall visas även om gruppen bara innehåller ett objekt.
- 2026-06-15: Gruppetiketter skall räknas som egna rader, men ha lägre höjd än objektrader.
- 2026-06-15: Endast grupper som innehåller minst ett synligt objekt skall visas.
- 2026-06-15: Om datumfält används i y-hierarkin skall MVP gruppera efter år.
- 2026-06-15: Om talfält används i y-hierarkin skall MVP gruppera efter exakt värde.

## Detaljformat

- 2026-06-15: Formatet `Rubrikrad` skall visa fältets värde större och tydligare än normal rad, och fältnamnet kan döljas om värde finns.
- 2026-06-15: Formatet `Normal rad` skall visa fältnamn och värde i ett standardradformat.
- 2026-06-15: Formatet `Kompakt etikett/chip` skall visa fältnamn som rubrik och fältvärden som chips, ett chip per värde för flervärdesfält.
- 2026-06-15: Formatet `Längre textblock` skall visa fältnamn som rubrik och värdet som vanlig radbruten text utan Markdown.
- 2026-06-15: Formatet `Bild` skall visa bilden i fältets plats i detaljvyn, maxad till panelens bredd med bibehållen proportion.
- 2026-06-15: Tomt bildfält skall visas som diskret platshållare.

## Filtersemantik

- 2026-06-15: Filter skall kunna ha valet `inkludera tomma värden`, med standard av.
- 2026-06-15: Filter skall kunna använda operatorn `är tomt` för att hitta objekt där ett fält saknar värde.
- 2026-06-15: Textfilter skall inte vara skiftlägeskänsliga.
- 2026-06-15: Textfilter skall i MVP bara ha operatorerna `innehåller` och `är exakt`.
- 2026-06-15: Listfilter skall kunna välja flera listvärden och betyda att fältet är någon av valda.
- 2026-06-15: För flervärdesfält räcker det att ett av objektets värden matchar listfiltret.
- 2026-06-15: Ogiltiga filter skall markeras och måste rättas eller tas bort innan vyn kan sparas.

## Vykonfiguration

- 2026-06-15: Vykontroller skall finnas direkt i huvudfönstret medan användaren ser diagrammet.
- 2026-06-15: Kontroller för redigering av vy skall vara alltid tillgängliga i huvudfönstret.
- 2026-06-15: En ny vy skall visa alla objekt, ha inga filter, ingen y-hierarki och sorteras endast efter internt id.
- 2026-06-15: Vynamn skall vara unika inom samma källarprojekt.
- 2026-06-15: Y-hierarki i vykonfiguration skall väljas med tre dropdown-rader, `Nivå 1`, `Nivå 2` och `Nivå 3`, med sorteringsriktning per nivå.
- 2026-06-15: Datumfält skall inkluderas eller exkluderas i en vy med checklista över datumfält.
- 2026-06-15: Nya datumfält skall vara ikryssade som standard i befintliga vyer.
- 2026-06-15: En vy behöver inte ha något datumfält valt, eftersom objekt utan visade datum ändå visas med röd radton.

## Radering och återställning

- 2026-06-15: All radering skall kräva bekräftelse.
- 2026-06-15: MVP skall inte ha papperskorg; radering är permanent efter bekräftelse.
- 2026-06-15: Det skall finnas en administrativ funktion för att rensa ett fältvärde från alla objekt.
- 2026-06-15: En vy skall kunna återställas till enkel standardkonfiguration.
- 2026-06-15: Ändringar som påverkar många objekt skall kräva extra bekräftelse.

## Objektredigering och validering

- 2026-06-15: Ett nytt objekt skall kunna sparas utan datumfält.
- 2026-06-15: Redigeringsläge skall ha knapparna `Spara` och `Återställ`.
- 2026-06-15: `Återställ` skall återgå till senaste sparade värden.
- 2026-06-15: Appen skall varna om användaren försöker stänga detaljpanelen med osparade ändringar.
- 2026-06-15: Valideringsfel skall kunna visas direkt, men hård spärr sker vid `Spara` eller försök att lämna redigeringsläge.
- 2026-06-15: Talfält skall kunna vara `heltal` eller `decimaltal`.

## Fältadministration

- 2026-06-15: Fältadministration skall ligga i separat konfigurationspanel ovanpå huvudvyn.
- 2026-06-15: Fältlistan i konfigurationspanelen skall visa fältnamn, typ, obligatorisk status, enkel-/flervärde, detaljformat och upp-/ner-knappar.
- 2026-06-15: Nya fält skall skapas från fältlistan via `Lägg till fält`.
- 2026-06-15: Fältnamn skall vara unika inom samma källarprojekt.
- 2026-06-15: Om objekt redan finns skall ett nytt fält först skapas som icke-obligatoriskt; det kan göras obligatoriskt när alla objekt har giltigt värde.

## Listfält

- 2026-06-15: Listvärden skall administreras direkt på respektive listfält.
- 2026-06-15: I MVP skall nya listvärden inte kunna skrivas in direkt vid objektredigering; de skall först skapas i fältadministrationen.
- 2026-06-15: Listvärden skall inte kunna inaktiveras i MVP.
- 2026-06-15: Ett listfält får finnas med tom lista, men då kan inget värde väljas förrän listvärden skapats.
- 2026-06-15: Listvärdesnamn skall vara unika inom samma listfält, men får förekomma i olika listfält.

## Projektstart och projektinställningar

- 2026-06-16: Nytt källarprojekt skall kunna skapas helt tomt.
- 2026-06-16: Minst ett fält krävs innan objekt kan skapas på meningsfullt sätt.
- 2026-06-16: Innan datumfält finns visas objekt som rader utan datumlinjer med diskret röd ton.
- 2026-06-16: Tom vy utan objekt visar tom diagramyta med kort startinstruktion.
- 2026-06-16: Exempeldata och mallar ingår inte i MVP.
- 2026-06-16: Projekt skall ha namn.
- 2026-06-16: Globala diagraminställningar ligger på projektet.
- 2026-06-16: Vyer skriver inte över globala layoutinställningar i MVP.
- 2026-06-16: Projektet har global fältordning och äger fält, listvärden och sparade vyer.

## Tidsintervall och x-axel

- 2026-06-16: Standardtidsintervallet är `Visa allt`.
- 2026-06-16: `Visa allt` beräknar minsta och största synliga datum i aktuell vy och lägger på marginal.
- 2026-06-16: Globala tidsval i MVP är `Visa allt`, `5 år`, `10 år` och `Egen period`.
- 2026-06-16: `5 år` och `10 år` räknas från dagens datum och framåt.
- 2026-06-16: `Egen period` har användarvald start och slut.
- 2026-06-16: Objekt med datum utanför synligt tidsintervall ligger kvar om de matchar filter och gruppering.
- 2026-06-16: Om inget datum syns inom intervallet markeras objektets rad som utan visade datum.
- 2026-06-16: X-axeln växlar automatiskt mellan år, månad och dag beroende på synligt intervall.
- 2026-06-16: Punkter placeras alltid efter exakt datum även när axeln visar grövre etiketter.
- 2026-06-16: `Visa allt` använder ungefär 5 procent marginal på varje sida av datumspannet.
- 2026-06-16: `Passa in alla datum` tar hänsyn till namnkolumnen.
- 2026-06-16: Datumetiketter visas inte direkt på punkter utan vid tryck eller hover.

## Inmatning och värderepresentation

- 2026-06-16: Datum anges med datumväljare eller manuellt som `YYYY-MM-DD`.
- 2026-06-16: Ogiltiga datum markeras direkt.
- 2026-06-16: Talfält valideras som `heltal` eller `decimaltal`.
- 2026-06-16: Decimaltal kan matas in med komma eller punkt i UI men normaliseras internt.
- 2026-06-16: Textfält har ingen användarstyrd maxlängd i MVP.
- 2026-06-16: Längre text hanteras via detaljformatet `Längre textblock`.
- 2026-06-16: Flervärden sorteras med upp-/ner-knappar.
- 2026-06-16: Standardvärden för nya objekt ingår inte i MVP.
- 2026-06-16: Tomt värde är inte samma sak som tom text; tom text normaliseras till tomt värde.
- 2026-06-16: Flervärdesfält kan inte innehålla tomma enskilda värden.
- 2026-06-16: Textvärden trimmas från blanksteg i början och slut.
- 2026-06-16: Datum lagras som kalenderdatum utan tid och tidzon.

## List- och bildinmatning

- 2026-06-16: Listvärden väljs med dropdown för enkelvärde och multiselect/chips för flervärde.
- 2026-06-16: Listvärden visas i listans manuella ordning vid val, visning, sortering och gruppering.
- 2026-06-16: Bild läggs till via filväljare.
- 2026-06-16: Bild kan tas bort i redigeringsläge.
- 2026-06-16: Om bildfält är obligatoriskt kan objektet inte sparas utan bild.
- 2026-06-16: Bildbeskärning och komprimeringsinställningar ingår inte i MVP.

## Objektflöden

- 2026-06-16: `Lägg till objekt` skall alltid finnas i huvudfönstret.
- 2026-06-16: Nytt objekt öppnas i samma detaljpanel som redigering, men i skapandeläge.
- 2026-06-16: Efter sparande läggs objektet in i aktuell vy om det matchar filter.
- 2026-06-16: Om objektet inte matchar aktuell vy visas bekräftelse om att objektet sparats men inte syns.
- 2026-06-16: Duplicering finns i detaljpanelens meny och öppnar kopian i redigeringsläge innan sparande.
- 2026-06-16: `Ta bort objekt` finns i detaljpanelen och kräver bekräftelse.
- 2026-06-16: Nytt objekt kan skapas även om aktiv vy har filter.

## Huvudfönster och vyredigering

- 2026-06-16: Ändringar i vykonfiguration skall slå igenom direkt i diagrammet.
- 2026-06-16: Vyändringar kräver `Spara vy` för att bli permanenta.
- 2026-06-16: Det finns ingen separat `Återställ vyändringar`; användaren kan välja samma vy i dropdownen för att ladda om sparad version.
- 2026-06-16: Aktiv vy markeras med diskret status vid osparade ändringar.
- 2026-06-16: Användaren kan byta vy direkt även om aktuell vy har osparade ändringar; osparade ändringar försvinner.
- 2026-06-16: Huvudfönstret skall ha fast toppbar.
- 2026-06-16: Toppbaren innehåller projektnamn, vy-dropdown, `Spara vy` när ändringar finns, `Lägg till objekt` och meny för projekt/konfiguration.
- 2026-06-16: Globala diagraminställningar ligger i samma konfigurationspanel som fältadministration.
- 2026-06-16: Konfigurationspanelen har sektionerna `Fält`, `Diagram`, `Etikett` och `Projekt`.
- 2026-06-16: Global diagrametikett redigeras i konfigurationspanelen.
- 2026-06-16: Vyinställningar ligger i huvudfönstret.
- 2026-06-16: Vy-dropdown är alltid synlig och övriga vyåtgärder ligger i meny nära dropdownen.
- 2026-06-16: Filter visas som knapp/sektion med antal aktiva filter.
- 2026-06-16: Gruppering visas som kompakt rad med tre dropdowns och sorteringsriktning.
- 2026-06-16: Datumfält visas som knapp/sektion med antal valda datumfält och checklista.
- 2026-06-16: Tidsintervallkontroll och `Passa in alla datum` ligger direkt synligt i huvudfönstret.

## Responsiv layout och tillgänglighet

- 2026-06-16: Mobil och desktop använder samma kontroller och flöden.
- 2026-06-16: Mobil toppbar blir kompakt med radbrytning eller horisontell scroll.
- 2026-06-16: Viktigast synligt först är vy, tidsintervall och `Lägg till objekt`.
- 2026-06-16: Filter- och datumfältspaneler öppnas som bottenpanel/helskärm på mobil och sidopanel på desktop.
- 2026-06-16: Detaljpanelen är modal på både mobil och desktop.
- 2026-06-16: Diagrammet använder hela återstående yta under toppbar och kontroller.
- 2026-06-16: Desktop får mer överblick genom större yta, inte genom annan struktur.
- 2026-06-16: Appen skall kunna användas utan hover.
- 2026-06-16: Primära åtgärder skall vara möjliga med tangentbord på desktop med Tab, Enter och Escape.
- 2026-06-16: Radnamn i vänsterkolumnen är fokuserbara och Enter öppnar detaljpanelen.
- 2026-06-16: Datumpunkter behöver inte vara separat tangentbordsfokus i MVP.
- 2026-06-16: Paneler och modaler låser fokus tills de stängs.
- 2026-06-16: MVP har inget krav på mörkt/ljust tema men skall ha god kontrast.

## Fel, spärrar och beroenden

- 2026-06-16: `Lägg till objekt` utan fält leder till tomläge med förklaring och knapp till fältadministration.
- 2026-06-16: Ogiltig global diagrametikett skall normalt inte kunna uppstå, men om det sker måste etikettkonfigurationen rättas.
- 2026-06-16: Fält som används i vyer, filter, gruppering, inkluderade datumfält eller global etikett får inte tas bort.
- 2026-06-16: Fälttyp får bara ändras om fältet är tomt på alla objekt och inte används i vyer, filter, gruppering, inkluderade datumfält eller global etikett.
- 2026-06-16: Spärrade åtgärder skall visa kort orsak.
- 2026-06-16: Fältadministrationen skall visa var ett fält används.
- 2026-06-16: Användningssammanfattning kan öppnas för enkel detaljlista.
- 2026-06-16: Objekt listas inte individuellt i MVP, men antal objekt med värde visas.
- 2026-06-16: Rensa fältvärde från alla objekt kräver konsekvensbekräftelse.
- 2026-06-16: Listvärden visar användningsantal, inklusive användning i objekt och sparade filter.
- 2026-06-16: Vyer med ogiltiga delar markeras i vy-dropdownen och måste rättas innan de kan sparas.

## Intern datamodell

- 2026-06-16: Projekt har internt id och visningsnamn.
- 2026-06-16: Objekt, fält, listvärden och vyer har stabila interna id:n.
- 2026-06-16: Objektvärden lagras som koppling mellan objekt och fält.
- 2026-06-16: Flervärden lagras som ordnade listor.
- 2026-06-16: Vyer, filter, gruppering, datumfält och global etikett refererar till fält-id, inte fältnamn.
- 2026-06-16: Datamodellen skall förberedas för flera projekt via `projectId`, även om MVP bara hanterar ett aktivt projekt.

## Sökning och visuell status

- 2026-06-16: Fritextsökning finns i MVP och ligger i huvudfönstret.
- 2026-06-16: Sökning söker i text- och listfält men inte datum och tal.
- 2026-06-16: Sökning sparas inte i vyn.
- 2026-06-16: Sökning är inte skiftlägeskänslig.
- 2026-06-16: Aktiv sökning visar diskret status och knapp för att rensa.
- 2026-06-16: Röd ton används i diagram för objekt utan visade datum och i formulär för valideringsfel.
- 2026-06-16: Objektlinjer har samma grundutseende i MVP.
- 2026-06-16: Färgkodning efter fält hålls öppet för senare.
- 2026-06-16: Filter och sökning döljer objekt i stället för att tona ned dem.
- 2026-06-16: Markerad rad eller punkt får diskret visuell markering.
- 2026-06-16: `Idag` visas som tunn vertikal linje med etikett längst ned vid x-axeln när dagens datum ligger inom synligt intervall.

## Export, import och lagring

- 2026-06-16: MVP har manuell export av hela källarprojektet.
- 2026-06-16: Export är komplett projektfil, inte Excel/CSV.
- 2026-06-16: MVP har manuell import av komplett projektfil.
- 2026-06-16: Import ersätter nuvarande projekt efter tydlig bekräftelse.
- 2026-06-16: Sammanfogning av projekt ingår inte i MVP.
- 2026-06-16: Exportfilen innehåller bilder.
- 2026-06-16: Appen visar inte `senast exporterad` i MVP.
- 2026-06-16: Export/import ligger i projektsektionen i konfigurationspanelen.
- 2026-06-16: MVP kräver persistent lagring.
- 2026-06-16: Appen skall som målprincip fungera utan internet.
- 2026-06-16: Synk mellan flera enheter ingår inte i MVP.
- 2026-06-16: Flera samtidiga användare ingår inte i MVP.
- 2026-06-16: Dataägarskap är ett uttalat mål.

## Första användningsflöde och UI-termer

- 2026-06-16: Första start visar tom huvudvy med kort startinstruktion.
- 2026-06-16: Primära val är `Skapa första fältet` och `Importera projekt`.
- 2026-06-16: Ett tomt projekt skapas automatiskt vid första start, med tillfälligt namn exempelvis `Min källare`.
- 2026-06-16: Första vyn skapas automatiskt, exempelvis `Alla objekt`.
- 2026-06-16: Första vyn har ingen hierarki, inga filter och sorterar på internt id.
- 2026-06-16: Användaren tvingas inte konfigurera fält innan huvudvyn kan öppnas.
- 2026-06-16: Konfigurationspanelen kan öppnas även när projektet saknar fält och objekt.
- 2026-06-16: UI använder `Objekt`, `Projekt`, `Vy`, `Fält` och `Gruppering`.
- 2026-06-16: `Källarprojekt` används främst i dokumentation och datamodell.

## MVP-gräns, acceptans och test

- 2026-06-16: MVP skall vara användbar för verklig katalogisering i liten skala, inte bara en teknisk prototyp.
- 2026-06-16: Design/utseende specificeras inte i detalj nu; blueprinten beskriver layout, funktion och interaktion.
- 2026-06-16: MVP skall fungera rimligt med minst 500 objekt, 50 fält och 10 sparade vyer som riktmärke.
- 2026-06-16: Bildlagring, storleksgränser och eventuell komprimering beslutas med arkitekturen.
- 2026-06-16: Mallar, färgkodning efter fält, flera projekt, CSV/Excel och avancerade diagramfunktioner ingår inte i MVP.
- 2026-06-16: Objektmodellen är godkänd när objekt kan skapas, redigeras, dupliceras och tas bort med fälttyperna text, tal, datum, lista och bild.
- 2026-06-16: Fältmodellen är godkänd när fält kan skapas, ändras, sorteras, göras obligatoriska och tas bort enligt spärrregler.
- 2026-06-16: Huvuddiagrammet är godkänt när rader, sticky namnkolumn, sticky x-axel, datumlinjer, punkter, röd radton och detaljpanel fungerar.
- 2026-06-16: Vyfunktionen är godkänd när vyer kan skapas, bytas, sparas, döpas om, dupliceras och tas bort.
- 2026-06-16: Datahantering är godkänd när projektet sparas persistent och kan exporteras/importeras komplett inklusive bilder.
- 2026-06-16: Intern testdata skall finnas för utveckling/test men inte visas som användarens startläge.
- 2026-06-16: Testdata skall omfatta cirka 10–20 objekt och täcka normalfall samt kantfall.
- 2026-06-16: Blueprinten skall innehålla manuella acceptanstest före konsolidering.

## Dokumentation och arkitekturgräns

- 2026-06-16: `blueprint.work.md` är primär målbeskrivning.
- 2026-06-16: `user-decisions.md` är beslutslogg.
- 2026-06-16: `plan.md` blir konkret genomförandeplan när målet är stabilt.
- 2026-06-16: Öppna frågor samlas i egen sektion.
- 2026-06-16: Beslut utanför MVP dokumenteras som senare möjligheter.
- 2026-06-16: Blueprinten skall beskriva arkitekturkrav utan att välja teknikstack.
- 2026-06-16: Arkitekturen måste stödja användardefinierade fält, interna id:n, persistent lagring, offline-princip, komplett export/import, mobil/desktop och diagram som central arbetsyta.
- 2026-06-16: Framtida synk och flera diagramtyper hålls öppna men ingår inte i MVP.
- 2026-06-16: Teknikval görs efter att MVP-målet är tillräckligt stabilt.
- 2026-06-16: Teknikvalet skall bara jämföra öppna och fria tekniker.
- 2026-06-16: Server skall inte införas utan tydlig nytta, eftersom MVP saknar inloggning, synk och flera samtidiga användare.

## Medvetet uppskjutna beslut

- 2026-06-16: CSV/Excel-import och CSV/Excel-export hålls utanför MVP; komplett projektfil ingår i MVP.
- 2026-06-13: En separat tabell- eller listvy skall inte ingå i första målbilden, men arkitekturen skall hållas öppen för liknande funktioner.
- 2026-06-13: Ångra och papperskorg hålls öppna som framtida möjligheter, men ingår inte i MVP.
- 2026-06-13: Särskild platsmodell för källaren införs inte; fysisk placering hanteras som vanliga fält.
- 2026-06-13: Färgkodning efter listfält hålls öppen för framtiden.
- 2026-06-13: Huvudbild och extra bilder som särskilda systemkoncept införs inte; bilder hanteras som vanliga bildfält, ett bildfält per bild.
- 2026-06-13: Diagraminteraktion på mobil och desktop skall testas praktiskt innan implementation låses helt.

## Kvalitetsgranskning 2026-06-17

- 2026-06-17: `Ja/nej` ingår inte som egen fälttyp i MVP; behovet kan lösas med listfält och hålls som senare möjlighet.
- 2026-06-17: `Längre anteckning` ingår inte som egen fälttyp; längre text löses med textfält och detaljformatet `Längre textblock`.
- 2026-06-17: GridCellar skall beskrivas som en generisk katalog för källarlagrade objekt; flaskor, öl och vin är första användningsfall och testdata, inte hårdkodad modell.
- 2026-06-17: Global diagrametikett får vara tom endast när projektet saknar fält.
- 2026-06-17: När minst ett fält finns måste global diagrametikett ha minst ett valt fält.
- 2026-06-17: Objekt skapas i detaljpanelens skapandeläge, inte i separat lägg till-vy.
- 2026-06-17: Tidsintervall är globalt och inte vybundet i MVP.
- 2026-06-17: Vyer lagrar inte tidsintervall eller layoutinställningar.
- 2026-06-17: Tidszoomens MVP-val är `Visa allt`, `5 år`, `10 år` och `Egen period`; tidigare formuleringar om öppen zoommodell är ersatta.
- 2026-06-17: Fält får bara tas bort om de saknar objektvärden och inte används i någon konfiguration.
- 2026-06-17: Listvärden får bara tas bort om de inte används av objekt och inte används i sparade filter.
- 2026-06-17: MVP har ingen separat objektsortering utöver grupperingsfältens sorteringsriktning och internt id som sista sorteringsnyckel.
- 2026-06-17: Acceptanskriterierna skall bara nämna MVP-fälttyperna text, tal, datum, lista och bild.
- 2026-06-17: `Antal`, `Öppnade`, `Förbrukad`, `Status` och liknande skall vara vanliga användardefinierade fält; ingen särskild öppna/förbruka-funktion ingår i MVP.
- 2026-06-17: MVP har inget särskilt begrepp för huvudbild eller extrabilder; bild är en vanlig fälttyp och flera bilder löses med flera bildfält.

## Implementationsplanens nivå 2026-06-19

- 2026-06-19: `plan.md` skall endast nämna de två valda spåren, Svelte och Iced, i en inledande sektion.
- 2026-06-19: Efter inledningen skall `plan.md` formuleras teknikneutralt utifrån minsta gemensamma nämnare mellan spåren.
- 2026-06-19: Planen skall inte luta sig mot ramverksspecifika begrepp, komponentbibliotek, webbspecifika lösningar, native-specifika lösningar eller spårspecifika genvägar.
- 2026-06-19: Båda implementationerna skall fortfarande ha exakt samma produktmål och acceptanskriterier enligt `blueprint.md`.

## Beslut: planens nivå efter val av Svelte och Iced

- Planen får nämna Svelte- och Iced-spåren i en inledande sektion.
- Efter inledningen skall planen kunna följas som en fristående plan för en enskild implementation.
- Begrepp som kräver medvetenhet om ett parallellt spår, exempelvis “båda implementationerna”, skall inte förekomma efter inledningen.
- Planen skall vara formulerad på minsta gemensamma nivå och inte luta mot webbspecifika, native-specifika eller ramverksspecifika lösningar.

