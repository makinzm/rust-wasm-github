# db

## Initial Settings

```bash
docker compose up -d
yarn prisma migrate reset
yarn prisma migrate dev

yarn prisma:seed
```


```bash
docker exec -it words_mysql mysql -u root -prootpassword wordsdb
```
