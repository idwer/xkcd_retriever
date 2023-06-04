all: run_actix

run_actix:
	cargo run

run_flask:
	FLASK_APP=xkcd_retriever.py FLASK_ENV=development flask run
