from flask import Flask
from flask import render_template
from flask import request

import requests

app = Flask(__name__)


@app.route('/xkcd', methods = ['POST'])
def hello_xkcd():
    req_form = request.form
    tmp = req_form.getlist('xkcd_id')

    id = int(tmp[0])

    url = f"https://xkcd.com/{id}/info.0.json"
    xkcd_response = requests.get(url)

    if xkcd_response.status_code == 404:
       return render_template('404.html')

    img_url = xkcd_response.json().get('img')

    if xkcd_response.status_code == 200:
            return f"""<img src={img_url}>"""


@app.route('/')
def hello_world():
    return render_template('form.html')


if __name__ == "__main__":
    app.run(host='0.0.0.0', port=8080, debug=True)
