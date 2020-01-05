# cevi-versand
A better tool for generating envelopes from our database

![image-20200105155146648](N:\Files\projects\cevi-versand\ui_\exampleEnvelope.png)

### Quick Start

```bash
cv.exe setup -t "MyS3rviceToken"
vim config.yaml # add endpoints, as outlined in the section "Setup"
cv.exe run
```
![icon](./ui_/icon_small.png)
### Help

Every subcommand features the `-h` flag for usage help. E.g. `cv.exe -h` or `cv.exe setup -h`.

## Setup

`cevi-versand` bietet zwei Authentifikationsmöglichkeiten:

* `cv.exe setup -e levanzo@cevi.ch -p meinP4sswort` holt ein user-token von der Datenbank und speichert dieses in `config.yaml`. So ist das Passwort nicht gespeichert. Wer möchte, kann `config.yaml` auch selber einrichten, braucht dafür aber natürlich das user-token von der Datenbank.
  Dieses user-token gibt dem Programm alle Rechte, die der Nutzer auch hat. Deshalb wird diese Authentifikationsart vermutlich mittelfristig von der cevi-db nicht mehr unterstützt.
* `cv.exe setup -t meinS3rviceToken` erlaubt feineres Management der Rechte, und falls dieses token in die falschen Hände gelangt, ist es einfach, es wieder zu deaktivieren.
  Um ein service-token einzurichten, siehe den Tab "API-Keys" auf der relevanten Ebene. Der link sieht ungefähr wie `https://db.cevi.ch/groups/115/service_tokens` aus, nur muss `115` ersetzt werden durch die korrekte Ebenen-ID.
  Das service-token benötigt nur die Rechte "Personen von Untergruppen Lesen".
* `cv.exe setup -t meinS3rviceToken -e levanzo@cevi.ch -p meinP4sswort`
  um beide Versionen in `config.yaml` zu hinterlegen.

Die Personen werden von den Datenbank-Endpoints geholt, die von dir in `config.yaml` spezifiziert werden. Dies **muss manuell gemacht werden**.
Folgende placeholder werden in den Endpoint links automatisch eingesetzt:

* `{api_token}`: Das user-token
* `{login_email}`: Die e-mail adresse
* `{service_token}`: Das service-token

Wenn du etwas davon nicht spezifizieren möchtest, setze es auf `""`.
Beispiel:

```yaml
#config.yaml
db_conf:
    # --- SECURE LOGIN ---
    # Das service-token muss manuell eingerichtet werden, z.B. unter db.cevi.ch/groups/115/service_tokens
    #    ( Ersetze die Zahl 115 durch die entsprechende Gruppe, der alle endpoint Gruppen untergeordnet sind )
    # Dieses service-token benötigt die Permissions "Personen von Untergruppen"
    # Falls das service_token gesetzt ist, kann in den ENDPOINTS service_token als placeholder verwendet werden.
    service_token: "asdfasdfasdfasdfa"
    # --- USERTOKEN LOGIN ---
    # Das user-token kann automatisch geholt werden. Das ist der einzige Vorteil davon. Dafür ist es weniger
    # sicher, weil es für den ganzen Nutzer das selbe ist, egal für welche Anwendung.
    # Das user-token wird hier auch api-token genannt.
    api_token: "asdfasdfasdfasdf"
    # die e-mail adresse zum einloggen in der db.cevi.ch
    login_email: "asdf@asdf.ch"
    # 
    # --- ENDPOINTS ---
    # Der link zu den Leuten in der datenbank. Relevant für dich als user sind nur die Zahlen für die gruppen,
    # sowie die filter_id
    # Ersetze sie durch die gruppen-id und filter-id, die du verwenden möchtest.
    # Bei login-type SECURE sind die links generell von der Form
    #    https://db.cevi.ch/groups/2423/people.json?token=[service_token]
    # nur mit geschweiften Klammern {} statt eckigen Klammern [].
    # Bei login-type USERTOKEN sind die links generell von der Form
    #    https://db.cevi.ch/groups/2423/people.json?user_email=[login_email]&user_token=[api_token]
    # nur mit geschweiften Klammern {} statt eckigen Klammern [].
    versand_endpoint_fmtstrs:
        - "https://db.cevi.ch/groups/2423/people.json?token={service_token}"
        - "https://db.cevi.ch/groups/116/people.json?filter_id=319&user_email={login_email}&user_token={api_token}"

```

### Modify

Wenn einige der generierten Couverts nicht so aussehen wie gewollt, ist es möglich die vom Programm generierten Dateien zu ändern:

* In `mapping.yaml` können alle `display_name:` modifiziert werden. Beim nächsten Programmdurchlauf wird dann der `original_name` durch den spezifizierten `display_name` ersetzt. Die Zahlen und der `original_name` sollten unverändert gelassen werden.
* In `inject_people.yaml` können Empfänger spezifiziert werden, die nicht in der Datenbank enthalten sind und trotzdem einen Umschlag erhalten sollen.

## Run

`cv.exe run` ist kurz für `cv.exe run -gnsm` und generiert eine `output_versand.pdf` Datei. Die erste Seite enthält Informationen, die restlichen Seiten sind C5-Couverts.

Wenn die Datenbank Personen enthält, deren Adressangaben unvollständig sind wird der Kommandozeilenoutput darüber informieren. Diese Personen werden trotzdem berücksichtigt beim generieren der Couverts, werden aber vermutlich Probleme beim per Post versenden verursachen.

### Troubleshooting

##### Clean

Mach ein Backup von deinen Dateien und lass `clean` laufen, dann mach nochmal `setup`.

```bash
cp config.yaml config.yaml.bak
cp inject_people.yaml inject_people.yaml.bak
cp mapping.yaml mapping.yaml.bak
cv.exe clean -ra
cv.exe setup -t servicetoken -e email@mail.ch -p passwort
# und dann die endpoints neu in config.yaml hinzufügen
```

##### Invalid Endpoint URL

Wenn das Programm abbricht und den folgenden Error anzeigt, hier ein Paar Tips:

```
thread 'main' panicked at 'WTF in main! Perhaps the credentials or the endpoint url are invalid?: Error("missing field `people`", line: 1, column: 81)'
```

* Ist in `config.yaml` ein service-token hinterlegt, oder eine e-mail und ein api-token?
* Ist in `config.yaml` mindestens ein Endpoint vorhanden?
* Ist `config.yaml` korrekt yaml-formatiert?
* Beinhaltet ein endpoint `user_token={service_token}` oder `service_token={service_token}` statt dem korrekten `token={service_token}`? 
* Beinhaltet ein endpoint mit user-token `token={api_token}` statt `user_token={api_token}`?



### Building

Not relevant to the average user.

If you want to clone this repository and build the executable yourself, you might need to install the dependencies for [winres](https://github.com/mxre/winres).

Note that this repository is **currently not licensed to you**. The source code belongs to me, but I do hereby grant you explicit permission to use the executables.