# cevi-versand
A better tool for generating envelopes from our database

![icon](./ui_/icon.png)

## Setup

Run the program once and it will generate you a `config.yaml` file that you should fill in.

- [ ] TODO: Guide the user through the process of getting a token and a filter link, then automatically re-run

```yaml
#config.yaml
db_conf:
    # paste your api_token here
    api_token: "th1s1sY0ur70k3n"
    # der Ceviname zum einloggen in der db.cevi.ch
    login_name: "GenerischerCeviname"
    # die e-mail adresse zum einloggen in der db.cevi.ch
    login_email: "irgendwer@irgendwas.ch"
    # Der link zu den Leuten in der datenbank. Relevant für dich als user sind nur die Zahlen.
    # Ersetze sie durch die gruppen-id und filter-id, die du verwenden möchtest.
    # Du kannst beliebig viele links verwenden.
    versand_endpoint_fmtstrs:
    	- "https://db.cevi.ch/groups/116/people.json?filter_id=319&user_email={login_email}&user_token={api_token}"
    	- "https://db.cevi.ch/groups/2423/people.json?user_email={login_email}&user_token={api_token}"
```

When you have set this up, run the program again.

It will tell you that `mapping.yaml` was missing and has been generated. From now on, running the program will look in `mapping.yaml` to replace any `original_name` with the corresponding `display_name`. That is a way for you to customize the generated pdfs - please only touch the `display_name`.

The program might also inform you about any people that are broken. E.g. they lack an address. These are still included as envelopes in the PDF, but will likely cause problems when sending them per post.

The program will at this point also have generated an `inject_people.yaml` file which allows you to add more envelopes, for people that are not in the database.

## Targets

* [x] Extract data directly from database, or automatically fetch csv
* [ ] Allow user to intuitively filter who should receive the consignment
* [x] Generate PDF with relevant information for manual sending, in correct order
* [x] No dependency on office if possible
* [x] Acceptable couverts by post standards

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
