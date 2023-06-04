all: run_flask

run_flask:
	FLASK_APP=xkcd_retriever.py FLASK_ENV=development flask run
