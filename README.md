# cevi-versand
A better tool for generating envelopes from our database

## Targets

* Extract data directly from database, or automatically fetch csv
* Allow user to intuitively filter who should receive the consignment
* Generate PDF with relevant information for manual sending, in correct order
* No dependency on office if possible
* Acceptable couverts by post standards

### Extract Data

Siehe [API docs](wiki.cevi.ch/index.php/CeviDB_API):

> ### Erstes Login
>
> Um sich anzumelden, muss ein POST-Request an https://db.cevi.ch/users/sign_in.json gesendet werden. Als Parameter müssen `person[email]` und `person[password]` übergeben werden.
>
> ```
> 1 import requests
> 2 email = "ceviname@cevi.ch"
> 3 passwort = "123456"
> 4 res = requests.post('https://db.cevi.ch/users/sign_in.json?person[email]='+email+'&person[password]='+passwort)
> 5 token = res.json()["people"][0]["authentication_token"]
> ```

endpoint Versand: https://db.cevi.ch/groups/116/people.json?filter_id=319

sortiert Versand: https://db.cevi.ch/groups/116/people.json?filter_id=319&user_email=secret@mail.ch&user_token=placeholder&sort=address

mitgliederbeitragfilter: filter_id=591

### Filter

* Leiter 
* Teilnehmer
* Ehemalige Leiter
* Ehemalige Alle
* Trägerkreis

Wer Leiter ist, ist nicht gleichzeitig Teilnehmer. Wer Ehemalig ist, ist nicht gleichzeitig Leiter oder Teilnehmer. Wer Trägerkreis ist, zählt nicht als Ehemalig.

### PDF

* Cevi Logo oben links
* Namen/Cevinamen und Stufe aller Empfänger oben links, nach alter absteigend sortiert
* Adresse in Adressfeld
* Unten links Balken "# Leiter", "# Teilnehmer", "# Ehemalige" etc. Wenn Anzahl 0 dann nicht anzeigen
* Ausgabe sortiert nach Stufen des ältesten Empfängers
* Linker Rand: 12mm Abstand bis Werbebereich
  Unten Rechts: Codierzone 140 x 15 mm
  Rechter Rand: 12mm Abstand bis Adressfeld
  Adressfeld: nicht mit Werbung überlappen, nicht mit codierzone überlappen
  Adressfeld: min (10+38)mm abstand nach oben, min (12+22)mm abstand nach links
  ![layout couvert post](.\post\screenshot1.png)
