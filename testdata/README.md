# Testdata

`gridcellar-sample.json` är teknikneutral testdata för modellkontrakt och acceptanstest. JSON-strukturen är inte ett beslutat export- eller lagringsformat.

Projektet använder öl och vin som realistiskt exempel men innehåller inga domänfält i modellen.

## Avsiktliga testfall

- `object-01`: komplett normalfall med bild.
- `object-02`: decimaltal och flera listvärden i användarvald ordning.
- `object-03`: två datumvärden på samma dag i olika datumfält.
- `object-04`: saknar alla datumvärden.
- `object-05`: tomma frivilliga fält.
- `object-06`: flera datum i samma flervärdesfält.
- `object-07`: text med interna mellanslag och längre text.
- `object-08`: listvärden i en ordning som skiljer sig från alfabetisk ordning.
- `object-09`: datum utanför ett typiskt femårsintervall.
- `object-10`: alla fält i diagrametiketten är tomma och objekt-id måste användas som fallback.
- `object-11`: heltalsvärdet noll.
- `object-12`: flera objekt delar grupp- och sorteringsvärden, så objekt-id blir sista sorteringsnyckel.

`images/generic-bottle.png` är en lokal testbild utan varumärke eller text. Den genererades för projektets testdata och skall användas för att verifiera bildfält samt komplett export och import.
