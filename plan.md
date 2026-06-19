# Implementationsplan

## Inledning

- Rust används som huvudspråk.
- Iced används som GUI-ramverk.

Statusvärden:

- Ej påbörjad: arbetet har inte startat.
- Pågår: arbetet är aktivt.
- Blockerad: arbetet kan inte fortsätta utan beslut, information eller extern åtgärd.
- Klar: arbetet är genomfört i relevant scope.
- Verifierad: arbetet är genomfört och verifierat enligt angiven verifiering.

## Styrande principer

1. `blueprint.md` är styrande produktmål.
2. Planen beskriver MVP och får inte utökas med funktioner utanför MVP utan nytt beslut.
3. Implementationens datamodell skall följa begreppen projekt, fält, listvärden, objekt, objektvärden, vyer och diagramrader.
4. Implementationens funktioner skall kunna verifieras mot acceptanstest enligt denna plan.
5. Tekniska avvikelser är tillåtna om de inte ändrar produktmålet.
6. Funktionella avvikelser skall dokumenteras som avvikelser från MVP.
7. Planen skall hålla sig på lägsta gemensamma nivå och undvika ramverksspecifika lösningar.
8. Endast öppna och fria tekniker är aktuella.

## Föreslagen projektstruktur

Strukturen nedan beskriver ett enskilt implementationsträd. Namnet på implementationens katalog sätts utanför denna plan.

```text
GridCellar/
  AGENTS.md
  blueprint.md
  blueprint.work.md
  plan.md
  user-decisions.md

  model-contract.md
  acceptance-tests.md
  testdata/
    gridcellar-sample.json

  implementation/
    ...
```

`model-contract.md`, `acceptance-tests.md` och `testdata/` är styrande stödmaterial. De skall inte bli ett runtime-bibliotek utan särskilt beslut.

## Fas 0: Planlåsning och arbetsregler

Status: Klar

Syfte:

- Säkerställa att implementationen byggs mot `blueprint.md`.
- Säkerställa att planen inte inför teknik- eller ramverksspecifik komplexitet.

Bygger på:

- `blueprint.md`
- `user-decisions.md`
- detta dokument

### Arbetspaket 0.1: Bekräfta MVP

Status: Klar

Scope:

- Ingår: kontroll att `blueprint.md` är giltigt mål för implementationen.
- Ingår inte: paketval, komponentbibliotek, filformat eller kod.

Verifiering:

- Det finns inget förenklat mål vid sidan av `blueprint.md`.
- Avvikelser från MVP dokumenteras uttryckligen.

Steg:

- [x] Bekräfta att `blueprint.md` är styrande.
- [x] Bekräfta att MVP-gränsen i `blueprint.md` gäller.
- [x] Bekräfta att acceptanstest skall användas som funktionell verifiering.

### Arbetspaket 0.2: Status och avvikelser

Status: Klar

Scope:

- Ingår: enkel statusmodell och plats för avvikelser.
- Ingår inte: poängsättning eller projektstyrningssystem.

Verifiering:

- Det finns en plats där blockerade punkter och funktionella avvikelser kan dokumenteras.

Steg:

- [x] Skapa statussektion i implementationens README eller motsvarande.
- [x] Skapa enkel avvikelselogg.
- [x] Ange att funktionella avvikelser skall lösas eller beslutas innan MVP räknas som klar.

## Fas 1: Kontrakt före implementation

Status: Klar

Syfte:

- Definiera den minsta produkt- och datakontraktnivå som implementationen skall följa innan produktfunktioner byggs.

Bygger på:

- `blueprint.md`

### Arbetspaket 1.1: Modellkontrakt

Status: Klar

Scope:

- Ingår: teknikneutral beskrivning av datamodellen på begreppsnivå.
- Ingår inte: språktyper, databasval, filformat och lagringsmotor.

Modellkontraktet skall beskriva:

- Projekt med internt id och namn.
- Globala projektinställningar.
- Fält med internt id, namn, typ, ordning, obligatorisk status, enkelvärde/flervärde och detaljformat.
- Fälttyperna text, tal, datum, lista och bild.
- Listvärden med internt id, namn och ordning.
- Objekt med internt id och fältvärden.
- Fältvärden som ordnade värdelistor.
- Tomvärdesregler.
- Vyer med filter, gruppering, sorteringsriktningar och datumfältsurval.
- Global diagrametikett.

Verifiering:

- Ett komplett projekt kan beskrivas utan informationsförlust.
- Kontraktet använder interna id:n där blueprinten kräver stabil referens.

Steg:

- [x] Skapa `model-contract.md`.
- [x] Beskriv varje entitet och dess minsta nödvändiga egenskaper.
- [x] Beskriv vilka relationer som använder interna id:n.
- [x] Beskriv regler för tomma värden, flervärden och sortering.

### Arbetspaket 1.2: Testdata

Status: Klar

Scope:

- Ingår: ett testprojekt med 10–20 objekt.
- Ingår inte: användarvisad exempeldata vid första start.

Testdata skall täcka:

- Textfält.
- Talfält med heltal och decimaltal.
- Datumfält.
- Listfält.
- Bildfält eller bildplatshållare enligt beslutad testnivå.
- Enkelvärden och flervärden.
- Tomma värden.
- Objekt utan visade datum.
- Flera datum på samma dag.
- Minst en vy med filter.
- Minst en vy med gruppering.

Verifiering:

- Testdata följer modellkontraktet.
- Varje kantfall har dokumenterat syfte.

Steg:

- [x] Skapa `testdata/gridcellar-sample.json` eller motsvarande projektfil.
- [x] Dokumentera syfte med varje särskilt testobjekt.
- [x] Kontrollera att testdata följer modellkontraktet.

### Arbetspaket 1.3: Acceptanstest

Status: Klar

Scope:

- Ingår: manuella acceptanstest för MVP.
- Ingår inte: automatiserad testinfrastruktur om sådan inte beslutas separat.

Acceptanstest skall täcka:

- Första start.
- Projektinställningar.
- Fältadministration.
- Listvärdesadministration.
- Objektflöden.
- Vyhantering.
- Filter och sökning.
- Diagram.
- Lokal persistens.
- Export/import.
- Smal och bred layout.

Verifiering:

- Varje test har förutsättning, steg och förväntat resultat.
- Testen kan köras utan kännedom om implementationens interna teknik.

Steg:

- [x] Skapa `acceptance-tests.md`.
- [x] Skriv testfall per huvudområde.
- [x] Lägg till plats för testresultat.

## Fas 2: Körbar grund

Status: Verifierad

Syfte:

- Skapa ett körbart minimiprojekt utan produktfunktioner.

Bygger på:

- Fas 0
- Fas 1

### Arbetspaket 2.1: Projektgrund

Status: Verifierad

Scope:

- Ingår: tom körbar app, dokumenterade start-/kontrollkommandon och grundläggande mappstruktur.
- Ingår inte: produktfunktioner, datamodell och UI-flöden.

Verifiering:

- Implementationens minimiprojekt kan startas lokalt.
- Start, kontroll och byggning är dokumenterade.
- Kända begränsningar är dokumenterade utan att ändra produktmålet.

Steg:

- [x] Skapa implementationens projektkatalog.
- [x] Skapa minimal körbar app.
- [x] Dokumentera startkommando.
- [x] Dokumentera build/check-kommando.
- [x] Lägg in README eller motsvarande statusfil.

### Arbetspaket 2.2: Arbetsdisciplin

Status: Klar

Scope:

- Ingår: statusmodell, referens till blueprint och avvikelselogg.
- Ingår inte: gemensam koddelning eller teknikval.

Verifiering:

- Det går att se vilka arbetspaket som är klara, blockerade eller avvikande.

Steg:

- [x] Skapa statussektion.
- [x] Skapa avvikelselogg.
- [x] Ange hur funktionella avvikelser dokumenteras.

## Fas 3: Kärnmodell, regler och lokal data

Status: Verifierad

Syfte:

- Implementera den logiska kärnan innan större UI-arbete.

Bygger på:

- `model-contract.md`
- `blueprint.md`

### Arbetspaket 3.1: Datamodell

Status: Verifierad

Scope:

- Ingår: interna strukturer för projekt, fält, listvärden, objekt, objektvärden och vyer.
- Ingår inte: fullständig UI, export/import och diagramrendering.

Verifiering:

- Implementationen kan skapa ett tomt standardprojekt.
- Implementationen kan representera testdata enligt modellkontraktet.

Steg:

- [x] Implementera projektenhet.
- [x] Implementera fältenhet.
- [x] Implementera listvärden.
- [x] Implementera objektenhet och fältvärden.
- [x] Implementera vyenhet.
- [x] Kontrollera mot modellkontraktet.

### Arbetspaket 3.2: Validering och spärrregler

Status: Verifierad

Scope:

- Ingår: regler som avgör om data eller konfiguration är giltig.
- Ingår inte: slutlig användarpresentation av fel.

Regler som minst skall ingå:

- Fältnamn är unika inom projektet.
- Obligatoriska fält måste ha giltigt värde för att objekt skall kunna sparas.
- Tom text normaliseras till tomt värde.
- Fälttyp får bara ändras när fältet saknar värden och inte används i konfigurationer.
- Fält får bara tas bort när det saknar objektvärden och inte används i konfigurationer.
- Listvärden får bara tas bort när de inte används av objekt eller sparade filter.
- Global diagrametikett får vara tom endast när projektet saknar fält.
- Ogiltiga filter skall kunna identifieras.

Verifiering:

- Testfall ger godkänd eller spärrad bedömning enligt blueprintens regler.

Steg:

- [x] Implementera fältregler.
- [x] Implementera objektregler.
- [x] Implementera listvärdesregler.
- [x] Implementera vy- och filterregler.
- [x] Implementera beroendekontroller.

### Arbetspaket 3.3: Vyberäkning

Status: Verifierad

Scope:

- Ingår: beräkning av vilka objekt som syns, deras ordning och deras diagramunderlag.
- Ingår inte: grafisk rendering.

Beräkningen skall hantera:

- Sparade filter med AND-logik.
- Filter på tomma värden.
- `inkludera tomma värden`.
- Fritextsökning i text- och listfält.
- Gruppering 0–3 nivåer.
- Sorteringsriktning per grupperingsnivå.
- Internt id som sista sorteringsnyckel.
- Flervärdesfält där första värdet styr gruppering i MVP.
- Datumfält som kan inkluderas/exkluderas i vy.
- Objekt utan visade datum.

Verifiering:

- Testdata ger förväntade synliga objekt, grupper och sorteringsordning.

Steg:

- [x] Implementera filterutvärdering.
- [x] Implementera fritextsökning.
- [x] Implementera gruppering.
- [x] Implementera sortering.
- [x] Implementera datumurval.
- [x] Implementera radunderlag för diagram.

### Arbetspaket 3.4: Lokal persistens

Status: Verifierad

Scope:

- Ingår: projektdata finns kvar mellan sessioner.
- Ingår inte: synk, flera användare och flera aktiva projekt i UI.

Verifiering:

- Skapade fält, objekt, vyer och inställningar finns kvar efter omstart/omladdning enligt implementationens körmiljö.

Steg:

- [x] Implementera lokal sparning.
- [x] Implementera lokal laddning.
- [x] Testa tomt standardprojekt.
- [x] Testa ändringar i fält, objekt och vyer.

## Fas 4: Huvudfönster och panelmodell

Status: Verifierad

Syfte:

- Bygga appens grundstruktur utan att först lösa alla detaljer.

Bygger på:

- `blueprint.md`

### Arbetspaket 4.1: Huvudlayout

Status: Verifierad

Scope:

- Ingår: projektnamn, vyval, tidsintervallkontroll, sökning, lägg till objekt, konfiguration och diagramyta.
- Ingår inte: full funktion bakom alla kontroller.

Verifiering:

- Huvudytan har blueprintens begrepp och primära flöden.
- Diagramytan är appens huvudområde.
- Layouten kan visas i smal och bred yta enligt implementationens förutsättningar.

Steg:

- [x] Skapa huvudfönster.
- [x] Lägg in projektnamn.
- [x] Lägg in vyval.
- [x] Lägg in tidsintervallkontroll.
- [x] Lägg in sökning.
- [x] Lägg in `Lägg till objekt`.
- [x] Lägg in konfigurationsåtkomst.
- [x] Lägg in diagramyta.

### Arbetspaket 4.2: Paneler och dialogflöden

Status: Verifierad

Scope:

- Ingår: detaljpanel, konfigurationspanel, filterpanel och datumfältspanel.
- Ingår inte: slutlig layoutpolering.

Verifiering:

- Varje panel kan öppnas och stängas.
- Panelerna stödjer blueprintens användarflöden.

Steg:

- [x] Skapa detaljpanel.
- [x] Skapa konfigurationspanel.
- [x] Skapa filterpanel.
- [x] Skapa datumfältspanel.
- [x] Kontrollera grundläggande stängning och fokusflöde där relevant.

## Fas 5: Projekt- och fältkonfiguration

Status: Verifierad

Syfte:

- Göra projektets globala konfiguration användbar.

Bygger på:

- Datamodell och validering.
- Blueprintens regler för konfiguration.

### Arbetspaket 5.1: Projektinställningar

Status: Verifierad

Scope:

- Ingår: projektnamn, global radhöjd, namnkolumnbredd och globalt tidsintervall.
- Ingår inte: flera aktiva projekt i UI.

Verifiering:

- Projektinställningar kan ändras och sparas.
- Inställningarna påverkar relevanta delar av huvudfönster och diagram.

Steg:

- [x] Implementera projektnamn.
- [x] Implementera radhöjd.
- [x] Implementera namnkolumnbredd.
- [x] Implementera globalt tidsintervall.

### Arbetspaket 5.2: Fältadministration

Status: Verifierad

Scope:

- Ingår: skapa, byta namn, ändra tillåtna egenskaper, sortera, göra obligatoriskt och ta bort fält enligt spärrregler.
- Ingår inte: fältgrupper, användarsynliga fältnycklar, Markdown och fälttyper utanför MVP.

Verifiering:

- Fält kan hanteras enligt blueprintens regler.
- Spärrade åtgärder visar orsak.

Steg:

- [x] Visa fältlista.
- [x] Skapa nytt fält.
- [x] Ändra fältnamn.
- [x] Ändra fälttyp endast när regler tillåter det.
- [x] Ändra enkelvärde/flervärde endast när regler tillåter det.
- [x] Ändra obligatorisk status enligt regler.
- [x] Ändra detaljformat när formatval finns.
- [x] Sortera fält med upp-/nedkontroller.
- [x] Ta bort fält enligt spärrregler.
- [x] Visa användningssammanfattning.

### Arbetspaket 5.3: Listvärdesadministration

Status: Verifierad

Scope:

- Ingår: skapa, byta namn, sortera och ta bort listvärden enligt spärrregler.
- Ingår inte: direkt skapande av listvärden från objektredigering.

Verifiering:

- Listvärden visas i manuell ordning.
- Listvärden som används av objekt eller filter kan inte tas bort.

Steg:

- [x] Visa listvärden per listfält.
- [x] Skapa listvärde.
- [x] Byt namn på listvärde.
- [x] Sortera listvärden med upp-/nedkontroller.
- [x] Visa användningsantal.
- [x] Ta bort listvärde enligt spärrregler.

### Arbetspaket 5.4: Global diagrametikett

Status: Verifierad

Scope:

- Ingår: etikettfältlista, fast separator, tomvärdeshantering och fallback till internt id.
- Ingår inte: etikett per vy och textmallar.

Verifiering:

- Etikett får vara tom endast när projektet saknar fält.
- Tomma etikettfält hoppas över.
- Internt id används som fallback när etiketten blir tom för ett objekt.

Steg:

- [x] Implementera val av 1–5 etikettfält.
- [x] Implementera fast separator.
- [x] Implementera tomvärdesrensning.
- [x] Implementera fallback till internt id.

## Fas 6: Objekt och detaljpanel

Status: Verifierad

Syfte:

- Göra det möjligt att skapa, visa, ändra, duplicera och ta bort objekt.

Bygger på:

- Fältadministration.
- Objektvalidering.

### Arbetspaket 6.1: Visningsläge

Status: Verifierad

Scope:

- Ingår: alla globala fält i global ordning, detaljformat, diskreta tomvärden och diskret internt id.
- Ingår inte: fältgrupper och Markdown.

Verifiering:

- Objekt visas med rätt fält, ordning och värden.

Steg:

- [x] Visa objektets interna id diskret.
- [x] Visa alla globala fält.
- [x] Visa tomma värden diskret.
- [x] Implementera format: rubrikrad, normal rad, chip, längre textblock, bild, datum och tal där relevant.

### Arbetspaket 6.2: Skapande och redigering

Status: Verifierad

Scope:

- Ingår: skapandeläge, redigeringsläge, `Spara`, `Återställ`, osparade ändringar, inmatning per fälttyp och validering.
- Ingår inte: standardvärden, bildbeskärning, bildkomprimeringsinställningar och skapande av listvärden från objektformulär.

Verifiering:

- Objekt kan skapas och redigeras enligt blueprintens regler.
- Obligatoriska fält spärrar sparande när de saknar giltigt värde.
- `Återställ` återgår till senast sparade värden.

Steg:

- [x] Öppna skapandeläge från `Lägg till objekt`.
- [x] Öppna redigeringsläge från detaljpanelen.
- [x] Implementera textinmatning.
- [x] Implementera talinmatning.
- [x] Implementera datuminmatning.
- [x] Implementera listval.
- [x] Implementera bildval.
- [x] Implementera flervärden med ordning.
- [x] Implementera `Spara`.
- [x] Implementera `Återställ`.
- [x] Implementera varning vid osparade ändringar.

### Arbetspaket 6.3: Duplicera och ta bort

Status: Verifierad

Scope:

- Ingår: duplicering med nytt internt id och radering med bekräftelse.
- Ingår inte: papperskorg och ångra.

Verifiering:

- Duplicerat objekt har nytt internt id och kopierade fältvärden.
- Radering kräver bekräftelse.

Steg:

- [x] Implementera duplicering.
- [x] Öppna duplicerat objekt i redigeringsläge innan sparande.
- [x] Implementera radering.
- [x] Implementera bekräftelse.

## Fas 7: Vyer, filter och sökning

Status: Pågår

Syfte:

- Göra det möjligt att styra vad huvuddiagrammet visar.

Bygger på:

- Vyberäkning.
- Huvudfönster.

### Arbetspaket 7.1: Vyhantering

Status: Verifierad

Scope:

- Ingår: skapa, byta, spara, byta namn, duplicera och ta bort vyer samt visa osparade vyändringar.
- Ingår inte: vybundet tidsintervall, vybunden layout och separat objektsortering.

Verifiering:

- Ny vy startar med alla objekt, inga filter, ingen gruppering och sortering efter internt id.
- Osparade vyändringar markeras.
- Direkt vybyte kastar osparade vyändringar.

Steg:

- [x] Implementera vyval.
- [x] Implementera ny vy.
- [x] Implementera spara vy.
- [x] Implementera byta namn.
- [x] Implementera duplicera vy.
- [x] Implementera ta bort vy.
- [x] Implementera status för osparade vyändringar.

### Arbetspaket 7.2: Gruppering och datumfältsval

Status: Verifierad

Scope:

- Ingår: 0–3 grupperingsfält, sorteringsriktning per nivå och checklista för datumfält.
- Ingår inte: kollapsbara grupper och separat objektsortering.

Verifiering:

- Vyer ger korrekt gruppering, ordning och datumfältsurval.

Steg:

- [x] Implementera 0–3 grupperingsnivåer.
- [x] Implementera sorteringsriktning per nivå.
- [x] Implementera datumfältschecklista.
- [x] Implementera regler för tomma värden.
- [x] Implementera regler för datumgruppering efter år.
- [x] Implementera regler för talgruppering efter exakt värde.

### Arbetspaket 7.3: Filter

Status: Verifierad

Scope:

- Ingår: textfilter, talfilter, datumfilter, listfilter, `är tomt`, `inkludera tomma värden`, flervärden och ogiltiga filter.
- Ingår inte: OR-logik, filtergrupper och avancerade uttryck.

Verifiering:

- Filter ger förväntade synliga objekt.

Steg:

- [x] Implementera filterlista.
- [x] Implementera textoperatorerna `innehåller` och `är exakt`.
- [x] Implementera taloperatorer enligt blueprinten.
- [x] Implementera datumoperatorer enligt blueprinten.
- [x] Implementera listfilter med flera val.
- [x] Implementera `är tomt`.
- [x] Implementera `inkludera tomma värden`.
- [x] Implementera ogiltighetsmarkering.

### Arbetspaket 7.4: Fritextsökning

Status: Ej påbörjad

Scope:

- Ingår: tillfällig sökning i text- och listfält, aktiv status och rensa.
- Ingår inte: sökning i datum och tal.

Verifiering:

- Sökterm ger förväntade synliga objekt.
- Sökning sparas inte i vyn.

Steg:

- [ ] Implementera sökkontroll.
- [ ] Implementera sökutvärdering.
- [ ] Implementera aktiv status.
- [ ] Implementera rensa sökning.

## Fas 8: Huvuddiagram

Status: Ej påbörjad

Syfte:

- Implementera appens centrala tidslinjediagram.

Bygger på:

- Vyberäkning.
- Huvudfönster.
- Diagramregler i blueprinten.

### Arbetspaket 8.1: Diagramradmodell

Status: Ej påbörjad

Scope:

- Ingår: beräkning av objektetiketter, grupprader, objektrader, datumlinjer, datumpunkter, synligt tidsintervall och status för objekt utan visade datum.
- Ingår inte: grafisk rendering.

Verifiering:

- Testdata ger förväntade diagramrader och datumunderlag.

Steg:

- [ ] Beräkna objektetiketter.
- [ ] Beräkna grupprader.
- [ ] Beräkna objektrader.
- [ ] Beräkna datumlinjer.
- [ ] Beräkna datumpunkter.
- [ ] Beräkna objekt utan visade datum.

### Arbetspaket 8.2: Diagramlayout

Status: Ej påbörjad

Scope:

- Ingår: fast radhöjd, grupprader, vänster namnkolumn, x-axel längst ned, vertikal scroll och horisontell panorering.
- Ingår inte: minikarta, fri zoomgest och kollapsbara grupper.

Verifiering:

- Diagrammet visar rader, grupper, namnkolumn och x-axel enligt blueprinten.
- Diagrammet går att navigera i både höjdled och tidsled.
- Eventuella begränsningar i exakt rendering dokumenteras om de påverkar upplevelsen.

Steg:

- [ ] Implementera diagramyta.
- [ ] Implementera radhöjd.
- [ ] Implementera grupprader.
- [ ] Implementera namnkolumn.
- [ ] Implementera x-axel längst ned.
- [ ] Implementera vertikal navigation.
- [ ] Implementera horisontell navigation.

### Arbetspaket 8.3: Datumlinjer, punkter och interaktion

Status: Ej påbörjad

Scope:

- Ingår: linje från tidigaste till senaste visade datum, punkter, flera datum samma dag, tooltip/tryckinformation, markerad rad/punkt och röd radton.
- Ingår inte: datumetiketter direkt på punkterna.

Verifiering:

- Datum placeras efter exakt kalenderdatum.
- Punktinformation visar objektetikett, fältnamn och datumvärde.
- Endast radnamn öppnar detaljpanel.
- Objekt utan visade datum markeras med diskret röd radton.

Steg:

- [ ] Rita datumlinjer.
- [ ] Rita datumpunkter.
- [ ] Hantera flera värden på samma datum.
- [ ] Visa punktinformation vid tryck/klick/hover där relevant.
- [ ] Markera rad/punkt.
- [ ] Öppna detaljpanel via radnamn.
- [ ] Markera rader utan visade datum.

### Arbetspaket 8.4: Tidskontroller

Status: Ej påbörjad

Scope:

- Ingår: `Visa allt`, `5 år`, `10 år`, `Egen period`, automatisk marginal, axelgranularitet, `Idag`-linje och `Passa in alla datum`.
- Ingår inte: vybundet tidsintervall och fri zoomgest.

Verifiering:

- Tidsval ger korrekt logiskt tidsintervall.
- `Idag` visas bara när dagens datum ligger inom synligt intervall.
- `Passa in alla datum` ändrar aktuell visning utan att ändra sparad vy.

Steg:

- [ ] Implementera tidsval.
- [ ] Implementera egen period.
- [ ] Implementera automatisk marginal.
- [ ] Implementera axelgranularitet.
- [ ] Implementera `Idag`-linje.
- [ ] Implementera `Passa in alla datum`.

## Fas 9: Export, import och dataägarskap

Status: Ej påbörjad

Syfte:

- Säkerställa att projektet kan säkerhetskopieras, flyttas och återskapas.

Bygger på:

- Datamodell.
- Lokal persistens.
- Blueprintens export/import-regler.

### Arbetspaket 9.1: Exportkontrakt

Status: Ej påbörjad

Scope:

- Ingår: logisk struktur för komplett projektfil inklusive version och bilder.
- Ingår inte: CSV/Excel och sammanfogning av projekt.

Verifiering:

- Exportkontraktet kan representera hela projektet utan informationsförlust.

Steg:

- [ ] Definiera versionsfält.
- [ ] Definiera projektdata.
- [ ] Definiera konfigurationsdata.
- [ ] Definiera objektdata.
- [ ] Definiera bildrepresentation.

### Arbetspaket 9.2: Export

Status: Ej påbörjad

Scope:

- Ingår: manuell export av komplett projektfil inklusive bilder.
- Ingår inte: delvis export, CSV/Excel och synk.

Verifiering:

- Exportfil kan användas för att återskapa projektet.

Steg:

- [ ] Implementera export.
- [ ] Exportera testprojekt.
- [ ] Kontrollera att exporten innehåller projekt, fält, listvärden, objekt, vyer, inställningar och bilder.

### Arbetspaket 9.3: Import

Status: Ej påbörjad

Scope:

- Ingår: import av komplett projektfil, validering och ersättning av nuvarande projekt efter bekräftelse.
- Ingår inte: sammanfogning av projekt.

Verifiering:

- Importerat projekt motsvarar exporterat projekt.
- Import kräver tydlig bekräftelse.

Steg:

- [ ] Implementera filval eller motsvarande importflöde.
- [ ] Validera importfil.
- [ ] Visa bekräftelse innan ersättning.
- [ ] Ersätt nuvarande projekt.
- [ ] Kontrollera att importerat projekt fungerar i huvuddiagrammet.

## Fas 10: Första start, tomlägen och vägledning

Status: Ej påbörjad

Syfte:

- Göra startläget begripligt utan mallar eller exempeldata.

Bygger på:

- Blueprintens regler för första användningsflöde.

### Arbetspaket 10.1: Första start

Status: Ej påbörjad

Scope:

- Ingår: tomt standardprojekt, första vy och startinstruktion.
- Ingår inte: mallar och användarvisad exempeldata.

Verifiering:

- Första start visar tom huvudvy.
- Primära val är att skapa första fältet eller importera projekt.
- Första vyn finns och är giltig.

Steg:

- [ ] Skapa tomt standardprojekt.
- [ ] Skapa första vy.
- [ ] Visa startinstruktion.
- [ ] Länka till fältadministration.
- [ ] Länka till import.

### Arbetspaket 10.2: Tomlägen och spärrorsaker

Status: Ej påbörjad

Scope:

- Ingår: tomt projekt, inga fält, inga objekt, spärrade åtgärder och ogiltiga konfigurationer.
- Ingår inte: avancerad onboarding-guide.

Verifiering:

- Användaren får kort förklaring när en åtgärd inte kan genomföras.

Steg:

- [ ] Visa tomläge när inga fält finns.
- [ ] Visa tomläge när inga objekt finns.
- [ ] Visa orsak för spärrad fältborttagning.
- [ ] Visa orsak för spärrad typändring.
- [ ] Visa orsak för spärrad listvärdesborttagning.
- [ ] Visa ogiltig vy eller filterkonfiguration.

## Fas 11: Smal/bred layout och grundläggande användbarhet

Status: Ej påbörjad

Syfte:

- Säkerställa att appen är praktiskt användbar i både smal och bred yta.

Bygger på:

- Blueprintens responsiva och användbarhetsmässiga regler.

### Arbetspaket 11.1: Smal och bred layout

Status: Ej påbörjad

Scope:

- Ingår: samma flöden i smal och bred yta, med skillnad i överblick och skalning.
- Ingår inte: två olika produktstrukturer.

Verifiering:

- Primära flöden kan genomföras i smal och bred yta.
- Diagrammet är fortfarande huvudytan.

Steg:

- [ ] Testa huvudfönster i smal yta.
- [ ] Testa huvudfönster i bred yta.
- [ ] Testa paneler i smal yta.
- [ ] Testa paneler i bred yta.
- [ ] Testa diagramnavigation i smal och bred yta.

### Arbetspaket 11.2: Grundläggande användbarhet och tillgänglighet

Status: Ej påbörjad

Scope:

- Ingår: användning utan hover, grundläggande tangentbordsflöde där relevant, fokusflöde för paneler och tydliga kontroller.
- Ingår inte: formell tillgänglighetscertifiering.

Verifiering:

- Viktig information kan nås utan hover.
- Primära flöden kan genomföras med klick/tryck.
- Där tangentbord är relevant skall grundläggande tangentbordsanvändning fungera.

Steg:

- [ ] Testa information från datumpunkter utan hover.
- [ ] Testa att radnamn öppnar detaljpanel.
- [ ] Testa panelstängning.
- [ ] Testa grundläggande tangentbordsflöde där relevant.
- [ ] Dokumentera eventuella begränsningar.

## Fas 12: Slutverifiering

Status: Ej påbörjad

Syfte:

- Avgöra om implementationen uppfyller MVP enligt blueprinten.

Bygger på:

- Acceptanstest.
- `blueprint.md`.

### Arbetspaket 12.1: Acceptanstest

Status: Ej påbörjad

Scope:

- Ingår: köra acceptanstest mot implementationen.
- Ingår inte: nya funktioner.

Verifiering:

- Varje test är markerat som godkänt, avviket eller blockerat.
- Avvikna och blockerade test har kort förklaring.

Steg:

- [ ] Kör acceptanstest.
- [ ] Dokumentera godkända test.
- [ ] Dokumentera avvikna test.
- [ ] Dokumentera blockerade test.
- [ ] Fatta beslut om kvarstående avvikelser skall lösas eller accepteras.

### Arbetspaket 12.2: MVP-beslut

Status: Ej påbörjad

Scope:

- Ingår: beslut om implementationen är färdig som MVP.
- Ingår inte: teknikutvärdering, nya funktioner och utökad roadmap.

Verifiering:

- Det finns ett tydligt beslut: MVP godkänd, MVP ej godkänd eller MVP godkänd med dokumenterade avvikelser.

Steg:

- [ ] Gå igenom acceptanstestresultat.
- [ ] Gå igenom avvikelselogg.
- [ ] Kontrollera att explicit icke-mål inte har införts.
- [ ] Dokumentera MVP-beslut.

## Explicit icke-mål i denna plan

- Planen inför inte server, inloggning, synk eller flera användare.
- Planen inför inte CSV/Excel, mallar, färgkodning, kollapsbara grupper eller fri zoomgest.
- Planen kräver inte gemensam koddelning.
- Planen kräver inte ett visst visuellt uttryck.
- Planen tillåter inte att tekniska skillnader används för att ändra produktmålet utan nytt beslut.
