import morfeusz2
import requests
import json

morf = morfeusz2.Morfeusz()

with open("skladniki.json", encoding="utf-8") as json_file:
    jsoningr = json.load(json_file)

def send_for_veryfication(original, analyzed):
    url = "http://pb.what2bake.com/api/collections/product_analyzing/records"
    headers = {
        "Authorization": "Bearer"
    }
    payload = {
        "original_text": original,
        "analyzed_product": analyzed,
        "state": "UNCHECKED"
    }
    response = requests.post(url, json=payload, headers=headers)

#SPRAWDZA CZY SKŁADNIK ZE STRONY PASUJE DO KTÓREGOŚ Z SKLADNIKI.JSON
def check_if_matches(z, text):
    for x in text:
        if x not in z:
            return False
    return True

#ZAMIENIA SKŁADNIK Z PRZEPISU DO JEGO PODSTAWOWEJ FORMY NP. 2 ZÓŁTKA JAJEK -> [JAJKA, JAJKO, JAJO, JAJA]
def revertWords(x):
    interpreted_ingredient = []
    for interpretation in morf.analyse(x):
        interpreted_ingredient.append(interpretation[2][1].split(":")[0])
    for key, ingr in jsoningr.items():
        for sing_ingr in ingr:
            if check_if_matches(interpreted_ingredient, sing_ingr):
                if key == "":
#                     send_for_veryfication(x, interpreted_ingredient)
                    return None
#                 send_for_veryfication(x, key)
                return key
        else:
            continue
#     send_for_veryfication(x, interpreted_ingredient)
    return None