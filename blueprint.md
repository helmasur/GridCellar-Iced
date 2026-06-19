# GridCellar blueprint

## 1. Syfte och produktidé

GridCellar är en mobil- och desktopanpassad webapp för att katalogisera källarlagrade objekt. Första användningsfallet är flaskor med öl och vin, men MVP-modellen är generisk: objekt beskrivs med användardefinierade fält och visas i ett tidslinjediagram.

Det primära värdet är en tidsbaserad visualisering där användaren kan se hur lagrade objekt fördelar sig över tid och hur de grupperas enligt användarens egna fält. Appen skall i första hand visualisera källarens innehåll, i andra hand hjälpa användaren avgöra när objekt bör användas eller öppnas, och i tredje hand fungera som inventarieförteckning.

Systemet skall inte låsas till en hårdkodad modell för öl, vin eller flaskor. Öl, vin och flaskor får användas som exempel och testdata, men skall inte hårdkodas i datamodellen.

## 2. MVP-målbild

Appen skall vara en webapp för både mobil och desktop. Mobil och desktop är lika viktiga och skall använda samma grund-UI. Skillnaden skall främst ligga i yta, överblick och skalning.

Första versionen omfattar ett aktivt källarprojekt och saknar inloggning. Datamodellen skall ändå förberedas för flera projekt senare genom stabila projekt-id:n.

MVP skall kräva persistent lagring och som målprincip kunna fungera utan internet. Synk mellan flera enheter och flera samtidiga användare ingår inte i MVP.

MVP skall vara en verkligt användbar app för katalogisering i liten skala, inte bara en teknisk prototyp. Appen behöver inte vara visuellt polerad, men data skall kunna sparas, exporteras och återimporteras.

Teknikval, lagringsform och arkitektur beslutas efter denna blueprint. Teknikval skall endast jämföra öppna och fria tekniker. Proprietära, slutna eller licensmässigt låsande alternativ skall inte vara förstahandskandidater.

## 3. Begrepp i UI och dokumentation

UI använder `Objekt` för poster i projektet. Öl, vin, flaskor och liknande är användarens egna fältvärden eller framtida mallar, inte hårdkodade UI-begrepp.

UI använder `Projekt` eller projektnamnet. `Källarprojekt` används främst i dokumentation och datamodell.

`Vy` används för sparad diagramkonfiguration. `Fält` används för användardefinierade egenskaper. UI använder `Gruppering`; dokumentation kan använda `Y-hierarki`.

## 4. Källarprojekt

Ett källarprojekt har internt id och visningsnamn. Visningsnamnet används i UI, medan internt id gör modellen redo för flera projekt senare.

Ett källarprojekt är den överordnade behållaren för objekt, globala fältdefinitioner, globala listor per listfält, sparade vyer, global fältordning, global detaljruteformatering per fält, global diagrametikett, globala diagraminställningar samt export- och importfunktioner.

Globala diagraminställningar ligger på projektet. Vyer skall inte skriva över globala layoutinställningar i MVP.

Projektet har global fältordning. Fältordningen styr både objektformulär och detaljpanel.

## 5. Objekt

Ett objekt är en post i källarprojektet. Objektet kan representera en enskild fysisk enhet eller ett parti av flera enheter. Första användningsfallet är flaskor och flaskpartier, men detta är inte hårdkodat i modellen.

Objektet skall inte ha hårdkodade domänfält som `namn`, `kategori`, `antal`, `öppnade`, `status`, `huvudbild` eller `plats`. Sådant skapas av användaren som globala fält.

Enda hårdkodade systemfältet är internt id. Det interna id:t skall vara stabilt, diskret synligt i detaljpanelen och alltid användas som sista sorteringsnyckel.

Om användaren vill hantera antal, ursprungligt antal, öppnade flaskor eller förbrukad status görs det med vanliga användardefinierade fält. MVP skall inte innehålla en särskild öppna- eller förbruka-funktion.

Objekt skall kunna skapas, visas, redigeras, dupliceras och tas bort. Duplicering skapar ett nytt objekt med kopierade fältvärden men nytt internt id.

## 6. Fältmodell

Fältdefinitioner är globala inom ett källarprojekt. Alla objekt använder samma uppsättning fält.

Användaren skall kunna skapa fält även när objekt redan finns. Befintliga objekt får då tomt värde i det nya fältet. Nya fält läggs sist i global fältordning och blir automatiskt tillgängliga i filter och gruppering.

Fält skall väljas i UI via fältnamn och internt fält-id. MVP skall inte använda användarsynliga fältnycklar, texttaggar eller textbaserade fältreferenser.

Varje fält har internt fält-id, visningsnamn, fälttyp, värdeläge, obligatorisk-markering, global ordning och globalt detaljformat när formatval är relevant.

Fält kan byta namn efter att de skapats. Eftersom vyer och värden kopplas via internt fält-id påverkar namnbyte bara visningen i UI.

Fältordning ändras med upp- och nedknappar i konfigurationsvyn. Samma enkla interaktion skall kunna användas på mobil och desktop.

Alla globala fält visas automatiskt i detaljrutan. Fält kan inte döljas från detaljrutan utan att tas bort från källarprojektet.

Ett fält får bara tas bort om fältet saknar värden på alla objekt och inte används i någon konfiguration, exempelvis global etikett, vy, filter, gruppering eller inkluderade datumfält.

Ett fälts typ får ändras endast om fältet saknar värden på alla objekt och inte används i någon konfiguration. Om obligatorisk-markering slås på måste alla befintliga objekt ha giltigt värde innan ändringen kan sparas.

Spärrade fältåtgärder skall visa orsak, exempelvis `Fältet används i global etikett och 2 vyer` eller `Fältet har värde på 14 objekt`.

## 7. Fälttyper

Fälttyper i MVP är text, tal, datum, val från lista och bild.

`Ja/nej` ingår inte som egen fälttyp i MVP. Vid behov kan detta lösas med listfält, exempelvis listvärdena `Ja` och `Nej`.

Längre anteckningar är inte en egen fälttyp. Längre text löses med textfält och detaljformatet `Längre textblock`.

Datum skall vara fullständiga kalenderdatum, exempelvis `2028-05-14`. Grova datum som endast år, år+månad eller intervall ingår inte i MVP.

Talfält har undertypen heltal eller decimaltal. Decimaltal får matas in med både komma och punkt i UI men skall normaliseras internt.

## 8. Värden och tomma värden

Tomt värde är inte samma sak som tom text. Tom text normaliseras till tomt värde. Textvärden trimmas i början och slut. Interna mellanslag påverkas inte.

Datum lagras som rena kalenderdatum i formatet `YYYY-MM-DD`, utan tid och utan tidzon.

Flervärdesfält kan inte innehålla tomma enskilda värden. Om värdelistan är tom räknas hela fältet som tomt.

Nya objekt skapas med tomma värden i alla globala fält. Obligatoriska fält måste fyllas innan objektet kan sparas eller innan användaren kan lämna redigeringsläge.

## 9. Flervärdesfält

När ett fält skapas väljer användaren om fältet tillåter ett värde eller flera värden. Detta värdeläge får bara ändras om fältet är tomt på alla objekt.

Alla fälttyper utom bild kan vara flervärdesfält.

Flervärdesfält redigeras som en ordnad lista eller chips där användaren kan lägga till värde, ta bort värde och flytta värde upp eller ned.

När ett flervärdesfält används för sortering jämförs värdena i användarens inbördes ordning: första värdet, därefter andra värdet om första är lika, och så vidare.

Ett flervärdesfält kan vara obligatoriskt. Då måste fältet innehålla minst ett värde.

Filter kan användas på flervärdesfält. För flervärdesfält betyder filtervillkoret att minst ett av objektets värden måste matcha.

Ett objekt skall alltid ha en enda placering i grupperingen. Ett objekt skall alltså inte dupliceras till flera grenar bara för att ett fält har flera värden. När ett flervärdesfält används i grupperingen placeras objektet efter första värdet i MVP. Övriga värden syns i detaljvyn och kan användas i filter, men skapar inte egna grupper.

## 10. Listfält

Val-listor är globala per fält. Exempel kan vara ölstilar, länder, druvor eller vad användaren själv väljer.

Listvärden har intern identitet och visningsnamn. Objekt refererar till listvärdets interna identitet, inte bara dess text.

Listvärden administreras direkt på respektive listfält i fältadministrationen. I MVP kan nya listvärden inte skrivas in direkt vid objektredigering; de skall först skapas i fältadministrationen.

Ett listfält får finnas med tom lista, men då kan användaren inte välja något värde förrän listvärden har skapats.

Listvärden skall kunna ändra ordning och byta namn även om objekt använder dem. Listvärden får bara tas bort om de inte används av objekt och inte används i sparade filter.

Listvärden skall inte kunna inaktiveras i MVP.

Listvärdesnamn skall vara unika inom samma listfält. Samma namn får däremot förekomma i olika listfält.

När listfält används för sortering skall listans manuella ordning användas i stället för naturlig sortering. Samma manuella ordning används vid val, visning, sortering och gruppering.

## 11. Bildfält

Bildfält är en vanlig fälttyp och inte ett systemfält. Ett källarprojekt kan ha noll, ett eller flera bildfält.

I MVP innehåller ett bildfält en bild per objekt. Det finns inte något särskilt systembegrepp för huvudbild eller extrabilder. Om användaren vill ha `Etikettbild`, `Källarplatsbild` eller `Extra bild` skapas separata bildfält.

Bildfält följer global fältordning i detaljrutan. Tomma bildfält visas diskret, exempelvis med diskret bildplatshållare eller `—`.

Bildfält skall inte användas i filter, sortering eller gruppering i MVP. Bildfält skall kunna vara obligatoriskt.

Bild läggs till via filväljare. Kamera/fotoflöde hålls öppet för senare men kan fungera indirekt om webbläsaren erbjuder kamera via filväljaren.

Bild kan tas bort i redigeringsläge. Om bildfält är obligatoriskt kan objektet inte sparas utan bild.

MVP skall inte innehålla bildbeskärning eller användarstyrda komprimeringsinställningar. Teknisk bildlagring och eventuell komprimering beslutas med arkitekturen.

## 12. Detaljruta och fältpresentation

Detaljrutan visar automatiskt alla globala fält i global fältordning. Ett fält visas högst en gång.

Varje fält skall ha ett globalt detaljformat. Formatet anges i fältadministrationen och gäller för fältets presentation i detaljrutan.

Om ett fält bara kan ha ett rimligt format skall användaren inte behöva välja format. Om flera format är rimliga visas bara relevanta format för aktuell fälttyp.

Detaljformat i MVP är normal rad, rubrikrad, kompakt etikett/chip, längre textblock, bild, datum och tal.

`Rubrikrad` visar värdet större och tydligare än normal rad. Fältnamnet kan döljas om värde finns. Om värde saknas visas fältnamn och diskret tomvärde.

`Normal rad` är standardrad med fältnamn och värde. Layouten får anpassas efter skärmbredd men skall vara enkel och konsekvent.

`Kompakt etikett/chip` visar fältnamn som rubrik och värden som chips. För flervärdesfält visas ett chip per värde.

`Längre textblock` visar fältnamn som rubrik och värdet som vanlig radbruten text. Markdown ingår inte i MVP.

`Bild` visar bilden på fältets plats i detaljvyn, maxad till panelens bredd med bibehållen proportion. Tomt bildfält visas som diskret platshållare.

`Datum` visar datum som fullständigt datum. `Tal` visar tal enligt fältets talinställning, heltal eller decimaltal.

Markdown-presentation, textbaserade mallar och fälttaggar skall inte ingå i MVP. Presentation skall styras via UI-val, inte via textnycklar.

Tomma fält visas diskret, exempelvis som `—`. Obligatoriska fält måste vara ifyllda för att användaren skall kunna lämna redigeringsläge.

Detaljrutan öppnas först i visningsläge. Användaren kan växla till redigeringsläge, ändra värden och spara där. Redigeringsläge har `Spara` och `Återställ`. `Återställ` återgår till senaste sparade värden.

Appen skall varna om användaren försöker stänga detaljpanelen med osparade ändringar. Valideringsfel kan visas direkt, men hård spärr sker vid `Spara` eller när användaren försöker lämna redigeringsläge.

## 13. Huvuddiagram

Diagrammet är appens huvudvy. Det är inte en sekundär rapport, utan den primära arbetsytan för att förstå källarens innehåll.

Varje objekt visas som en egen horisontell rad. X-axeln visar tid. Varje visat datumvärde visas som en punkt på objektets rad. Ett objekt får en sammanhängande linje från tidigaste till senaste visade datum.

Alla datumfält visas som standard, men varje vy kan ändra vilka datumfält som visas. Nya datumfält ingår automatiskt i alla befintliga vyer. En vy kan inkludera och exkludera datumfält, men inte ändra datumfältens ordning.

Datumfältens interna ordning vid samma datum styrs av global fältordning. Datum före och efter dagens datum behandlas inte visuellt olika i MVP.

Objekt utan visade datumfält visas ändå som egen rad, sorteras enligt gruppering och sortering och markeras med diskret röd radton. Exkludering av datumfält påverkar bara vilka datum som ritas, inte vilka objekt som finns i vyn.

Objektets radnamn ligger låst vid diagrammets vänsterkant. Tidslinjen rör sig bakom namnkolumnen. Långa etiketter kapas med `…`. Full etikett visas vid tryck eller hover.

Namnkolumnens bredd är globalt inställbar. Radhöjd är globalt inställbar. Gruppetiketter får lägre höjd än objektrader.

Diagrammet har vertikal scroll och horisontell panorering/scroll. X-axeln ligger sticky längst ned i bild. X-axeln växlar automatiskt mellan år, månad och dag beroende på synligt tidsintervall. Punkter placeras alltid efter exakt datum även om axeln visar grövre etiketter.

Ingen fri zoomgest ingår i MVP. Tidsvisning styrs med globala tidsval och horisontell panorering. Det finns ingen minikarta i MVP.

`Idag` visas som tunn vertikal linje med etikett längst ned vid x-axeln endast om dagens datum ligger inom synligt intervall.

Datumetiketter visas inte direkt på punkter. Vid tryck eller hover på punkt visas tooltip. Tooltip visar först objektets etikett, därefter datumfältets namn och datumvärde. Om flera fält eller värden ligger på samma datum visas en punkt och tooltip visar alla fält/värden där.

Endast radnamnet öppnar detaljpanelen. Linje och punkter används för markering och tooltip, inte navigation.

Markerad rad eller markerad datumpunkt får diskret visuell markering tills användaren väljer något annat eller klickar bort.

## 14. Global diagrametikett

Global diagrametikett används som radnamn i diagrammet. Etiketten är global för projektet och inte per vy.

Etiketten byggs i UI som en fältlista, inte som textmall, Markdown eller taggformat. Användaren väljer 1–5 fält i ordning. Appen använder fast separator, exempelvis ` – `.

Tomma etikettfält hoppas över och separatorer städas automatiskt. Om alla etikettfält är tomma för ett objekt används objektets interna id som fallback.

Global diagrametikett måste ha minst ett valt fält när projektet har minst ett fält. När projektet saknar fält får global diagrametikett vara tom. När sista etikettfältet är valt kan det inte tas bort förrän ett annat fält valts.

Global diagrametikett redigeras i konfigurationspanelen.

## 15. Gruppering och y-led

Gruppering byggs direkt i huvudfönstret och påminner om en förenklad pivot-tabell.

En vy kan ha 0–3 grupperingsfält. Varje grupperingsfält har egen sorteringsriktning. Om inga grupperingsfält väljs sorteras objekten direkt enligt internt id.

Vyn har ingen separat objektsortering utöver grupperingsfältens sortering och internt id som sista sorteringsnyckel.

Gruppetiketter visas som egna tunna rubrikrader i diagrammet. Varje hierarkinivå får visuellt indrag i namnkolumnen. Objektetiketten hamnar efter sista hierarkinivån.

Gruppetiketter visas även om gruppen bara innehåller ett objekt. Endast grupper som innehåller minst ett synligt objekt visas. Tomma värden i grupperingen samlas under gruppen `Saknar värde`.

Om flervärdesfält används i grupperingen används endast första värdet för gruppplacering i MVP.

Om datumfält används i grupperingen grupperar MVP endast efter år. Talfält kan användas i grupperingen och grupperas efter exakt värde i MVP.

Grupper kan inte fällas ihop eller expanderas i MVP, men arkitekturen skall inte blockera detta senare.

## 16. Filter och sökning

Filter sparas i vyer som en lista med villkor där alla villkor måste uppfyllas. MVP använder endast AND-logik. Inga filtergrupper, OR-villkor eller avancerade filteruttryck ingår.

En vy kan ha noll filter. Filtrerade objekt döljs helt, inte tonas ned.

Filteroperatorer i MVP är:

- text: `innehåller`, `är exakt`, `är tomt`, samt möjlighet att inkludera tomma värden,
- tal: `lika med`, `större än`, `mindre än`, `intervall`, `är tomt`, samt möjlighet att inkludera tomma värden,
- datum: `före`, `efter`, `mellan`, `är tomt`, samt möjlighet att inkludera tomma värden,
- lista: `är någon av valda`, `är tomt`, samt möjlighet att inkludera tomma värden.

Textfilter är inte skiftlägeskänsliga.

För flervärdesfält betyder filtervillkoret att minst ett av objektets värden matchar. `Är tomt` betyder att fältet saknar värden.

Listfilter kan välja flera listvärden. Ett listfilter betyder att fältet är någon av valda värden. För flervärdesfält räcker det att ett av objektets värden matchar.

Ogiltiga filter markeras och måste rättas innan vyn kan sparas.

MVP skall också ha fritextsökning i huvudfönstret. Sökningen söker i text- och listfält. Datum och tal ingår inte i fritextsökningen i MVP. Sökning är inte skiftlägeskänslig. Sökningen sparas inte i vyn och påverkar inte sparad vykonfiguration.

När sökning är aktiv visas diskret status, exempelvis `Sökning aktiv`, och en knapp för att rensa sökningen. Sökning döljer objekt som inte matchar, på samma sätt som filter.

## 17. Sparade vyer

En vy är en sparad diagramkonfiguration. Vynamn måste vara unika inom samma källarprojekt.

En vy sparar filter, 0–3 grupperingsfält, sorteringsriktning per grupperingsnivå och vilka datumfält som är inkluderade.

En vy sparar inte global diagrametikett, radhöjd, namnkolumnbredd eller tidsintervall.

Det måste alltid finnas minst en sparad vy. Det finns inget särskilt standardvy-begrepp; senast använda vy öppnas automatiskt. Om den saknas öppnas första vyn.

Ny vy skapas med enkel grundmodell: visar alla objekt, inga filter, ingen gruppering, sortering endast efter internt id och datumfält enligt standardregeln.

En vy kan tas bort även om den är aktiv. Appen byter då till första kvarvarande vy. Om det är sista vyn får den inte tas bort förrän en ny vy skapats.

Vyändringar slår igenom direkt i diagrammet men blir permanenta först när användaren väljer `Spara vy`. Aktiv vy markeras med diskret status, exempelvis `Osparade ändringar`, när det finns osparade vyändringar.

Ingen separat `Återställ vyändringar` behövs i MVP. Användaren kan välja samma vy i dropdownen för att ladda om senast sparad version. Användaren kan byta vy direkt även om aktuell vy har osparade ändringar; osparade ändringar försvinner då.

`Återställ vy` sätter vyn till enkel standardkonfiguration: alla objekt, inga filter, ingen gruppering, sortering efter internt id och alla datumfält inkluderade.

Vyer med ogiltiga filter eller saknad konfiguration markeras i vy-dropdownen och måste rättas innan de kan sparas.

## 18. Tidsintervall och x-axel

Tidsintervall är globalt, inte vybundet. Vyer lagrar inte tidsintervall eller layoutinställningar.

Standardtidsintervallet är `Visa allt`. Diagrammet beräknar minsta och största synliga datum i aktuell vy och lägger på automatisk marginal, cirka 5 % av datumspannet på varje sida, med minsta marginal så att första och sista punkt inte hamnar precis vid kanten.

Globala tidsval i MVP är `Visa allt`, `5 år`, `10 år` och `Egen period`.

`5 år` och `10 år` räknas från dagens datum och framåt. `Egen period` har användarvald start och slut.

Objekt med datum utanför synligt tidsintervall ligger kvar om de matchar filter och gruppering, men datum utanför intervallet syns inte. Om inget av objektets datum syns inom intervallet behandlas raden som utan visade datum och får diskret röd ton.

Vid vybyte återställs vertikal position till toppen och horisontell tidsposition enligt aktuell global tidsinställning.

`Passa in alla datum` påverkar bara aktuell visning, inte sparad vy eller global inställning. Knappen skall ta hänsyn till namnkolumnen så att tidigaste synliga datumpunkt hamnar till höger om namnkolumnen, inte bakom den.

## 19. Huvudfönster och kontrollstruktur

Huvudfönstret har fast toppbar. Toppbaren innehåller projektnamn, vy-dropdown, `Spara vy` när ändringar finns, `Lägg till objekt` och meny för projekt/konfiguration.

Vy-dropdown är alltid synlig. Övriga vyåtgärder ligger i meny nära dropdownen: `Ny vy`, `Byt namn`, `Duplicera`, `Ta bort`, `Spara vy`.

Vykontroller finns direkt i huvudfönstret medan användaren ser diagrammet. Redigeringskontroller för vy är alltid tillgängliga.

Filter visas som knapp eller sektion med antal aktiva filter, exempelvis `Filter (3)`. Filterlistan öppnas ovanpå eller bredvid diagrammet beroende på skärmbredd.

Gruppering visas som kompakt rad med tre dropdowns: `Nivå 1`, `Nivå 2`, `Nivå 3`, plus sorteringsriktning per vald nivå.

Datumfält visas som knapp eller sektion med antal valda datumfält, exempelvis `Datumfält (4/6)`. När den öppnas visas checklista.

Tidsintervall ligger direkt synligt som dropdown. `Egen period` visar start- och slutdatum. `Passa in alla datum` ligger direkt synligt nära tidsintervallkontrollen.

Konfigurationspanelen har sektionerna `Fält`, `Diagram`, `Etikett` och `Projekt`. Globala diagraminställningar och global diagrametikett ligger i konfigurationspanelen. Vyinställningar ligger i huvudfönstret.

## 20. Objektflöden

`Lägg till objekt` finns alltid i huvudfönstret. Om inga fält finns leder knappen till ett tomläge som förklarar att minst ett fält måste skapas först och ger direkt knapp till fältadministration.

När fält finns öppnar `Lägg till objekt` detaljpanelen i skapandeläge. Samma panel används för visning, redigering och skapande.

Skapandeläget visar alla globala fält i global ordning. Objektet sparas först när användaren väljer `Spara`.

Nytt objekt kan sparas utan datumfält, så länge obligatoriska fält är ifyllda.

Efter sparande läggs objektet in i aktuell vy om det matchar vyns filter. Om det inte matchar visas en kort bekräftelse om att objektet sparades men inte syns i aktuell vy.

Duplicering finns i detaljpanelens meny. Duplicering skapar ett nytt objekt med kopierade värden och nytt internt id, öppnat i redigeringsläge innan det sparas.

`Ta bort objekt` finns i detaljpanelen och kräver bekräftelse.

Nytt objekt kan skapas även om aktiv vy har filter.

## 21. Fältadministration och projektkonfiguration

Fältadministration ligger i separat konfigurationspanel ovanpå huvudvyn. Användaren lämnar inte appens kontext, men huvuddiagrammet behöver inte samsas med fältadministration samtidigt.

Fältlistan visar fältnamn, typ, obligatorisk status, enkelvärde/flervärde, detaljformat och upp-/nedknappar.

Nya fält skapas via `Lägg till fält`. Användaren anger namn, typ, obligatorisk status, enkelvärde/flervärde och format där formatval är relevant.

Fältnamn måste vara unika inom samma källarprojekt.

Om objekt redan finns kan ett nytt fält inte skapas direkt som obligatoriskt. Fältet skapas först som icke-obligatoriskt och kan göras obligatoriskt när alla objekt har giltigt värde.

Fältadministrationen skall visa var ett fält används. Varje fält kan visa en enkel användningssammanfattning, exempelvis `Används i: etikett, 3 vyer, 12 objektvärden`. Sammanfattningen kan öppnas för en enkel detaljlista över vilka vyer och konfigurationer som använder fältet. Objekt listas inte individuellt i MVP; antal objekt med värde räcker.

Det finns administrativ funktion för att rensa ett fältvärde från alla objekt. Bekräftelsen skall visa antal objekt som påverkas och att åtgärden inte kan ångras i MVP.

All radering kräver bekräftelse. Ändringar som påverkar många objekt kräver extra bekräftelse. Ingen papperskorg eller ångra-modell ingår i MVP; radering är permanent efter bekräftelse.

Listvärden skall visa användningsantal. Det förklarar varför ett listvärde inte kan tas bort.

## 22. Responsiv layout och tillgänglighet

Mobil och desktop använder samma kontroller och flöden. Mobil toppbar blir kompakt med radbrytning eller horisontell scroll. Viktigast synligt först är vy, tidsintervall och `Lägg till objekt`.

Filter- och datumfältspaneler öppnas som bottenpanel eller helskärm på mobil och sidopanel på desktop. Funktionen är densamma, bara presentationen anpassas.

Detaljpanelen är modal på både mobil och desktop. På mobil blir den nästan helskärm. På desktop blir den en större panel ovanpå diagrammet. Samma innehåll och knappar används i båda.

Diagrammet använder hela återstående yta under toppbar och kontroller. Desktop får mer överblick genom större yta, inte genom annan struktur.

Appen skall kunna användas utan hover. All information som visas vid hover skall också kunna nås med tryck/klick. Hover är bara en förbättring på desktop.

Primära åtgärder skall vara möjliga med tangentbord på desktop. Tab, Enter och Escape räcker som tangentbordsnivå i MVP. Radnamnen i vänsterkolumnen är fokuserbara och Enter öppnar detaljpanelen. Datumpunkter behöver inte vara separat tangentbordsfokus i MVP.

Paneler och modaler låser fokus tills de stängs.

MVP har inget krav på mörkt/ljust tema. MVP skall använda ett tema med god kontrast.

## 23. Projektstart och tomlägen

Ett tomt projekt skapas automatiskt vid första start, med tillfälligt namn, exempelvis `Min källare`. Användaren kan byta namn i projektinställningar.

Första vyn skapas automatiskt, exempelvis `Alla objekt`. Den har ingen gruppering, inga filter och sorterar på internt id.

Första start visar tom huvudvy med kort startinstruktion och tydliga primära val: `Skapa första fältet` och `Importera projekt`.

Användaren tvingas inte konfigurera fält innan huvudvyn kan öppnas. Konfigurationspanelen kan öppnas även när projektet saknar fält och objekt.

Minst ett fält krävs innan objekt kan skapas. Om inga fält finns visas i stället en uppmaning att skapa första fältet i konfigurationen.

Innan datumfält finns visas objekt som rader utan datumlinjer, markerade med diskret röd ton.

Ingen exempeldata ingår i användarens startläge. Mallar och exempeldata för användare hålls öppna för senare.

## 24. Fysisk placering

Fysisk placering hanteras som vanliga globala fält. Appen skall inte ha en särskild platsmodell i MVP.

Om användaren vill beskriva plats kan fält som `Rum`, `Hylla`, `Sektion`, `Låda` eller `Rad` skapas på samma sätt som andra fält. Dessa fält kan användas i filter, gruppering och detaljvyn enligt vanliga regler.

## 25. Visuell status

Röd används i olika sammanhang. I diagrammet betyder diskret röd radton att objektet saknar visade datum. I formulär betyder röd ram eller ton valideringsfel.

Objektlinjer har samma grundutseende i MVP. Färgkodning efter fält hålls öppen för senare.

Filter och sökning döljer objekt i stället för att tona ned dem.

## 26. Export, import och lagring

MVP skall ha manuell export av hela källarprojektet som komplett projektfil. Exporten är till för backup och flytt, inte för dataanalys. Excel/CSV-export ingår inte i MVP.

MVP skall ha manuell import av sådan projektfil. Import ersätter nuvarande projekt efter tydlig bekräftelse. Sammanfogning av projekt ingår inte i MVP.

Exportfilen skall innehålla bilder så att projektet kan återskapas utan externa filer.

Appen skall inte visa när projektet senast exporterades i MVP. Export och import placeras i projektsektionen i konfigurationspanelen.

MVP kräver persistent lagring. Appen skall som målprincip kunna fungera utan internet.

Dataägarskap är ett uttalat mål: användaren skall kunna exportera hela projektet inklusive bilder.

## 27. Intern datamodell på målnivå

Projekt, objekt, fält, listvärden och vyer skall ha stabila interna id:n. Objektets interna id är diskret synligt i detaljpanelen; övriga id:n visas normalt inte.

Objektvärden lagras som koppling mellan objekt och fält. Ett objekt består alltså av internt objekt-id plus en samling fältvärden kopplade till fält-id.

Flervärden lagras som ordnade listor. Även enkelvärdesfält kan i modellen betraktas som en lista med högst ett värde.

Vyer, filter, gruppering, datumfältsval och global etikett refererar till fält-id, inte fältnamn. Därför kan fält byta namn utan att vyer går sönder.

Datamodellen skall förberedas för flera projekt via `projectId`, även om MVP bara hanterar ett aktivt projekt.

## 28. Arkitekturkrav utan teknikval

Blueprinten beskriver krav på arkitekturen utan att välja teknikstack.

Arkitekturen måste stödja användardefinierade fält, stabila interna id:n, persistent lagring, offline-princip, komplett export/import av projekt, mobil och desktop, diagram som central arbetsyta, framtida möjlighet till flera projekt, framtida möjlighet till synk utan att synk byggs i MVP, samt framtida möjlighet till flera diagramtyper utan att fler diagramtyper byggs i MVP.

Teknikvalet skall göras efter att MVP-målet är tillräckligt stabilt. Teknikvalet skall endast jämföra öppna och fria tekniker.

Eftersom MVP saknar inloggning, synk och flera samtidiga användare skall server inte införas utan tydlig nytta.

MVP har en huvuddiagramtyp: tidslinjediagram med objekt på y-led och datum på x-led.

## 29. MVP-gräns

Följande skall inte införas i MVP utan nytt beslut:

- inloggning,
- flera källare i faktisk första version,
- flera aktiva projekt i UI,
- synk mellan flera enheter,
- flera samtidiga användare,
- CSV/Excel-import och CSV/Excel-export,
- separat tabell- eller listvy,
- streckkodsskanning,
- extern databas över öl/vin,
- rekommendationslogik för bästa öppningsår,
- särskild öppna/förbruka-funktion,
- hårdkodade fält för antal, öppnade, förbrukade eller status,
- hårdkodade fält för huvudbild eller extra bilder,
- användarsynliga fältnycklar eller texttaggar,
- Markdown-baserad presentation,
- textbaserade etikettmallar,
- diagrametikett per vy,
- särskild platsmodell,
- expanderbara eller kollapsbara diagramgrupper,
- färgkodning efter listfält,
- startmallar,
- fältgrupper i detaljvyn,
- papperskorg eller ångra-modell,
- minikarta eller översiktskarta i diagrammet,
- fri zoomgest i tidsled,
- inaktivering av listvärden,
- direkt skapande av listvärden från objektredigering,
- bildbeskärning eller användarstyrda komprimeringsinställningar,
- detaljerade designbeslut om färger, typografi och visuell stil.

Flera av dessa är önskvärda framtida möjligheter, men de skall inte styra första målbilden innan arkitektur och implementation beslutas.

## 30. Acceptanskriterier för MVP

Objektmodellen är godkänd när användaren kan skapa, redigera, duplicera och ta bort objekt som innehåller fälttyperna text, tal, datum, lista och bild.

Fältmodellen är godkänd när användaren kan skapa, ändra, sortera, göra obligatoriskt och ta bort fält enligt spärrreglerna. Fält skall kunna vara enkelvärde eller flervärde, utom bild som är enkelbild.

Huvuddiagrammet är godkänt när objekt visas som rader med sticky namnkolumn, sticky x-axel längst ned, datumlinjer med punkter, röd radton för objekt utan visade datum och detaljpanel via radnamn.

Vyfunktionen är godkänd när användaren kan skapa, byta, spara, byta namn, duplicera och ta bort vyer. En vy skall kunna lagra filter, 0–3 grupperingsfält, sorteringsriktningar och inkluderade datumfält.

Datahantering är godkänd när data finns kvar mellan sessioner och hela projektet kan exporteras/importeras som komplett projektfil inklusive bilder.

Prestandariktmärke: MVP skall fungera rimligt med minst 500 objekt, 50 fält och 10 sparade vyer. Detta är ett riktmärke, inte ett optimeringskrav.

## 31. Testdata och verifiering

Intern testdata skall finnas för utveckling och test, men inte visas som användarens startläge. Testdata får vara öl/vin-relaterad för att testa verkligt användningsfall, men datamodellen skall fortfarande vara generisk.

Testdata bör innehålla cirka 10–20 objekt och täcka text, tal, datum, listor, bild, flervärden, tomma värden, objekt utan datum, flera datum på samma dag och andra avsiktliga normalfallsbrott.

Manuella acceptanstest skall tas fram för fält, objekt, vyer, diagram, filter och import/export innan implementation betraktas som färdig.
