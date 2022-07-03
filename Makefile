docker-create-volumes:
	docker volume create --name=diary_postgres

docker-rm-volumes:
	docker rm diary_postgres

docker-compose:
#	make docker-create-volumes
	docker-compose up