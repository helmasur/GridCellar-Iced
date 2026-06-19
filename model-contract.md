# Modellkontrakt

## Syfte och gräns

Detta dokument definierar den minsta teknikneutrala datamodell som krävs för att uppfylla `blueprint.md`.

Kontraktet fastställer begrepp, relationer och regler. Det fastställer inte programmeringsspråkstyper, filformat, databas, lagringsmotor eller serialiseringsmetod.

## Gemensamma identitetsregler

Projekt, fält, listvärden, objekt och vyer har stabila interna id:n.

Interna id:n:

- är unika inom den identitetsrymd där de används,
- ändras inte när ett visningsnamn ändras,
- återanvänds inte för en ny entitet efter radering,
- används i relationer i stället för visningsnamn.

Objektets id visas diskret i detaljpanelen. Övriga interna id:n behöver normalt inte visas i UI.

## Projekt

Ett projekt är den överordnade behållaren för all projektdata.

Ett projekt innehåller minst:

- internt projekt-id,
- visningsnamn,
- fältdefinitioner,
- listvärden som hör till listfält,
- objekt,
- sparade vyer,
- global fältordning,
- global diagrametikett,
- globala diagraminställningar,
- referens till senast använda vy.

MVP hanterar ett aktivt projekt. Alla projektbundna entiteter skall ändå kunna knytas till ett stabilt projekt-id.

Globala diagraminställningar omfattar minst:

- radhöjd,
- gruppetiketthöjd,
- namnkolumnens bredd,
- tidsintervallets läge,
- start- och slutdatum när läget är egen period.

## Fält

En fältdefinition innehåller minst:

- internt fält-id,
- projekt-id,
- visningsnamn,
- fälttyp,
- värdeläge,
- obligatorisk status,
- position i global fältordning,
- detaljformat,
- typberoende inställningar.

Tillåtna fälttyper i MVP är:

- text,
- tal,
- datum,
- lista,
- bild.

Värdeläget är ett värde eller flera värden. Bildfält tillåter endast ett värde.

Talfält anger undertypen heltal eller decimaltal.

Detaljformatet skall vara giltigt för fälttypen. Tillåtna format är, där de är relevanta:

- normal rad,
- rubrikrad,
- kompakt etikett eller chip,
- längre textblock,
- bild,
- datum,
- tal.

Fältnamn används för visning och får ändras utan att relationer påverkas.

Ett nytt fält placeras sist i global fältordning. Befintliga objekt får tomt värde för fältet.

Ett fält får endast tas bort eller byta typ när:

- fältet saknar värden på alla objekt,
- fältet inte används av global diagrametikett,
- fältet inte används av någon vy, gruppering, filterkonfiguration eller datumfältskonfiguration.

Värdeläget får endast ändras när fältet saknar värden på alla objekt.

Obligatorisk status får endast aktiveras när samtliga befintliga objekt har giltigt värde.

## Listvärden

Ett listvärde tillhör exakt ett listfält och innehåller minst:

- internt listvärdes-id,
- fält-id,
- visningsnamn,
- position i listans manuella ordning.

Listvärdesnamn är unika inom samma listfält. Samma namn får förekomma i olika listfält.

Listvärdets namn och ordning får ändras utan att objektreferenser påverkas.

Ett listvärde får endast tas bort när det inte används av något objekt och inte refereras av något sparat filter.

## Objekt

Ett objekt innehåller minst:

- internt objekt-id,
- projekt-id,
- en samling fältvärden kopplade till fält-id.

Objektet har inga andra hårdkodade domänfält.

Ett objekt får skapas, redigeras, dupliceras och tas bort. Duplicering kopierar fältvärdena men skapar ett nytt objekt-id.

Ett objekt får endast sparas när samtliga obligatoriska fält har giltiga värden.

## Fältvärden

Ett objektvärde identifieras av kombinationen objekt-id och fält-id.

Varje objektvärde representeras begreppsligt som en ordnad värdelista:

- enkelvärdesfält innehåller noll eller ett värde,
- flervärdesfält innehåller noll eller flera värden,
- bildfält innehåller noll eller en bild.

Ett tomt fält representeras av att värdelistan saknar värden. Tom text är inte ett separat värde.

Textvärden trimmas i början och slut. Interna mellanslag bevaras.

Datumvärden är fullständiga kalenderdatum i formen `YYYY-MM-DD` utan tid eller tidszon.

Decimaltal får matas in med komma eller punkt men representerar efter normalisering samma numeriska värde.

Listvärden lagras som referenser till listvärdes-id, inte som listvärdets namn.

Bildvärden skall innehålla eller referera till den bilddata som krävs för persistent lagring och fullständig export. Den tekniska representationen beslutas senare.

En flervärdeslista får inte innehålla tomma enskilda värden. Värdenas ordning är en del av projektets information.

## Vyer

En vy innehåller minst:

- internt vy-id,
- projekt-id,
- visningsnamn,
- noll till tre ordnade grupperingsnivåer,
- sorteringsriktning per grupperingsnivå,
- noll eller flera filtervillkor,
- konfiguration för inkluderade och exkluderade datumfält.

Det skall alltid finnas minst en vy i ett projekt.

Vyer lagrar inte globala layoutinställningar, global diagrametikett eller globalt tidsintervall.

Nya datumfält ingår automatiskt i befintliga vyer. Därför skall datumfältskonfigurationen kunna skilja på uttryckligen exkluderade fält och fält som skapats efter vyn.

Varje grupperingsnivå refererar till fält-id och innehåller stigande eller fallande sorteringsriktning.

Bildfält får inte användas för gruppering.

När ett flervärdesfält används för gruppering används dess första värde. Tomma värden placeras i gruppen `Saknar värde`. Datumfält grupperas efter år och talfält efter exakt värde.

## Filter

Ett filtervillkor innehåller minst:

- fält-id,
- operator,
- operand eller operander när operatorn kräver det,
- inställning för att inkludera tomma värden.

Alla filtervillkor i en vy kombineras med AND.

Tillåtna operatorer beror på fälttyp:

- text: innehåller, är exakt,
- tal: lika med, större än, mindre än, intervall,
- datum: före, efter, mellan,
- lista: är någon av valda,
- alla filterbara typer: är tomt.

Textjämförelser är inte skiftlägeskänsliga.

För flervärdesfält matchar villkoret när minst ett värde matchar.

Listfilter refererar till listvärdes-id. Bildfält får inte användas i filter.

## Global diagrametikett

Den globala diagrametiketten är en ordnad lista med ett till fem fält-id:n.

När projektet har minst ett fält skall listan innehålla minst ett giltigt fält-id. När projektet saknar fält får listan vara tom.

Vid presentation:

- används fälten i angiven ordning,
- hoppas tomma värden över,
- används en fast separator mellan återstående delar,
- används objektets interna id om alla valda fält är tomma.

## Sortering

Internt objekt-id är alltid sista sorteringsnyckel.

När ingen gruppering används sorteras objekten efter internt objekt-id.

Text sorteras naturligt enligt implementationens fastställda jämförelseregel.

Tal sorteras numeriskt och datum kronologiskt.

Listvärden sorteras enligt listfältets manuella ordning.

Flervärden jämförs lexikografiskt i användarens värdeordning: första värdet jämförs först, därefter nästa värde tills en skillnad hittas eller en lista tar slut.

Regler för tomma värdens placering skall vara konsekventa inom implementationen och verifieras i acceptanstest.

## Diagramunderlag

Diagramrader är härledd data och behöver inte lagras som självständiga projektentiteter.

Diagramunderlaget härleds från:

- aktiv vy,
- projektets objekt och fältvärden,
- global diagrametikett,
- globalt tidsintervall,
- global fältordning,
- globala layoutinställningar.

Varje synligt objekt förekommer exakt en gång i grupperingen.

Ett objekt utan synliga datum inom det aktiva datumfältsurvalet och tidsintervallet ligger kvar som diagramrad och markeras som utan visade datum.

## Persistens och export

Persistent lagring skall kunna återskapa hela projektets informationsinnehåll mellan sessioner.

En komplett export skall kunna återskapa:

- projektet och dess inställningar,
- samtliga interna id:n,
- fält och listvärden med ordning,
- objekt och ordnade fältvärden,
- bilder,
- vyer, grupperingar, filter och datumfältskonfiguration,
- global diagrametikett,
- senast använda vy.

Import ersätter det aktiva projektet efter bekräftelse. Sammanfogning ingår inte i MVP.

Filformat, versionsmärkning, valideringsformat och migreringsstrategi fastställs i samband med export- och lagringsarkitekturen.
