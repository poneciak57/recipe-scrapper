import morfeusz2
import json

with open("skladniki.json", encoding="utf-8") as json_file:
    jsoningr = json.load(json_file)

#SPRAWDZA CZY SKŁADNIK ZE STRONY PASUJE DO KTÓREGOŚ Z SKLADNIKI.JSON
def check_if_matches(z, text):
    for x in text:
        if x not in z:
            return False
    return True

#ZAMIENIA SKŁADNIK Z PRZEPISU DO JEGO PODSTAWOWEJ FORMY NP. 2 ZÓŁTKA JAJEK -> [JAJKA, JAJKO, JAJO, JAJA]
def revertWords(x):
    morf = morfeusz2.Morfeusz()

    interpreted_ingredient = []
    for interpretation in morf.analyse(x):
        interpreted_ingredient.append(interpretation[2][1].split(":")[0])
    for key, ingr in jsoningr.items():
        for sing_ingr in ingr:
            if check_if_matches(interpreted_ingredient, sing_ingr):
                if key == "":
                    return None
                return key
        else:
            continue

    return f"Nie wiem co to {interpreted_ingredient}"